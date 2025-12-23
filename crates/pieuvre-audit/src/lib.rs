//! pieuvre Audit Engine
//!
//! Low-level system audit module for Windows 11.
//! Provides registry introspection, hardware detection,
//! latency monitoring, package inventory, and security audit.

pub mod appx;
pub mod compliance;
pub mod etw;
pub mod hardware;
pub mod network;
pub mod registry;
pub mod security;
pub mod services;

#[cfg(test)]
mod tests;

use chrono::Utc;
use pieuvre_common::{AuditReport, Result, SystemInfo};
use uuid::Uuid;

// Re-exports for public API
pub use registry::{DefenderStatus, FirewallStatus, UacStatus};
pub use security::{SecurityAudit, SecurityRecommendation, Severity};

/// Performs a full system audit
pub fn full_audit() -> Result<AuditReport> {
    tracing::info!("Starting full system audit");

    let system = get_system_info()?;
    let hardware = hardware::probe_hardware()?;
    let services = services::inspect_services()?;
    let telemetry = registry::get_telemetry_status()?;
    let appx_packages = appx::scan_packages()?;

    Ok(AuditReport {
        id: Uuid::new_v4(),
        timestamp: Utc::now(),
        system,
        hardware,
        services,
        telemetry,
        latency: None, // Separate ETW (requires duration)
        appx: appx_packages,
    })
}

/// Performs a full security audit
pub fn security_audit() -> Result<SecurityAudit> {
    tracing::info!("Starting security audit");
    security::audit_security()
}

/// Retrieves system information
fn get_system_info() -> Result<SystemInfo> {
    let (os_version, edition) = get_os_info();

    Ok(SystemInfo {
        os_version,
        build_number: get_build_number(),
        edition,
        hostname: std::env::var("COMPUTERNAME").unwrap_or_default(),
    })
}

/// Retrieves OS version and edition from registry
fn get_os_info() -> (String, String) {
    use windows::core::PCWSTR;
    use windows::Win32::System::Registry::{
        RegCloseKey, RegOpenKeyExW, HKEY_LOCAL_MACHINE, KEY_READ,
    };

    unsafe {
        let subkey: Vec<u16> = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion"
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();

        let mut hkey = Default::default();
        if RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(subkey.as_ptr()),
            Some(0),
            KEY_READ,
            &mut hkey,
        )
        .is_err()
        {
            return ("Windows 11".into(), "Unknown".into());
        }

        // ProductName
        let product_name =
            read_string_value_raw(hkey, "ProductName").unwrap_or_else(|| "Windows 11".into());

        // EditionID
        let edition = read_string_value_raw(hkey, "EditionID").unwrap_or_else(|| "Unknown".into());

        let _ = RegCloseKey(hkey);

        (product_name, edition)
    }
}

/// Reads a string value from an already open key
fn read_string_value_raw(
    hkey: windows::Win32::System::Registry::HKEY,
    value_name: &str,
) -> Option<String> {
    use windows::core::PCWSTR;
    use windows::Win32::System::Registry::RegQueryValueExW;

    unsafe {
        let value_wide: Vec<u16> = value_name
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        let mut buffer = vec![0u8; 512];
        let mut size = buffer.len() as u32;

        if RegQueryValueExW(
            hkey,
            PCWSTR(value_wide.as_ptr()),
            None,
            None,
            Some(buffer.as_mut_ptr()),
            Some(&mut size),
        )
        .is_ok()
        {
            let s = String::from_utf16_lossy(std::slice::from_raw_parts(
                buffer.as_ptr() as *const u16,
                (size as usize / 2).saturating_sub(1),
            ));
            Some(s.trim().to_string())
        } else {
            None
        }
    }
}

/// Retrieves the build number
fn get_build_number() -> u32 {
    use windows::core::PCWSTR;
    use windows::Win32::System::Registry::{
        RegCloseKey, RegOpenKeyExW, RegQueryValueExW, HKEY_LOCAL_MACHINE, KEY_READ,
    };

    unsafe {
        let subkey: Vec<u16> = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion"
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();

        let mut hkey = Default::default();
        if RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(subkey.as_ptr()),
            Some(0),
            KEY_READ,
            &mut hkey,
        )
        .is_ok()
        {
            let value_name: Vec<u16> = "CurrentBuildNumber"
                .encode_utf16()
                .chain(std::iter::once(0))
                .collect();
            let mut buffer = vec![0u8; 32];
            let mut size = buffer.len() as u32;

            if RegQueryValueExW(
                hkey,
                PCWSTR(value_name.as_ptr()),
                None,
                None,
                Some(buffer.as_mut_ptr()),
                Some(&mut size),
            )
            .is_ok()
            {
                let _ = RegCloseKey(hkey);
                let s = String::from_utf16_lossy(std::slice::from_raw_parts(
                    buffer.as_ptr() as *const u16,
                    (size as usize / 2).saturating_sub(1),
                ));
                return s.parse().unwrap_or(22631);
            }
            let _ = RegCloseKey(hkey);
        }
    }
    22631
}

/// Checks if the system is a laptop
pub fn is_laptop() -> bool {
    hardware::is_laptop()
}

/// Quick network audit
pub fn network_audit() -> Result<network::NetworkStatus> {
    network::inspect_network()
}
