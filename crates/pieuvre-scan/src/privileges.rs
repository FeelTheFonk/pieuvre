use crate::{Result, ScanError};
use std::ptr;
use windows_sys::Win32::Foundation::{
    CloseHandle, GetLastError, ERROR_NOT_ALL_ASSIGNED, HANDLE, LUID,
};
use windows_sys::Win32::Security::{
    AdjustTokenPrivileges, LookupPrivilegeValueW, SE_PRIVILEGE_ENABLED, TOKEN_ADJUST_PRIVILEGES,
    TOKEN_PRIVILEGES, TOKEN_QUERY,
};
use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

/// Acquiert le privilège SeDebugPrivilege pour le processus actuel.
/// Nécessaire pour inspecter les processus système et accéder aux clés protégées.
pub fn enable_debug_privilege() -> Result<()> {
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

        // S'assurer que le handle est fermé à la fin
        let _guard = HandleGuard(token_handle);

        // 2. Chercher le LUID pour SeDebugPrivilege
        let privilege_name: Vec<u16> = "SeDebugPrivilege\0".encode_utf16().collect();
        let mut luid: LUID = std::mem::zeroed();

        if LookupPrivilegeValueW(ptr::null(), privilege_name.as_ptr(), &mut luid) == 0 {
            return Err(ScanError::WindowsError(GetLastError()));
        }

        // 3. Préparer la structure TOKEN_PRIVILEGES
        let tp = TOKEN_PRIVILEGES {
            PrivilegeCount: 1,
            Privileges: [windows_sys::Win32::Security::LUID_AND_ATTRIBUTES {
                Luid: luid,
                Attributes: SE_PRIVILEGE_ENABLED,
            }],
        };

        // 4. Ajuster les privilèges du jeton
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

        // ATTENTION : AdjustTokenPrivileges peut réussir tout en échouant partiellement
        let last_error = GetLastError();
        if last_error == ERROR_NOT_ALL_ASSIGNED {
            return Err(ScanError::Privilege(
                "SeDebugPrivilege not assigned to token".to_string(),
            ));
        }

        Ok(())
    }
}

struct HandleGuard(HANDLE);
impl Drop for HandleGuard {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.0);
        }
    }
}
