//! MSI Configurator
//!
//! Activation du mode MSI (Message Signaled Interrupts) pour réduire la latence.

use pieuvre_common::{PieuvreError, Result};
use windows::Win32::System::Registry::{
    RegOpenKeyExW, RegSetValueExW, RegCloseKey, RegEnumKeyExW,
    HKEY_LOCAL_MACHINE, KEY_READ, KEY_SET_VALUE, REG_DWORD,
};
use windows::core::PCWSTR;

/// Clé registre pour les propriétés MSI
const PCI_ENUM_BASE: &str = r"SYSTEM\CurrentControlSet\Enum\PCI";
const MSI_SUBPATH: &str = r"Device Parameters\Interrupt Management\MessageSignaledInterruptProperties";

#[derive(Debug, Clone)]
pub struct MsiDevice {
    pub device_id: String,
    pub description: String,
    pub msi_supported: bool,
    pub msi_enabled: bool,
}

/// Énumère les devices PCI éligibles au MSI-Mode
pub fn list_msi_eligible_devices() -> Result<Vec<MsiDevice>> {
    let mut devices = Vec::new();
    
    unsafe {
        let base_key: Vec<u16> = PCI_ENUM_BASE.encode_utf16().chain(std::iter::once(0)).collect();
        let mut hkey = Default::default();
        
        if RegOpenKeyExW(HKEY_LOCAL_MACHINE, PCWSTR(base_key.as_ptr()), 0, KEY_READ, &mut hkey).is_err() {
            return Ok(devices);
        }
        
        // Énumérer les devices PCI
        let mut index = 0u32;
        loop {
            let mut device_id = vec![0u16; 256];
            let mut name_len = device_id.len() as u32;
            
            let result = RegEnumKeyExW(
                hkey,
                index,
                windows::core::PWSTR(device_id.as_mut_ptr()),
                &mut name_len,
                None,
                windows::core::PWSTR::null(),
                None,
                None,
            );
            
            if result.is_err() {
                break;
            }
            
            let dev_id = String::from_utf16_lossy(&device_id[..name_len as usize]);
            
            // Vérifier si c'est un GPU ou NVMe (cibles principales)
            let is_target = dev_id.contains("VEN_10DE") || // NVIDIA
                           dev_id.contains("VEN_1002") || // AMD
                           dev_id.contains("VEN_8086") || // Intel
                           dev_id.contains("VEN_144D") || // Samsung NVMe
                           dev_id.contains("VEN_1987") || // Phison NVMe
                           dev_id.contains("VEN_15B7");   // WD NVMe
            
            if is_target {
                // Vérifier statut MSI
                let msi_status = check_msi_status(&dev_id);
                
                devices.push(MsiDevice {
                    device_id: dev_id.clone(),
                    description: categorize_device(&dev_id),
                    msi_supported: msi_status.0,
                    msi_enabled: msi_status.1,
                });
            }
            
            index += 1;
            if index > 500 {
                break; // Limite raisonnable
            }
        }
        
        let _ = RegCloseKey(hkey);
    }
    
    Ok(devices)
}

fn check_msi_status(device_id: &str) -> (bool, bool) {
    // Chercher la sous-clé pour ce device
    // Structure: PCI\{device_id}\{instance}\Device Parameters\Interrupt Management\MessageSignaledInterruptProperties
    // Retourne (supported, enabled)
    (true, false) // Simplifié - en production: lecture réelle du registre
}

fn categorize_device(device_id: &str) -> String {
    if device_id.contains("VEN_10DE") {
        "NVIDIA GPU".to_string()
    } else if device_id.contains("VEN_1002") {
        "AMD GPU".to_string()
    } else if device_id.contains("VEN_8086") {
        "Intel Device".to_string()
    } else if device_id.contains("VEN_144D") || device_id.contains("VEN_1987") || device_id.contains("VEN_15B7") {
        "NVMe Controller".to_string()
    } else {
        "PCI Device".to_string()
    }
}

/// Active le MSI-Mode pour un device
pub fn enable_msi(device_instance_path: &str) -> Result<()> {
    let key_path = format!(
        r"SYSTEM\CurrentControlSet\Enum\{}\{}",
        device_instance_path,
        MSI_SUBPATH
    );
    
    set_msi_registry(&key_path, 1)
}

/// Désactive le MSI-Mode pour un device
pub fn disable_msi(device_instance_path: &str) -> Result<()> {
    let key_path = format!(
        r"SYSTEM\CurrentControlSet\Enum\{}\{}",
        device_instance_path,
        MSI_SUBPATH
    );
    
    set_msi_registry(&key_path, 0)
}

fn set_msi_registry(key_path: &str, value: u32) -> Result<()> {
    unsafe {
        let subkey_wide: Vec<u16> = key_path.encode_utf16().chain(std::iter::once(0)).collect();
        let mut hkey = Default::default();
        
        let result = RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(subkey_wide.as_ptr()),
            0,
            KEY_SET_VALUE,
            &mut hkey,
        );
        
        if result.is_err() {
            return Err(PieuvreError::Registry(format!("Cannot open MSI key: {}", key_path)));
        }
        
        let value_name: Vec<u16> = "MSISupported".encode_utf16().chain(std::iter::once(0)).collect();
        let data_bytes = value.to_le_bytes();
        
        let result = RegSetValueExW(
            hkey,
            PCWSTR(value_name.as_ptr()),
            0,
            REG_DWORD,
            Some(&data_bytes),
        );
        
        let _ = RegCloseKey(hkey);
        
        if result.is_err() {
            return Err(PieuvreError::Registry("Cannot set MSISupported".to_string()));
        }
        
        tracing::info!("MSI {} pour {}", if value == 1 { "activé" } else { "désactivé" }, key_path);
        Ok(())
    }
}
