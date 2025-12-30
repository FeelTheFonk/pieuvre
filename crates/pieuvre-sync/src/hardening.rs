//! Registry Hardening
//!
//! Verrouillage des clés de registre via ACLs pour empêcher les réinitialisations.
//! Utilise SDDL (Security Descriptor Definition Language) pour une précision maximale.

use pieuvre_common::{PieuvreError, Result};
use windows::core::{PCWSTR, PWSTR};
use windows::Win32::Foundation::{LocalFree, HANDLE, HLOCAL, LUID};
use windows::Win32::Security::Authorization::{
    ConvertStringSecurityDescriptorToSecurityDescriptorW, SetNamedSecurityInfoW, SE_REGISTRY_KEY,
    SE_SERVICE,
};
use windows::Win32::Security::{
    AdjustTokenPrivileges, LookupPrivilegeValueW, DACL_SECURITY_INFORMATION, LUID_AND_ATTRIBUTES,
    PROTECTED_DACL_SECURITY_INFORMATION, PSECURITY_DESCRIPTOR, PSID, SE_PRIVILEGE_ENABLED,
    TOKEN_ADJUST_PRIVILEGES, TOKEN_PRIVILEGES, TOKEN_QUERY,
};
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

const SDDL_REVISION_1: u32 = 1;

/// Verrouille une clé de registre en lecture seule
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

/// Verrouille un service
/// Empêche l'arrêt et la modification par tout le monde sauf SYSTEM
pub fn lock_service(service_name: &str) -> Result<()> {
    // Vérifier si le service existe avant de tenter quoi que ce soit
    if !service_exists(service_name) {
        return Err(PieuvreError::ServiceNotFound(service_name.to_string()));
    }
    apply_sddl_service(service_name, "D:P(A;;LCRP;;;WD)(A;;KA;;;SY)")
}

fn service_exists(name: &str) -> bool {
    use windows::Win32::System::Services::{
        CloseServiceHandle, OpenSCManagerW, OpenServiceW, SC_MANAGER_CONNECT, SERVICE_QUERY_CONFIG,
    };
    unsafe {
        let scm = match OpenSCManagerW(PCWSTR::null(), PCWSTR::null(), SC_MANAGER_CONNECT) {
            Ok(s) => s,
            Err(_) => return false,
        };
        let name_wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
        let service = OpenServiceW(scm, PCWSTR(name_wide.as_ptr()), SERVICE_QUERY_CONFIG);
        let exists = service.is_ok();
        if let Ok(s) = service {
            let _ = CloseServiceHandle(s);
        }
        let _ = CloseServiceHandle(scm);
        exists
    }
}

