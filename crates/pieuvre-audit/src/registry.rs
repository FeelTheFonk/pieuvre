use pieuvre_common::{PieuvreError, Result};
use std::path::Path;
use windows::Win32::System::Registry::{
    RegCloseKey, RegOpenKeyExW, RegQueryValueExW, RegSaveKeyExW, HKEY, HKEY_LOCAL_MACHINE,
    KEY_READ, REG_DWORD, REG_SAVE_FORMAT, REG_VALUE_TYPE,
};

pub struct RegistryCleaner;

impl RegistryCleaner {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RegistryCleaner {
    fn default() -> Self {
        Self::new()
    }
}

impl RegistryCleaner {
    pub fn create_snapshot(hkey: HKEY, subkey: &str, output_path: &Path) -> Result<()> {
        unsafe {
            let mut hkey_result = HKEY::default();
            let subkey_u16: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();

            RegOpenKeyExW(
                hkey,
                windows::core::PCWSTR(subkey_u16.as_ptr()),
                None,
                KEY_READ,
                &mut hkey_result,
            )
            .ok()
            .map_err(|e| PieuvreError::System(format!("Failed to open key for snapshot: {}", e)))?;

            let path_u16: Vec<u16> = output_path
                .to_string_lossy()
                .encode_utf16()
                .chain(std::iter::once(0))
                .collect();

            let res_save = RegSaveKeyExW(
                hkey_result,
                windows::core::PCWSTR(path_u16.as_ptr()),
                None,
                REG_SAVE_FORMAT(1), // REG_LATEST_FORMAT
            );

            let _ = RegCloseKey(hkey_result);

            res_save
                .ok()
                .map_err(|e| PieuvreError::System(format!("Failed to save registry key: {}", e)))?;

            Ok(())
        }
    }
}

/// Vérifie si une clé de registre existe.
pub fn key_exists(hkey: HKEY, subkey: &str) -> bool {
    unsafe {
        let mut hkey_result = HKEY::default();
        let subkey_u16: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
        let res = RegOpenKeyExW(
            hkey,
            windows::core::PCWSTR(subkey_u16.as_ptr()),
            None,
            KEY_READ,
            &mut hkey_result,
        );
        if res.is_ok() {
            let _ = RegCloseKey(hkey_result);
            true
        } else {
            false
        }
    }
}

/// Lit une valeur DWORD (u32) du registre.
pub fn read_dword_value(hkey: HKEY, subkey: &str, value_name: &str) -> Result<u32> {
    unsafe {
        let mut hkey_result = HKEY::default();
        let subkey_u16: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
        RegOpenKeyExW(
            hkey,
            windows::core::PCWSTR(subkey_u16.as_ptr()),
            None,
            KEY_READ,
            &mut hkey_result,
        )
        .ok()
        .map_err(|e| PieuvreError::System(format!("Failed to open key: {}", e)))?;

        let mut data = 0u32;
        let mut data_size = std::mem::size_of::<u32>() as u32;
        let mut value_type = REG_VALUE_TYPE::default();
        let value_name_u16: Vec<u16> = value_name
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();

        let res = RegQueryValueExW(
            hkey_result,
            windows::core::PCWSTR(value_name_u16.as_ptr()),
            None,
            Some(&mut value_type),
            Some(&mut data as *mut u32 as *mut u8),
            Some(&mut data_size),
        );

        let _ = RegCloseKey(hkey_result);

        res.ok().map_err(|e| {
            PieuvreError::System(format!("Failed to query value {}: {}", value_name, e))
        })?;

        if value_type != REG_DWORD {
            return Err(PieuvreError::Parse(format!(
                "Value {} is not a DWORD",
                value_name
            )));
        }

        Ok(data)
    }
}

/// Version simplifiée pour HKLM.
pub fn read_hklm_dword(subkey: &str, value_name: &str) -> Result<u32> {
    read_dword_value(HKEY_LOCAL_MACHINE, subkey, value_name)
}
