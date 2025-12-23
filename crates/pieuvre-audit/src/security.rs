use crate::registry::{key_exists, read_hklm_dword};
use pieuvre_common::{Result, SecurityAudit, TelemetryStatus};
use windows::Win32::System::Registry::HKEY_LOCAL_MACHINE;

pub fn run_security_audit() -> Result<SecurityAudit> {
    let defender_enabled = read_hklm_dword(
        r"SOFTWARE\Microsoft\Windows Defender\Real-Time Protection",
        "DisableRealtimeMonitoring",
    )
    .unwrap_or(0)
        == 0;
    let tamper_protection = read_hklm_dword(
        r"SOFTWARE\Microsoft\Windows Defender\Features",
        "TamperProtection",
    )
    .unwrap_or(0)
        != 0;
    let firewall_enabled = read_hklm_dword(
        r"SYSTEM\CurrentControlSet\Services\SharedAccess\Parameters\FirewallPolicy\StandardProfile",
        "EnableFirewall",
    )
    .unwrap_or(0)
        == 1;
    let uac_level = read_hklm_dword(
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System",
        "ConsentPromptBehaviorAdmin",
    )
    .unwrap_or(0);
    let secure_boot = read_hklm_dword(
        r"SYSTEM\CurrentControlSet\Control\SecureBoot\State",
        "UEFISecureBootEnabled",
    )
    .unwrap_or(0)
        == 1;
    let credential_guard =
        read_hklm_dword(r"SYSTEM\CurrentControlSet\Control\Lsa", "LsaCfgFlags").unwrap_or(0) >= 1;
    let bitlocker_active = key_exists(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\BitlockerStatus",
    );

    Ok(SecurityAudit {
        defender_enabled,
        tamper_protection,
        firewall_enabled,
        uac_level,
        secure_boot,
        credential_guard,
        bitlocker_active,
    })
}

pub fn get_telemetry_status() -> Result<TelemetryStatus> {
    let diagtrack_enabled = crate::services::get_service_start_type_by_name("DiagTrack")
        != pieuvre_common::ServiceStartType::Disabled;
    let data_collection_level = read_hklm_dword(
        r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
        "AllowTelemetry",
    )
    .unwrap_or(1);

    Ok(TelemetryStatus {
        diagtrack_enabled,
        data_collection_level,
        advertising_id_enabled: read_hklm_dword(r"SOFTWARE\Microsoft\Windows\CurrentVersion\AdvertisingInfo", "Enabled").unwrap_or(1) == 1,
        location_enabled: read_hklm_dword(r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location", "Value").unwrap_or(0) == 1,
        activity_history_enabled: read_hklm_dword(r"SOFTWARE\Policies\Microsoft\Windows\System", "PublishUserActivities").unwrap_or(1) == 1,
        cortana_enabled: read_hklm_dword(r"SOFTWARE\Policies\Microsoft\Windows\Windows Search", "AllowCortana").unwrap_or(1) == 1,
        web_search_enabled: read_hklm_dword(r"SOFTWARE\Policies\Microsoft\Windows\Windows Search", "ConnectedSearchUseWeb").unwrap_or(1) == 1,
        error_reporting_enabled: read_hklm_dword(r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting", "Disabled").unwrap_or(0) == 0,
    })
}