fn apply_sddl_service(service_name: &str, sddl: &str) -> Result<()> {
    unsafe {
        let _ = enable_privilege("SeTakeOwnershipPrivilege");
        let _ = enable_privilege("SeRestorePrivilege");

        let sddl_wide: Vec<u16> = sddl.encode_utf16().chain(std::iter::once(0)).collect();

        let mut sd: PSECURITY_DESCRIPTOR = PSECURITY_DESCRIPTOR::default();
        ConvertStringSecurityDescriptorToSecurityDescriptorW(
            PCWSTR(sddl_wide.as_ptr()),
            SDDL_REVISION_1,
            &mut sd,
            None,
        )
        .map_err(|e| PieuvreError::Internal(format!("SDDL conversion failed: {}", e)))?;

        let mut dacl = std::ptr::null_mut();
        let mut dacl_present = 0i32;
        let mut dacl_defaulted = 0i32;

        let _ = windows::Win32::Security::GetSecurityDescriptorDacl(
            sd,
            &mut dacl_present as *mut i32 as *mut _,
            &mut dacl,
            &mut dacl_defaulted as *mut i32 as *mut _,
        );

        // Tentative via SetNamedSecurityInfoW avec préfixe SERVICE\\ pour plus de fiabilité
        let full_service_path = format!("SERVICE\\{}", service_name);
        let full_service_path_wide: Vec<u16> = full_service_path
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();

        let result = SetNamedSecurityInfoW(
            PCWSTR(full_service_path_wide.as_ptr()),
            SE_SERVICE,
            DACL_SECURITY_INFORMATION | PROTECTED_DACL_SECURITY_INFORMATION,
            None,
            None,
            Some(dacl),
            None,
        )
        .ok();

        let _ = LocalFree(Some(HLOCAL(sd.0 as *mut _)));
        if let Err(e) = result {
            return Err(PieuvreError::Internal(format!(
                "Failed to lock service {}: {:?}",
                service_name, e
            )));
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
        )
        .map_err(|e| {
            PieuvreError::Internal(format!("SDDL conversion failed for {}: {}", sddl, e))
        })?;

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
            return Err(PieuvreError::Internal(
                "Failed to get DACL from SD".to_string(),
            ));
        }

        let mut result = SetNamedSecurityInfoW(
            PCWSTR(path_wide.as_ptr()),
            SE_REGISTRY_KEY,
            DACL_SECURITY_INFORMATION | PROTECTED_DACL_SECURITY_INFORMATION,
            None,
            None,
            Some(dacl),
            None,
        )
        .ok();

        // Si Access Denied (5), tenter de prendre possession (Ownership) d'abord
        if let Err(e) = &result {
            if e.code().0 as u32 == 5 {
                tracing::warn!(
                    "Access Denied for {}, attempting to take ownership...",
                    full_path
                );
                let _ = take_ownership(&full_path);
                // Réessayer après ownership
                result = SetNamedSecurityInfoW(
                    PCWSTR(path_wide.as_ptr()),
                    SE_REGISTRY_KEY,
                    DACL_SECURITY_INFORMATION | PROTECTED_DACL_SECURITY_INFORMATION,
                    None,
                    None,
                    Some(dacl),
                    None,
                )
                .ok();
            }
        }

        let _ = LocalFree(Some(HLOCAL(sd.0 as *mut _)));

        if let Err(e) = result {
            return Err(PieuvreError::Internal(format!(
                "SetNamedSecurityInfo failed for {}: {:?}",
                full_path, e
            )));
        }

        tracing::info!(path = %full_path, sddl = %sddl, "ACL appliquée avec succès");
        Ok(())
    }
}

fn take_ownership(path: &str) -> Result<()> {
    use windows::Win32::Security::{LookupAccountNameW, OWNER_SECURITY_INFORMATION, SID_NAME_USE};
    unsafe {
        let path_wide: Vec<u16> = path.encode_utf16().chain(std::iter::once(0)).collect();

        // Obtenir le SID de SYSTEM
        let mut sid_size = 0u32;
        let mut domain_size = 0u32;
        let mut sid_name_use = SID_NAME_USE::default();
        let system_name: Vec<u16> = "SYSTEM".encode_utf16().chain(std::iter::once(0)).collect();

        let _ = LookupAccountNameW(
            None,
            PCWSTR(system_name.as_ptr()),
            None,
            &mut sid_size,
            None,
            &mut domain_size,
            &mut sid_name_use,
        );

        let mut sid = vec![0u8; sid_size as usize];
        let mut domain = vec![0u16; domain_size as usize];

        if LookupAccountNameW(
            None,
            PCWSTR(system_name.as_ptr()),
            Some(PSID(sid.as_mut_ptr() as _)),
            &mut sid_size,
            Some(PWSTR(domain.as_mut_ptr())),
            &mut domain_size,
            &mut sid_name_use,
        )
        .is_err()
        {
            return Err(PieuvreError::Internal(
                "LookupAccountName failed".to_string(),
            ));
        }

        if let Err(e) = SetNamedSecurityInfoW(
            PCWSTR(path_wide.as_ptr()),
            SE_REGISTRY_KEY,
            OWNER_SECURITY_INFORMATION,
            Some(PSID(sid.as_ptr() as _)),
            None,
            None,
            None,
        )
        .ok()
        {
            return Err(PieuvreError::Internal(format!(
                "SetNamedSecurityInfo (Ownership) failed: {}",
                e
            )));
        }

        Ok(())
    }
}

