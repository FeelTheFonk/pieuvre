//! MSI Configurator
//!
//! Activation du mode MSI (Message Signaled Interrupts) pour réduire la latence.

use pieuvre_common::{PieuvreError, Result};
use windows::Win32::System::Registry::{
    RegOpenKeyExW, RegSetValueExW, RegCloseKey, RegEnumKeyExW, RegQueryValueExW,
    HKEY_LOCAL_MACHINE, KEY_READ, KEY_SET_VALUE, REG_DWORD,
};
use windows::core::PCWSTR;

/// Clé registre pour les propriétés MSI
const PCI_ENUM_BASE: &str = r"SYSTEM\CurrentControlSet\Enum\PCI";
const MSI_SUBPATH: &str = r"Device Parameters\Interrupt Management\MessageSignaledInterruptProperties";

#[derive(Debug, Clone)]
pub struct MsiDevice {
    pub device_id: String,
    pub full_path: String,  // Chemin complet registre pour enable/disable
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
        
        if RegOpenKeyExW(HKEY_LOCAL_MACHINE, PCWSTR(base_key.as_ptr()), Some(0), KEY_READ, &mut hkey).is_err() {
            return Ok(devices);
        }
        
        // Énumérer les devices PCI
        let mut index = 0u32;
        loop {
            let mut device_id_buf = vec![0u16; 256];
            let mut name_len = device_id_buf.len() as u32;
            
            let result = RegEnumKeyExW(
                hkey,
                index,
                Some(windows::core::PWSTR(device_id_buf.as_mut_ptr())),
                &mut name_len,
                None,
                None,
                None,
                None,
            );
            
            if result.is_err() {
                break;
            }
            
            let dev_id = String::from_utf16_lossy(&device_id_buf[..name_len as usize]);
            
            // Vérifier si c'est un GPU ou NVMe (cibles principales)
            let is_target = dev_id.contains("VEN_10DE") || // NVIDIA
                           dev_id.contains("VEN_1002") || // AMD
                           dev_id.contains("VEN_8086") || // Intel
                           dev_id.contains("VEN_144D") || // Samsung NVMe
                           dev_id.contains("VEN_1987") || // Phison NVMe
                           dev_id.contains("VEN_15B7");   // WD NVMe
            
            if is_target {
                // Récupérer le statut MSI avec le chemin complet
                if let Some((full_path, msi_supported, msi_enabled)) = get_msi_info(&dev_id) {
                    devices.push(MsiDevice {
                        device_id: dev_id.clone(),
                        full_path,
                        description: categorize_device(&dev_id),
                        msi_supported,
                        msi_enabled,
                    });
                }
            }
            
            index += 1;
            if index > 500 {
                break;
            }
        }
        
        let _ = RegCloseKey(hkey);
    }
    
    Ok(devices)
}

/// Retourne (full_registry_path, msi_supported, msi_enabled)
fn get_msi_info(device_id: &str) -> Option<(String, bool, bool)> {
    unsafe {
        // Ouvrir la clé du device pour énumérer les instances
        let device_path = format!(r"{}\{}", PCI_ENUM_BASE, device_id);
        let device_key: Vec<u16> = device_path.encode_utf16().chain(std::iter::once(0)).collect();
        let mut hkey_device = Default::default();
        
        if RegOpenKeyExW(HKEY_LOCAL_MACHINE, PCWSTR(device_key.as_ptr()), Some(0), KEY_READ, &mut hkey_device).is_err() {
            return None;
        }
        
        // Énumérer les instances (ex: "3&2411e6fe&0&00E5")
        let mut instance_buffer = vec![0u16; 256];
        let mut instance_len = instance_buffer.len() as u32;
        
        let result = RegEnumKeyExW(
            hkey_device,
            0, // Première instance
            Some(windows::core::PWSTR(instance_buffer.as_mut_ptr())),
            &mut instance_len,
            None,
            None,
            None,
            None,
        );
        
        let _ = RegCloseKey(hkey_device);
        
        if result.is_err() {
            return None;
        }
        
        let instance = String::from_utf16_lossy(&instance_buffer[..instance_len as usize]);
        
        // Construire le chemin complet vers MSI properties
        let msi_path = format!(r"{}\{}\{}\{}", PCI_ENUM_BASE, device_id, instance, MSI_SUBPATH);
        let msi_key: Vec<u16> = msi_path.encode_utf16().chain(std::iter::once(0)).collect();
        let mut hkey_msi = Default::default();
        
        if RegOpenKeyExW(HKEY_LOCAL_MACHINE, PCWSTR(msi_key.as_ptr()), Some(0), KEY_READ, &mut hkey_msi).is_err() {
            return None; // MSI key n'existe pas = non supporté
        }
        
        // Lire MSISupported
        let value_name: Vec<u16> = "MSISupported".encode_utf16().chain(std::iter::once(0)).collect();
        let mut msi_value: u32 = 0;
        let mut data_size = std::mem::size_of::<u32>() as u32;
        let mut value_type = REG_DWORD;
        
        let read_ok = RegQueryValueExW(
            hkey_msi,
            PCWSTR(value_name.as_ptr()),
            None,
            Some(&mut value_type),
            Some(&mut msi_value as *mut u32 as *mut u8),
            Some(&mut data_size),
        ).is_ok();
        
        let _ = RegCloseKey(hkey_msi);
        
        if read_ok && msi_value == 1 {
            // MSI supporté si la clé existe, activé si valeur = 1
            Some((msi_path, true, true))
        } else if read_ok {
            Some((msi_path, true, false))
        } else {
            Some((msi_path, false, false))
        }
    }
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

/// Active le MSI-Mode pour un device (utiliser full_path de MsiDevice)
pub fn enable_msi(full_registry_path: &str) -> Result<()> {
    set_msi_value(full_registry_path, 1)
}

/// Désactive le MSI-Mode pour un device
pub fn disable_msi(full_registry_path: &str) -> Result<()> {
    set_msi_value(full_registry_path, 0)
}

fn set_msi_value(key_path: &str, value: u32) -> Result<()> {
    unsafe {
        let subkey_wide: Vec<u16> = key_path.encode_utf16().chain(std::iter::once(0)).collect();
        let mut hkey = Default::default();
        
        let result = RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(subkey_wide.as_ptr()),
            Some(0),
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
            Some(0),
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

/// Vérifie si le MSI est déjà activé pour tous les GPU
pub fn is_msi_enabled_on_gpu() -> bool {
    match list_msi_eligible_devices() {
        Ok(devices) => {
            let gpus: Vec<_> = devices.iter()
                .filter(|d| d.description.contains("GPU"))
                .collect();
            
            if gpus.is_empty() {
                return true; // Pas de GPU à configurer
            }
            
            gpus.iter().all(|d| d.msi_enabled)
        }
        Err(_) => false,
    }
}
