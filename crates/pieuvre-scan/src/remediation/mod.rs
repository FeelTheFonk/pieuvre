use crate::{Result, ScanError};
use std::path::Path;
use windows_sys::Win32::Storage::FileSystem::{MoveFileExW, MOVEFILE_DELAY_UNTIL_REBOOT};

pub struct Remediator {
    quarantine_path: String,
}

impl Remediator {
    pub fn new(quarantine_path: &str) -> Self {
        Self {
            quarantine_path: quarantine_path.to_string(),
        }
    }

    /// Supprime un fichier immédiatement, avec fallback sur le redémarrage si verrouillé.
    pub fn delete_file(&self, file_path: &str) -> Result<()> {
        let path = Path::new(file_path);
        if !path.exists() {
            return Ok(());
        }

        if let Err(e) = std::fs::remove_file(path) {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                tracing::warn!(
                    "Accès refusé pour {}, planification au redémarrage...",
                    file_path
                );
                self.delete_on_reboot(file_path)?;
            } else {
                return Err(ScanError::Io(e));
            }
        }

        Ok(())
    }

    /// Supprime un fichier au prochain redémarrage (SOTA: MOVEFILE_DELAY_UNTIL_REBOOT)
    pub fn delete_on_reboot(&self, file_path: &str) -> Result<()> {
        let path_u16: Vec<u16> = file_path.encode_utf16().chain(std::iter::once(0)).collect();

        unsafe {
            if MoveFileExW(
                path_u16.as_ptr(),
                std::ptr::null(),
                MOVEFILE_DELAY_UNTIL_REBOOT,
            ) == 0
            {
                return Err(ScanError::WindowsError(
                    windows_sys::Win32::Foundation::GetLastError(),
                ));
            }
        }

        Ok(())
    }

    /// Met un fichier en quarantaine (XOR encryption simple)
    pub fn quarantine(&self, file_path: &str) -> Result<()> {
        let path = Path::new(file_path);
        if !path.exists() {
            return Err(ScanError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File not found",
            )));
        }

        let data = std::fs::read(path)?;
        let encrypted_data: Vec<u8> = data.into_iter().map(|b| b ^ 0xFF).collect();

        let filename = path
            .file_name()
            .ok_or_else(|| ScanError::Other("Invalid filename".to_string()))?;
        let dest_path = Path::new(&self.quarantine_path).join(filename);

        std::fs::create_dir_all(&self.quarantine_path)?;
        std::fs::write(dest_path, encrypted_data)?;

        Ok(())
    }
}
