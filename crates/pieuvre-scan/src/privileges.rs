use crate::{Result, ScanError};
use std::ptr;
use windows_sys::Win32::Foundation::{
    CloseHandle, GetLastError, ERROR_NOT_ALL_ASSIGNED, HANDLE, LUID,
};
use windows_sys::Win32::Security::{
    AdjustTokenPrivileges, LookupPrivilegeValueW, LUID_AND_ATTRIBUTES, SE_PRIVILEGE_ENABLED,
    TOKEN_ADJUST_PRIVILEGES, TOKEN_PRIVILEGES, TOKEN_QUERY,
};
use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

/// Liste des privilèges critiques pour un scan et nettoyage SOTA.
const REQUIRED_PRIVILEGES: &[&str] = &[
    "SeDebugPrivilege",   // Inspection des processus et accès aux clés protégées
    "SeBackupPrivilege",  // Lecture de fichiers/ruches sans égard aux ACLs
    "SeRestorePrivilege", // Écriture/Suppression de fichiers/ruches sans égard aux ACLs
];

/// Acquiert les privilèges nécessaires pour le processus actuel.
/// Nécessite une exécution en tant qu'Administrateur.
pub fn enable_required_privileges() -> Result<()> {
    unsafe {
        let mut token_handle: HANDLE = ptr::null_mut();

        // 1. Ouvrir le jeton du processus actuel
        if OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
            &mut token_handle,
        ) == 0
        {
            return Err(ScanError::WindowsError(GetLastError()));
        }

        let _guard = HandleGuard(token_handle);

        for &priv_name in REQUIRED_PRIVILEGES {
            if let Err(e) = enable_single_privilege(token_handle, priv_name) {
                tracing::warn!("Échec de l'acquisition de {}: {:?}", priv_name, e);
            } else {
                tracing::debug!("Privilège acquis: {}", priv_name);
            }
        }

        Ok(())
    }
}

unsafe fn enable_single_privilege(token_handle: HANDLE, priv_name: &str) -> Result<()> {
    let mut luid: LUID = std::mem::zeroed();
    let priv_name_u16: Vec<u16> = priv_name.encode_utf16().chain(std::iter::once(0)).collect();

    if LookupPrivilegeValueW(ptr::null(), priv_name_u16.as_ptr(), &mut luid) == 0 {
        return Err(ScanError::WindowsError(GetLastError()));
    }

    let tp = TOKEN_PRIVILEGES {
        PrivilegeCount: 1,
        Privileges: [LUID_AND_ATTRIBUTES {
            Luid: luid,
            Attributes: SE_PRIVILEGE_ENABLED,
        }],
    };

    if AdjustTokenPrivileges(
        token_handle,
        0,
        &tp,
        std::mem::size_of::<TOKEN_PRIVILEGES>() as u32,
        ptr::null_mut(),
        ptr::null_mut(),
    ) == 0
    {
        return Err(ScanError::WindowsError(GetLastError()));
    }

    if GetLastError() == ERROR_NOT_ALL_ASSIGNED {
        return Err(ScanError::Privilege(format!(
            "Privilege {} not assigned to token",
            priv_name
        )));
    }

    Ok(())
}

struct HandleGuard(HANDLE);
impl Drop for HandleGuard {
    fn drop(&mut self) {
        unsafe {
            if !self.0.is_null() {
                CloseHandle(self.0);
            }
        }
    }
}
