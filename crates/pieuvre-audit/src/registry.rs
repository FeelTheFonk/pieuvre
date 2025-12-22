//! Inspection du registre SOTA
//!
//! Parsing des ruches, détection télémétrie, et audit sécurité complet.

use pieuvre_common::{PieuvreError, Result, TelemetryStatus};
use windows::Win32::System::Registry::{
    RegOpenKeyExW, RegQueryValueExW, RegCloseKey, RegEnumValueW,
    HKEY_LOCAL_MACHINE, HKEY_CURRENT_USER, HKEY, KEY_READ, REG_DWORD, REG_SZ,
};
use windows::core::PCWSTR;
use serde::{Deserialize, Serialize};

// ============================================================================
// TÉLÉMÉTRIE - Clés complètes (30+)
// ============================================================================

/// Clés télémétrie/privacy complètes
#[allow(dead_code)]
const TELEMETRY_KEYS_FULL: &[(&str, &str, &str)] = &[
    // Data Collection
    (r"SOFTWARE\Policies\Microsoft\Windows\DataCollection", "AllowTelemetry", "Telemetry Level"),
    (r"SOFTWARE\Policies\Microsoft\Windows\DataCollection", "AllowDeviceNameInTelemetry", "Device Name"),
    (r"SOFTWARE\Policies\Microsoft\Windows\DataCollection", "DisableOneSettingsDownloads", "OneSettings"),
    (r"SOFTWARE\Policies\Microsoft\Windows\DataCollection", "DoNotShowFeedbackNotifications", "Feedback UI"),
    
    // Advertising & Tracking
    (r"SOFTWARE\Microsoft\Windows\CurrentVersion\AdvertisingInfo", "Enabled", "Advertising ID"),
    (r"SOFTWARE\Policies\Microsoft\Windows\AdvertisingInfo", "DisabledByGroupPolicy", "Ad ID GPO"),
    
    // Cortana & Search
    (r"SOFTWARE\Policies\Microsoft\Windows\Windows Search", "AllowCortana", "Cortana"),
    (r"SOFTWARE\Policies\Microsoft\Windows\Windows Search", "DisableWebSearch", "Web Search"),
    (r"SOFTWARE\Policies\Microsoft\Windows\Windows Search", "ConnectedSearchUseWeb", "Cloud Search"),
    
    // Activity History
    (r"SOFTWARE\Policies\Microsoft\Windows\System", "EnableActivityFeed", "Activity Feed"),
    (r"SOFTWARE\Policies\Microsoft\Windows\System", "PublishUserActivities", "Publish Activities"),
    (r"SOFTWARE\Policies\Microsoft\Windows\System", "UploadUserActivities", "Upload Activities"),
    
    // Customer Experience
    (r"SOFTWARE\Policies\Microsoft\SQMClient\Windows", "CEIPEnable", "CEIP"),
    (r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "AITEnable", "App Telemetry"),
    
    // Error Reporting
    (r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting", "Disabled", "Error Reporting"),
    (r"SOFTWARE\Microsoft\Windows\Windows Error Reporting", "AutoApproveOSDumps", "Auto OS Dumps"),
    
    // Handwriting/Typing
    (r"SOFTWARE\Policies\Microsoft\InputPersonalization", "RestrictImplicitInkCollection", "Ink Collection"),
    (r"SOFTWARE\Policies\Microsoft\InputPersonalization", "RestrictImplicitTextCollection", "Text Collection"),
    
    // Timeline
    (r"SOFTWARE\Policies\Microsoft\Windows\System", "EnableCdp", "Timeline CDP"),
    
    // Cloud Content
    (r"SOFTWARE\Policies\Microsoft\Windows\CloudContent", "DisableWindowsConsumerFeatures", "Consumer Features"),
    (r"SOFTWARE\Policies\Microsoft\Windows\CloudContent", "DisableSoftLanding", "Soft Landing"),
    (r"SOFTWARE\Policies\Microsoft\Windows\CloudContent", "DisableCloudOptimizedContent", "Cloud Content"),
];

// ============================================================================
// DEFENDER - Status complet
// ============================================================================

/// Status de Windows Defender
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefenderStatus {
    pub antispyware_enabled: bool,
    pub realtime_protection: bool,
    pub behavior_monitoring: bool,
    pub tamper_protection: bool,
    pub cloud_protection: bool,
    pub sample_submission: u32,
    pub pua_protection: bool,
    pub exclusion_paths: Vec<String>,
    pub exclusion_extensions: Vec<String>,
    pub exclusion_processes: Vec<String>,
}

impl Default for DefenderStatus {
    fn default() -> Self {
        Self {
            antispyware_enabled: true,
            realtime_protection: true,
            behavior_monitoring: true,
            tamper_protection: true,
            cloud_protection: true,
            sample_submission: 1,
            pua_protection: true,
            exclusion_paths: Vec::new(),
            exclusion_extensions: Vec::new(),
            exclusion_processes: Vec::new(),
        }
    }
}

/// Audit complet de Windows Defender
pub fn get_defender_status() -> Result<DefenderStatus> {
    let mut status = DefenderStatus::default();
    
    // AntiSpyware global
    status.antispyware_enabled = read_dword_value(
        r"SOFTWARE\Microsoft\Windows Defender",
        "DisableAntiSpyware"
    ).unwrap_or(0) == 0;
    
    // Real-time protection
    status.realtime_protection = read_dword_value(
        r"SOFTWARE\Microsoft\Windows Defender\Real-Time Protection",
        "DisableRealtimeMonitoring"
    ).unwrap_or(0) == 0;
    
    // Behavior monitoring
    status.behavior_monitoring = read_dword_value(
        r"SOFTWARE\Microsoft\Windows Defender\Real-Time Protection",
        "DisableBehaviorMonitoring"
    ).unwrap_or(0) == 0;
    
    // Tamper protection
    status.tamper_protection = read_dword_value(
        r"SOFTWARE\Microsoft\Windows Defender\Features",
        "TamperProtection"
    ).unwrap_or(0) != 0;
    
    // Cloud protection (MAPS)
    status.cloud_protection = read_dword_value(
        r"SOFTWARE\Microsoft\Windows Defender\Spynet",
        "SpynetReporting"
    ).unwrap_or(0) != 0;
    
    // Sample submission level
    status.sample_submission = read_dword_value(
        r"SOFTWARE\Microsoft\Windows Defender\Spynet",
        "SubmitSamplesConsent"
    ).unwrap_or(1);
    
    // PUA protection
    status.pua_protection = read_dword_value(
        r"SOFTWARE\Microsoft\Windows Defender",
        "PUAProtection"
    ).unwrap_or(0) != 0;
    
    // Exclusions
    status.exclusion_paths = enumerate_registry_values(
        r"SOFTWARE\Microsoft\Windows Defender\Exclusions\Paths"
    ).unwrap_or_default();
    
    status.exclusion_extensions = enumerate_registry_values(
        r"SOFTWARE\Microsoft\Windows Defender\Exclusions\Extensions"
    ).unwrap_or_default();
    
    status.exclusion_processes = enumerate_registry_values(
        r"SOFTWARE\Microsoft\Windows Defender\Exclusions\Processes"
    ).unwrap_or_default();
    
    Ok(status)
}

// ============================================================================
// UAC - Status
// ============================================================================

/// Status UAC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UacStatus {
    pub enabled: bool,
    pub consent_prompt_behavior: u32,
    pub secure_desktop: bool,
    pub admin_approval_mode: bool,
    pub virtualize_file_registry: bool,
}

