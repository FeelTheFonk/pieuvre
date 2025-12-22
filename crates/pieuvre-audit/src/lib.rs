//! Pieuvre Audit Engine SOTA
//!
//! Module d'audit système bas-niveau pour Windows 11.
//! Fournit l'introspection du registre, la détection matérielle,
//! le monitoring de latence, l'inventaire des packages, et l'audit sécurité.

pub mod appx;
pub mod hardware;
pub mod network;
pub mod registry;
pub mod security;
pub mod services;

#[cfg(test)]
mod tests;

use pieuvre_common::{AuditReport, Result, SystemInfo};
use chrono::Utc;
use uuid::Uuid;

// Re-exports pour API publique
pub use security::{SecurityAudit, SecurityRecommendation, Severity};
pub use registry::{DefenderStatus, FirewallStatus, UacStatus};

/// Effectue un audit complet du système
pub fn full_audit() -> Result<AuditReport> {
    tracing::info!("Démarrage audit complet SOTA");
    
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

/// Effectue un audit de sécurité complet
pub fn security_audit() -> Result<SecurityAudit> {
    tracing::info!("Démarrage audit sécurité");
    security::audit_security()
}

/// Récupère les informations système
fn get_system_info() -> Result<SystemInfo> {
    let (os_version, edition) = get_os_info();
    
    Ok(SystemInfo {
        os_version,
        build_number: get_build_number(),
        edition,
        hostname: std::env::var("COMPUTERNAME").unwrap_or_default(),
    })
}

/// Récupère version OS et édition depuis le registre
fn get_os_info() -> (String, String) {
    use windows::Win32::System::Registry::{
        RegOpenKeyExW, RegCloseKey, 
        HKEY_LOCAL_MACHINE, KEY_READ,
    };
    use windows::core::PCWSTR;
    
    unsafe {
        let subkey: Vec<u16> = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion"
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        
        let mut hkey = Default::default();
        if RegOpenKeyExW(HKEY_LOCAL_MACHINE, PCWSTR(subkey.as_ptr()), Some(0), KEY_READ, &mut hkey).is_err() {
            return ("Windows 11".into(), "Unknown".into());
        }
        
        // ProductName
        let product_name = read_string_value_raw(hkey, "ProductName")
            .unwrap_or_else(|| "Windows 11".into());
        
        // EditionID
        let edition = read_string_value_raw(hkey, "EditionID")
            .unwrap_or_else(|| "Unknown".into());
        
        let _ = RegCloseKey(hkey);
        
        (product_name, edition)
    }
}

/// Lit une valeur string depuis une clé déjà ouverte
fn read_string_value_raw(hkey: windows::Win32::System::Registry::HKEY, value_name: &str) -> Option<String> {
    use windows::Win32::System::Registry::RegQueryValueExW;
    use windows::core::PCWSTR;
    
    unsafe {
        let value_wide: Vec<u16> = value_name.encode_utf16().chain(std::iter::once(0)).collect();
        let mut buffer = vec![0u8; 512];
        let mut size = buffer.len() as u32;
        
        if RegQueryValueExW(
            hkey,
            PCWSTR(value_wide.as_ptr()),
            None,
            None,
            Some(buffer.as_mut_ptr()),
            Some(&mut size),
        ).is_ok() {
            let s = String::from_utf16_lossy(
                std::slice::from_raw_parts(buffer.as_ptr() as *const u16, (size as usize / 2).saturating_sub(1))
            );
            Some(s.trim().to_string())
        } else {
            None
        }
    }
}

/// Récupère le build number
fn get_build_number() -> u32 {
    use windows::Win32::System::Registry::{
        RegOpenKeyExW, RegQueryValueExW, RegCloseKey, 
        HKEY_LOCAL_MACHINE, KEY_READ,
    };
    use windows::core::PCWSTR;
    
    unsafe {
        let subkey: Vec<u16> = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion"
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        
        let mut hkey = Default::default();
        if RegOpenKeyExW(HKEY_LOCAL_MACHINE, PCWSTR(subkey.as_ptr()), Some(0), KEY_READ, &mut hkey).is_ok() {
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

/// Vérifie si le système est un laptop
pub fn is_laptop() -> bool {
    hardware::is_laptop()
}

/// Audit réseau rapide
pub fn network_audit() -> Result<network::NetworkStatus> {
    network::inspect_network()
}