fn enable_privilege(privilege_name: &str) -> Result<()> {
    unsafe {
        let mut token: HANDLE = HANDLE::default();
        OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
            &mut token,
        )
        .map_err(|e| PieuvreError::Internal(format!("Failed to open process token: {}", e)))?;

        let mut luid = LUID::default();
        let priv_name_wide: Vec<u16> = privilege_name
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();

        if LookupPrivilegeValueW(None, PCWSTR(priv_name_wide.as_ptr()), &mut luid).is_err() {
            let _ = windows::Win32::Foundation::CloseHandle(token);
            return Err(PieuvreError::Internal(format!(
                "LookupPrivilegeValue failed for {}",
                privilege_name
            )));
        }

        let tp = TOKEN_PRIVILEGES {
            PrivilegeCount: 1,
            Privileges: [LUID_AND_ATTRIBUTES {
                Luid: luid,
                Attributes: SE_PRIVILEGE_ENABLED,
            }],
        };

        let result = AdjustTokenPrivileges(token, false, Some(&tp), 0, None, None);

        let _ = windows::Win32::Foundation::CloseHandle(token);

        if result.is_err() {
            return Err(PieuvreError::Internal(format!(
                "AdjustTokenPrivileges failed for {}",
                privilege_name
            )));
        }

        Ok(())
    }
}

/// Active la protection PPL (Protected Process Light) pour le processus actuel
/// Nécessite que le binaire soit signé avec un certificat ELAM ou spécifique.
pub fn enable_ppl_protection() -> Result<()> {
    unsafe {
        use windows::Win32::System::Threading::{
            ProcessProtectionLevelInfo, SetProcessInformation,
            PROCESS_PROTECTION_LEVEL_INFORMATION, PROTECTION_LEVEL_NONE,
        };

        let mut protection = PROCESS_PROTECTION_LEVEL_INFORMATION {
            ProtectionLevel: PROTECTION_LEVEL_NONE, // Base level
        };

        SetProcessInformation(
            GetCurrentProcess(),
            ProcessProtectionLevelInfo,
            &mut protection as *mut _ as *mut _,
            std::mem::size_of::<PROCESS_PROTECTION_LEVEL_INFORMATION>() as u32,
        )
        .map_err(|e| PieuvreError::Internal(format!("Failed to set PPL protection: {}", e)))?;

        tracing::info!("Protection PPL activée");
        Ok(())
    }
}

// ============================================
// REGISTRY PATH CONSTANTS (SOTA Source of Truth)
// ============================================

// --- SECURITY ---
pub const HVCI_KEY: &str =
    r"SYSTEM\CurrentControlSet\Control\DeviceGuard\Scenarios\HypervisorEnforcedCodeIntegrity";
pub const DEVICE_GUARD_KEY: &str = r"SYSTEM\CurrentControlSet\Control\DeviceGuard";
pub const MEMORY_MANAGEMENT_KEY: &str =
    r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management";
pub const UAC_POLICIES_KEY: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System";

// --- PRIVACY & TELEMETRY ---
pub const DATA_COLLECTION_KEY: &str = r"SOFTWARE\Policies\Microsoft\Windows\DataCollection";
pub const DATA_COLLECTION_POLICIES_KEY: &str =
    r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection";
pub const ADVERTISING_INFO_KEY: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\AdvertisingInfo";
pub const ADVERTISING_INFO_POLICIES_KEY: &str =
    r"SOFTWARE\Policies\Microsoft\Windows\AdvertisingInfo";
pub const SQM_CLIENT_KEY: &str = r"SOFTWARE\Policies\Microsoft\SQMClient\Windows";
pub const SQM_CLIENT_HKLM_KEY: &str = r"SOFTWARE\Microsoft\SQMClient\Windows";
pub const APP_PRIVACY_KEY: &str = r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy";
pub const CONSENT_STORE_KEY: &str =
    r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore";