/// Audit UAC
pub fn get_uac_status() -> Result<UacStatus> {
    Ok(UacStatus {
        enabled: read_dword_value(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System",
            "EnableLUA"
        ).unwrap_or(1) == 1,
        
        consent_prompt_behavior: read_dword_value(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System",
            "ConsentPromptBehaviorAdmin"
        ).unwrap_or(5),
        
        secure_desktop: read_dword_value(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System",
            "PromptOnSecureDesktop"
        ).unwrap_or(1) == 1,
        
        admin_approval_mode: read_dword_value(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System",
            "FilterAdministratorToken"
        ).unwrap_or(0) == 1,
        
        virtualize_file_registry: read_dword_value(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System",
            "EnableVirtualization"
        ).unwrap_or(1) == 1,
    })
}

// ============================================================================
// FIREWALL - Status
// ============================================================================

/// Status Firewall par profil
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallStatus {
    pub domain_enabled: bool,
    pub private_enabled: bool,
    pub public_enabled: bool,
    pub domain_default_inbound_block: bool,
    pub private_default_inbound_block: bool,
    pub public_default_inbound_block: bool,
}

/// Audit Firewall
pub fn get_firewall_status() -> Result<FirewallStatus> {
    let base = r"SYSTEM\CurrentControlSet\Services\SharedAccess\Parameters\FirewallPolicy";
    
    Ok(FirewallStatus {
        domain_enabled: read_dword_value(
            &format!("{}\\DomainProfile", base),
            "EnableFirewall"
        ).unwrap_or(1) == 1,
        
        private_enabled: read_dword_value(
            &format!("{}\\StandardProfile", base),
            "EnableFirewall"
        ).unwrap_or(1) == 1,
        
        public_enabled: read_dword_value(
            &format!("{}\\PublicProfile", base),
            "EnableFirewall"
        ).unwrap_or(1) == 1,
        
        domain_default_inbound_block: read_dword_value(
            &format!("{}\\DomainProfile", base),
            "DefaultInboundAction"
        ).unwrap_or(1) == 1,
        
        private_default_inbound_block: read_dword_value(
            &format!("{}\\StandardProfile", base),
            "DefaultInboundAction"
        ).unwrap_or(1) == 1,
        
        public_default_inbound_block: read_dword_value(
            &format!("{}\\PublicProfile", base),
            "DefaultInboundAction"
        ).unwrap_or(1) == 1,
    })
}

