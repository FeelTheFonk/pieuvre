//! Pieuvre Audit Engine
//!
//! Module d'audit système bas-niveau pour Windows 11.
//! Fournit l'introspection du registre, la détection matérielle,
//! le monitoring de latence, et l'inventaire des packages.

pub mod appx;
pub mod hardware;
pub mod network;
pub mod registry;
pub mod services;

use pieuvre_common::{AuditReport, Result, SystemInfo};
use chrono::Utc;
use uuid::Uuid;

/// Effectue un audit complet du système
pub fn full_audit() -> Result<AuditReport> {
    tracing::info!("Démarrage audit complet");
    
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
        latency: None, // ETW séparé (nécessite durée)
        appx: appx_packages,
    })
}

fn get_system_info() -> Result<SystemInfo> {
    // Utiliser RtlGetVersion pour une info système précise
    Ok(SystemInfo {
        os_version: "Windows 11".into(),
        build_number: get_build_number(),
        edition: "Pro".into(),
        hostname: std::env::var("COMPUTERNAME").unwrap_or_default(),
    })
}

fn get_build_number() -> u32 {
    // Lire depuis le registre pour précision
    use windows::Win32::System::Registry::{RegOpenKeyExW, RegQueryValueExW, RegCloseKey, HKEY_LOCAL_MACHINE, KEY_READ};
    use windows::core::PCWSTR;
    
    unsafe {
        let subkey: Vec<u16> = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion"
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        
        let mut hkey = Default::default();
        if RegOpenKeyExW(HKEY_LOCAL_MACHINE, PCWSTR(subkey.as_ptr()), 0, KEY_READ, &mut hkey).is_ok() {
            let value_name: Vec<u16> = "CurrentBuildNumber".encode_utf16().chain(std::iter::once(0)).collect();
            let mut buffer = vec![0u8; 32];
            let mut size = buffer.len() as u32;
            
            if RegQueryValueExW(
                hkey,
                PCWSTR(value_name.as_ptr()),
                None,
                None,
                Some(buffer.as_mut_ptr()),
                Some(&mut size),
            ).is_ok() {
                let _ = RegCloseKey(hkey);
                let s = String::from_utf16_lossy(
                    std::slice::from_raw_parts(buffer.as_ptr() as *const u16, (size as usize / 2).saturating_sub(1))
                );
                return s.parse().unwrap_or(22631);
            }
            let _ = RegCloseKey(hkey);
        }
    }
    22631
}
