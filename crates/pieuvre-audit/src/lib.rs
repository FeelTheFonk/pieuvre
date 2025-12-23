pub mod appx;
pub mod compliance;
pub mod etw;
pub mod hardware;
pub mod network;
pub mod registry;
pub mod security;
pub mod services;

pub use compliance::{check_compliance, ComplianceCheck, ComplianceStatus};
pub use security::run_security_audit;

use chrono::Utc;
use pieuvre_common::{AuditReport, Result, SystemInfo};
use uuid::Uuid;

pub fn full_audit() -> Result<AuditReport> {
    let hardware = hardware::probe_hardware()?;
    let security = security::run_security_audit()?;
    let telemetry = security::get_telemetry_status()?;
    let services = services::inspect_services()?;
    let appx = appx::scan_packages()?;

    // System Info detection (SOTA: Real detection via Registry)
    let (os_version, build_number) = {
        let hklm = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);
        let key = hklm
            .open_subkey(r"SOFTWARE\Microsoft\Windows NT\CurrentVersion")
            .ok();

        let ver = key
            .as_ref()
            .and_then(|k: &winreg::RegKey| k.get_value("ProductName").ok())
            .unwrap_or_else(|| "Windows 11".to_string());

        let build = key
            .as_ref()
            .and_then(|k: &winreg::RegKey| k.get_value("CurrentBuildNumber").ok())
            .and_then(|s: String| s.parse::<u32>().ok())
            .unwrap_or(22631);

        (ver, build)
    };

    let hostname = std::env::var("COMPUTERNAME").unwrap_or_else(|_| "Unknown".into());

    Ok(AuditReport {
        id: Uuid::new_v4(),
        timestamp: Utc::now(),
        system: SystemInfo {
            os_version,
            build_number,
            edition: "Pro".to_string(),
            hostname,
        },
        hardware,
        services,
        telemetry,
        security,
        latency: None, // ETW latency is optional and heavy
        appx,
    })
}