// ============================================================================
// TÉLÉMÉTRIE - API publique
// ============================================================================

/// Récupère le statut complet de la télémétrie
pub fn get_telemetry_status() -> Result<TelemetryStatus> {
    let diagtrack_start = read_dword_value(
        r"SYSTEM\CurrentControlSet\Services\DiagTrack",
        "Start",
    ).unwrap_or(2);
    
    // Vérifier niveau GPO d'abord, puis user setting
    let data_collection = read_dword_value(
        r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
        "AllowTelemetry",
    ).or_else(|_| read_dword_value(
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection",
        "AllowTelemetry",
    )).unwrap_or(3);
    
    let advertising_id = read_dword_value(
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\AdvertisingInfo",
        "Enabled",
    ).unwrap_or(1) == 1;
    
    // Location consent (REG_SZ "Allow" ou "Deny")
    let location = read_string_value(
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location",
        "Value",
    ).map(|v| v.to_lowercase() == "allow").unwrap_or(true);
    
    // Activity History
    let activity_history = read_dword_value(
        r"SOFTWARE\Policies\Microsoft\Windows\System",
        "PublishUserActivities",
    ).unwrap_or(1) == 1;
    
    // Cortana
    let cortana_enabled = read_dword_value(
        r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
        "AllowCortana",
    ).unwrap_or(1) == 1;
    
    // Web Search
    let web_search_enabled = read_dword_value(
        r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
        "DisableWebSearch",
    ).unwrap_or(0) == 0;
    
    // Error Reporting
    let error_reporting = read_dword_value(
        r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting",
        "Disabled",
    ).unwrap_or(0) == 0;
    
    Ok(TelemetryStatus {
        diagtrack_enabled: diagtrack_start != 4,
        data_collection_level: data_collection,
        advertising_id_enabled: advertising_id,
        location_enabled: location,
        activity_history_enabled: activity_history,
        cortana_enabled,
        web_search_enabled,
        error_reporting_enabled: error_reporting,
    })
}

// ============================================================================
// HELPERS - Lecture registre
// ============================================================================

/// Lit une valeur DWORD du registre (HKLM)
pub fn read_dword_value(subkey: &str, value_name: &str) -> Result<u32> {
    read_dword_value_from_hive(HKEY_LOCAL_MACHINE, subkey, value_name)
}

/// Lit une valeur DWORD du registre (HKCU)
pub fn read_dword_value_hkcu(subkey: &str, value_name: &str) -> Result<u32> {
    read_dword_value_from_hive(HKEY_CURRENT_USER, subkey, value_name)
}