// --- AI & COGNITIVE ---
pub const WINDOWS_AI_KEY: &str = r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI";
pub const WINDOWS_COPILOT_KEY: &str = r"SOFTWARE\Policies\Microsoft\Windows\WindowsCopilot";
pub const AI_DATA_ANALYSIS_KEY: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\AI\DataAnalysis";

// --- SHELL & EXPLORER ---
pub const DSH_KEY: &str = r"SOFTWARE\Policies\Microsoft\Dsh";
pub const EXPLORER_ADVANCED_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Advanced";
pub const EXPLORER_POLICIES_KEY: &str = r"SOFTWARE\Policies\Microsoft\Windows\Explorer";
pub const WINDOWS_SEARCH_KEY: &str = r"SOFTWARE\Policies\Microsoft\Windows\Windows Search";
pub const EXPLORER_SHELL_DELAY_KEY: &str =
    r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\ShellServiceObjectDelayLoad";
pub const WINLOGON_KEY: &str = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon";
pub const APPINIT_DLLS_KEY: &str =
    r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Windows\AppInit_DLLs";
pub const IFEO_KEY: &str =
    r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options";

// --- NETWORK & PERFORMANCE ---
pub const DELIVERY_OPTIMIZATION_KEY: &str =
    r"SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization";
pub const WIFI_MANAGER_KEY: &str = r"SOFTWARE\Microsoft\WcmSvc\wifinetworkmanager\config";
pub const PRIORITY_CONTROL_KEY: &str = r"SYSTEM\CurrentControlSet\Control\PriorityControl";
pub const SESSION_MANAGER_KERNEL_KEY: &str =
    r"SYSTEM\CurrentControlSet\Control\Session Manager\Kernel";
pub const DNS_CACHE_PARAMS_KEY: &str = r"SYSTEM\CurrentControlSet\Services\Dnscache\Parameters";
pub const MULTIMEDIA_PROFILE_KEY: &str =
    r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile";

// --- SERVICES ---
pub const SERVICE_DIAGTRACK: &str = "DiagTrack";
pub const SERVICE_SYSMAIN: &str = "SysMain";
pub const SERVICE_WSEARCH: &str = "WSearch";
pub const SERVICE_WERSVC: &str = "WerSvc";
pub const SERVICE_NV_TELEMETRY: &str = "NvTelemetryContainer";
pub const SERVICE_INTEL_HECI: &str = "Intel(R) Content Protection HECI Service";
pub const SERVICE_WAP_PUSH: &str = "dmwappushservice";
pub const SERVICE_UPDATE: &str = "wuauserv";
pub const SERVICE_USOSVC: &str = "UsoSvc";
pub const SERVICE_DOSVC: &str = "DoSvc";

/// Clés critiques à verrouiller
pub const CRITICAL_KEYS: &[&str] = &[
    PRIORITY_CONTROL_KEY,
    MEMORY_MANAGEMENT_KEY,
    MULTIMEDIA_PROFILE_KEY,
    SESSION_MANAGER_KERNEL_KEY,
    r"SYSTEM\CurrentControlSet\Services\DiagTrack",
    r"SYSTEM\CurrentControlSet\Services\SysMain",
    IFEO_KEY,
    APPINIT_DLLS_KEY,
    WINLOGON_KEY,
    EXPLORER_SHELL_DELAY_KEY,
    WINDOWS_AI_KEY,
    AI_DATA_ANALYSIS_KEY,
    WINDOWS_COPILOT_KEY,
    DNS_CACHE_PARAMS_KEY,
    DATA_COLLECTION_KEY,
    DSH_KEY,
    WINDOWS_SEARCH_KEY,
    DELIVERY_OPTIMIZATION_KEY,
];

/// Services critiques à verrouiller
pub const CRITICAL_SERVICES: &[&str] = &[
    SERVICE_DIAGTRACK,
    SERVICE_SYSMAIN,
    SERVICE_WSEARCH,
    SERVICE_WERSVC,
    SERVICE_NV_TELEMETRY,
    SERVICE_INTEL_HECI,
];
