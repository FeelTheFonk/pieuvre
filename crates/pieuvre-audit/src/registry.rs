//! Inspection du registre
//!
//! Parsing des ruches et détection des clés télémétrie.

use pieuvre_common::{PieuvreError, Result, TelemetryStatus};
use windows::Win32::System::Registry::{
    RegOpenKeyExW, RegQueryValueExW, RegCloseKey, HKEY_LOCAL_MACHINE, KEY_READ, REG_DWORD,
};
use windows::core::PCWSTR;

/// Clés registre liées à la télémétrie (référence)
#[allow(dead_code)]
const TELEMETRY_KEYS: &[(&str, &str)] = &[
    (r"SOFTWARE\Policies\Microsoft\Windows\DataCollection", "AllowTelemetry"),
    (r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection", "AllowTelemetry"),
    (r"SYSTEM\CurrentControlSet\Services\DiagTrack", "Start"),
    (r"SYSTEM\CurrentControlSet\Services\dmwappushservice", "Start"),
];

/// Récupère le statut de la télémétrie
pub fn get_telemetry_status() -> Result<TelemetryStatus> {
    let diagtrack_start = read_dword_value(
        r"SYSTEM\CurrentControlSet\Services\DiagTrack",
        "Start",
    ).unwrap_or(2); // 2 = Automatic
    
    let data_collection = read_dword_value(
        r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
        "AllowTelemetry",
    ).unwrap_or(3); // 3 = Full
    
    let advertising_id = read_dword_value(
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\AdvertisingInfo",
        "Enabled",
    ).unwrap_or(1) == 1;
    
    let location = read_dword_value(
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location",
        "Value",
    ).map(|v| v == 1).unwrap_or(true);
    
    Ok(TelemetryStatus {
        diagtrack_enabled: diagtrack_start != 4, // 4 = Disabled
        data_collection_level: data_collection,
        advertising_id_enabled: advertising_id,
        location_enabled: location,
    })
}

/// Lit une valeur DWORD du registre
fn read_dword_value(subkey: &str, value_name: &str) -> Result<u32> {
    unsafe {
        let mut hkey = Default::default();
        let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
        
        let result = RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
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

/// Vérifie si une clé existe
pub fn key_exists(subkey: &str) -> bool {
    unsafe {
        let mut hkey = Default::default();
        let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
        
        RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(subkey_wide.as_ptr()),
            0,
            KEY_READ,
            &mut hkey,
        ).is_ok()
    }
}
