//! Registry Hardening SOTA 2026
//!
//! Verrouillage des clés de registre via ACLs pour empêcher les réinitialisations.
//! Utilise SDDL (Security Descriptor Definition Language) pour une précision maximale.

use pieuvre_common::{PieuvreError, Result};
use windows::Win32::Foundation::{HANDLE, LUID, LocalFree, HLOCAL};
use windows::Win32::Security::{
    DACL_SECURITY_INFORMATION, PROTECTED_DACL_SECURITY_INFORMATION,
    PSECURITY_DESCRIPTOR,
    TOKEN_ADJUST_PRIVILEGES, TOKEN_QUERY, LUID_AND_ATTRIBUTES,
    TOKEN_PRIVILEGES, SE_PRIVILEGE_ENABLED, LookupPrivilegeValueW,
    AdjustTokenPrivileges,
};
use windows::Win32::Security::Authorization::{
    SetNamedSecurityInfoW, SE_REGISTRY_KEY, SE_SERVICE,
    ConvertStringSecurityDescriptorToSecurityDescriptorW,
};
use windows::core::PCWSTR;
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

const SDDL_REVISION_1: u32 = 1;

/// Verrouille une clé de registre en lecture seule (SOTA)
/// SDDL: D:P(A;;KR;;;WD)(A;;KA;;;SY) -> Allow Read (KR) to Everyone (WD), Full Control (KA) to SYSTEM (SY)
pub fn lock_registry_key(key_path: &str) -> Result<()> {
    apply_sddl(key_path, "D:P(A;;KR;;;WD)(A;;KA;;;SY)")
}

/// Déverrouille une clé (Contrôle total pour tout le monde - Temporaire pour modif)
/// SDDL: D:P(A;;KA;;;WD)(A;;KA;;;SY) -> Allow All (KA) to Everyone (WD), Full Control (KA) to SYSTEM (SY)
/// Déverrouille une clé (Contrôle total pour tout le monde - Temporaire pour modif)
/// SDDL: D:P(A;;KA;;;WD)(A;;KA;;;SY) -> Allow All (KA) to Everyone (WD), Full Control (KA) to SYSTEM (SY)
pub fn unlock_registry_key(key_path: &str) -> Result<()> {
    apply_sddl(key_path, "D:P(A;;KA;;;WD)(A;;KA;;;SY)")
}

/// Verrouille un service (SOTA)
/// Empêche l'arrêt et la modification par tout le monde sauf SYSTEM
pub fn lock_service(service_name: &str) -> Result<()> {
    apply_sddl_service(service_name, "D:P(A;;LCRP;;;WD)(A;;KA;;;SY)")
}

fn apply_sddl_service(service_name: &str, sddl: &str) -> Result<()> {
    unsafe {
        let _ = enable_privilege("SeTakeOwnershipPrivilege");
        let _ = enable_privilege("SeRestorePrivilege");

        let service_name_wide: Vec<u16> = service_name.encode_utf16().chain(std::iter::once(0)).collect();
        let sddl_wide: Vec<u16> = sddl.encode_utf16().chain(std::iter::once(0)).collect();

        let mut sd: PSECURITY_DESCRIPTOR = PSECURITY_DESCRIPTOR::default();
        ConvertStringSecurityDescriptorToSecurityDescriptorW(
            PCWSTR(sddl_wide.as_ptr()),
            SDDL_REVISION_1,
            &mut sd,
            None,
        ).map_err(|e| PieuvreError::Internal(format!("SDDL conversion failed: {}", e)))?;

        let mut dacl = std::ptr::null_mut();
        let mut dacl_present = 0i32;
        let mut dacl_defaulted = 0i32;

        let _ = windows::Win32::Security::GetSecurityDescriptorDacl(
            sd,
            &mut dacl_present as *mut i32 as *mut _,
            &mut dacl,
            &mut dacl_defaulted as *mut i32 as *mut _,
        );

        let result = SetNamedSecurityInfoW(
            PCWSTR(service_name_wide.as_ptr()),
            SE_SERVICE,
            DACL_SECURITY_INFORMATION | PROTECTED_DACL_SECURITY_INFORMATION,
            None,
            None,
            Some(dacl),
            None,
        );

        let _ = LocalFree(Some(HLOCAL(sd.0 as *mut _)));
        if result.is_err() {
            return Err(PieuvreError::Internal(format!("Failed to lock service {}: {:?}", service_name, result)));
        }

        tracing::info!(service = %service_name, "Service verrouillé avec succès");
        Ok(())
    }
}

