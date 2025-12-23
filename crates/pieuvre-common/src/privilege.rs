use crate::error::{PieuvreError, Result};
use windows::Win32::Foundation::{HANDLE, LUID};
use windows::Win32::Security::{
    AdjustTokenPrivileges, LookupPrivilegeValueW, SE_PRIVILEGE_ENABLED, TOKEN_ADJUST_PRIVILEGES,
    TOKEN_PRIVILEGES, TOKEN_QUERY,
};
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

/// Gère l'acquisition des privilèges système nécessaires pour les opérations bas niveau.
pub struct PrivilegeManager;

impl PrivilegeManager {
    /// Active les privilèges nécessaires pour le backup et la restauration du registre/fichiers.
    pub fn enable_system_privileges() -> Result<()> {
        Self::enable_privilege("SeBackupPrivilege")?;
        Self::enable_privilege("SeRestorePrivilege")?;
        Ok(())
    }

    fn enable_privilege(privilege_name: &str) -> Result<()> {
        unsafe {
            let mut token: HANDLE = HANDLE::default();
            OpenProcessToken(
                GetCurrentProcess(),
                TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
                &mut token,
            )
            .map_err(|e| PieuvreError::System(format!("Failed to open process token: {}", e)))?;

            let mut luid = LUID::default();
            let name: Vec<u16> = privilege_name
                .encode_utf16()
                .chain(std::iter::once(0))
                .collect();
            LookupPrivilegeValueW(None, windows::core::PCWSTR(name.as_ptr()), &mut luid).map_err(
                |e| PieuvreError::System(format!("Failed to lookup privilege value: {}", e)),
            )?;

            let tp = TOKEN_PRIVILEGES {
                PrivilegeCount: 1,
                Privileges: [windows::Win32::Security::LUID_AND_ATTRIBUTES {
                    Luid: luid,
                    Attributes: SE_PRIVILEGE_ENABLED,
                }; 1],
            };

            AdjustTokenPrivileges(token, false, Some(&tp), 0, None, None).map_err(|e| {
                PieuvreError::System(format!("Failed to adjust token privileges: {}", e))
            })?;

            Ok(())
        }
    }
}