fn read_dword_value_from_hive(hive: HKEY, subkey: &str, value_name: &str) -> Result<u32> {
    unsafe {
        let mut hkey = Default::default();
        let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
        
        let result = RegOpenKeyExW(
            hive,
            PCWSTR(subkey_wide.as_ptr()),
            0,
            KEY_READ,
            &mut hkey,
        );
        
        if result.is_err() {
            return Err(PieuvreError::Registry(format!("Cannot open key: {}", subkey)));
        }
        
        let value_wide: Vec<u16> = value_name.encode_utf16().chain(std::iter::once(0)).collect();
        let mut data = 0u32;
        let mut data_size = std::mem::size_of::<u32>() as u32;
        let mut value_type = REG_DWORD;
        
        let result = RegQueryValueExW(
            hkey,
            PCWSTR(value_wide.as_ptr()),
            None,
            Some(&mut value_type),
            Some(&mut data as *mut u32 as *mut u8),
            Some(&mut data_size),
        );
        
        let _ = RegCloseKey(hkey);
        
        if result.is_err() {
            return Err(PieuvreError::Registry(format!("Cannot read value: {}", value_name)));
        }
        
        Ok(data)
    }
}

/// Lit une valeur String du registre
pub fn read_string_value(subkey: &str, value_name: &str) -> Result<String> {
    unsafe {
        let mut hkey = Default::default();
        let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
        
        if RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(subkey_wide.as_ptr()),
            0,
            KEY_READ,
            &mut hkey,
        ).is_err() {
            return Err(PieuvreError::Registry(format!("Cannot open key: {}", subkey)));
        }
        
        let value_wide: Vec<u16> = value_name.encode_utf16().chain(std::iter::once(0)).collect();
        let mut buffer = vec![0u8; 1024];
        let mut data_size = buffer.len() as u32;
        let mut value_type = REG_SZ;
        
        let result = RegQueryValueExW(
            hkey,
            PCWSTR(value_wide.as_ptr()),
            None,
            Some(&mut value_type),
            Some(buffer.as_mut_ptr()),
            Some(&mut data_size),
        );
        
        let _ = RegCloseKey(hkey);
        
        if result.is_err() {
            return Err(PieuvreError::Registry(format!("Cannot read value: {}", value_name)));
        }
        
        // Convertir UTF-16 en String
        let chars = data_size as usize / 2;
        let s = String::from_utf16_lossy(
            std::slice::from_raw_parts(buffer.as_ptr() as *const u16, chars.saturating_sub(1))
        );
        
        Ok(s)
    }
}

/// Énumère les noms de valeurs d'une clé (pour les exclusions Defender)
pub fn enumerate_registry_values(subkey: &str) -> Result<Vec<String>> {
    unsafe {
        let mut hkey = Default::default();
        let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
        
        if RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(subkey_wide.as_ptr()),
            0,
            KEY_READ,
            &mut hkey,
        ).is_err() {
            return Ok(Vec::new()); // Clé n'existe pas = pas d'exclusions
        }
        
        let mut values = Vec::new();
        let mut index = 0u32;
        
        loop {
            let mut name_buffer = vec![0u16; 1024];
            let mut name_len = name_buffer.len() as u32;
            
            let result = RegEnumValueW(
                hkey,
                index,
                windows::core::PWSTR(name_buffer.as_mut_ptr()),
                &mut name_len,
                None,
                None,
                None,
                None,
            );
            
            if result.is_err() {
                break;
            }
            
            let name = String::from_utf16_lossy(&name_buffer[..name_len as usize]);
            if !name.is_empty() {
                values.push(name);
            }
            
            index += 1;
        }
        
        let _ = RegCloseKey(hkey);
        Ok(values)
    }
}

/// Vérifie si une clé existe
pub fn key_exists(subkey: &str) -> bool {
    unsafe {
        let mut hkey = Default::default();
        let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
        
        let exists = RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(subkey_wide.as_ptr()),
            0,
            KEY_READ,
            &mut hkey,
        ).is_ok();
        
        if exists {
            let _ = RegCloseKey(hkey);
        }
        
        exists
    }
}

/// Vérifie si Secure Boot est activé
pub fn is_secure_boot_enabled() -> bool {
    read_dword_value(
        r"SYSTEM\CurrentControlSet\Control\SecureBoot\State",
        "UEFISecureBootEnabled"
    ).unwrap_or(0) == 1
}

/// Vérifie si Credential Guard est activé
pub fn is_credential_guard_enabled() -> bool {
    let config = read_dword_value(
        r"SYSTEM\CurrentControlSet\Control\DeviceGuard",
        "EnableVirtualizationBasedSecurity"
    ).unwrap_or(0);
    
    config != 0
}