fn apply_sddl(key_path: &str, sddl: &str) -> Result<()> {
    unsafe {
        let _ = enable_privilege("SeTakeOwnershipPrivilege");
        let _ = enable_privilege("SeRestorePrivilege");

        let full_path = format!("MACHINE\\{}", key_path);
        let path_wide: Vec<u16> = full_path.encode_utf16().chain(std::iter::once(0)).collect();
        let sddl_wide: Vec<u16> = sddl.encode_utf16().chain(std::iter::once(0)).collect();

        let mut sd: PSECURITY_DESCRIPTOR = PSECURITY_DESCRIPTOR::default();
        
        ConvertStringSecurityDescriptorToSecurityDescriptorW(
            PCWSTR(sddl_wide.as_ptr()),
            SDDL_REVISION_1,
            &mut sd,
            None,
        ).map_err(|e| PieuvreError::Internal(format!("SDDL conversion failed for {}: {}", sddl, e)))?;

        // Extraire le DACL du Security Descriptor
        // Note: Dans windows-rs 0.62, BOOL est souvent projeté comme bool dans les arguments de sortie
        // ou via un type spécifique. Si BOOL n'est pas trouvé, on utilise i32 pour le pointeur brut.
        let mut dacl_present = 0i32; 
        let mut dacl = std::ptr::null_mut();
        let mut dacl_defaulted = 0i32;

        let res_dacl = windows::Win32::Security::GetSecurityDescriptorDacl(
            sd,
            &mut dacl_present as *mut i32 as *mut _,
            &mut dacl,
            &mut dacl_defaulted as *mut i32 as *mut _,
        );

        if res_dacl.is_err() {
            let _ = LocalFree(Some(HLOCAL(sd.0 as *mut _)));
            return Err(PieuvreError::Internal("Failed to get DACL from SD".to_string()));
        }

        let result = SetNamedSecurityInfoW(
            PCWSTR(path_wide.as_ptr()),
            SE_REGISTRY_KEY,
            DACL_SECURITY_INFORMATION | PROTECTED_DACL_SECURITY_INFORMATION,
            None,
            None,
            Some(dacl),
            None,
        );

        let _ = LocalFree(Some(HLOCAL(sd.0 as *mut _)));

        if result.is_err() {
            return Err(PieuvreError::Internal(format!("SetNamedSecurityInfo failed for {}: {:?}", full_path, result)));
        }

        tracing::info!(path = %full_path, sddl = %sddl, "ACL appliquée avec succès");
        Ok(())
    }
}

fn enable_privilege(privilege_name: &str) -> Result<()> {
    unsafe {
        let mut token: HANDLE = HANDLE::default();
        OpenProcessToken(GetCurrentProcess(), TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, &mut token)
            .map_err(|e| PieuvreError::Internal(format!("Failed to open process token: {}", e)))?;

        let mut luid = LUID::default();
        let priv_name_wide: Vec<u16> = privilege_name.encode_utf16().chain(std::iter::once(0)).collect();
        
        if LookupPrivilegeValueW(None, PCWSTR(priv_name_wide.as_ptr()), &mut luid).is_err() {
            let _ = windows::Win32::Foundation::CloseHandle(token);
            return Err(PieuvreError::Internal(format!("LookupPrivilegeValue failed for {}", privilege_name)));
        }

        let tp = TOKEN_PRIVILEGES {
            PrivilegeCount: 1,
            Privileges: [LUID_AND_ATTRIBUTES {
                Luid: luid,
                Attributes: SE_PRIVILEGE_ENABLED,
            }],
        };

        let result = AdjustTokenPrivileges(
            token,
            false,
            Some(&tp),
            0,
            None,
            None,
        );

        let _ = windows::Win32::Foundation::CloseHandle(token);

        if result.is_err() {
            return Err(PieuvreError::Internal(format!("AdjustTokenPrivileges failed for {}", privilege_name)));
        }

        Ok(())
    }
}

/// Clés critiques à verrouiller (SOTA 2026)
pub const CRITICAL_KEYS: &[&str] = &[
    r"SYSTEM\CurrentControlSet\Control\PriorityControl",
    r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
    r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile",
    r"SYSTEM\CurrentControlSet\Control\Session Manager\Kernel",
    r"SYSTEM\CurrentControlSet\Services\DiagTrack",
    r"SYSTEM\CurrentControlSet\Services\SysMain",
    // Vecteurs de persistance & IFEO
    r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options",
    r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Windows\AppInit_DLLs",
    r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon",
    r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\ShellServiceObjectDelayLoad",
];

/// Services critiques à verrouiller (SOTA 2026)
pub const CRITICAL_SERVICES: &[&str] = &[
    "DiagTrack",
    "SysMain",
    "WSearch",
    "WerSvc",
    "NvTelemetryContainer", // NVIDIA Telemetry
    "Intel(R) Content Protection HECI Service", // Intel
];
