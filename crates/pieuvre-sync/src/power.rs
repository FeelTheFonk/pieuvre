//! Power Manager SOTA 2026
//!
//! Gestion des plans d'alimentation et paramètres énergétiques.
//! Utilise les APIs Windows natives (PowerGetActiveScheme, PowerSetActiveScheme).

use pieuvre_common::{PieuvreError, Result};
use windows::Win32::System::Power::{
    PowerGetActiveScheme, PowerSetActiveScheme,
};
use windows::Win32::Foundation::{LocalFree, HLOCAL};
use windows::core::GUID;

/// Plans d'alimentation prédéfinis
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerPlan {
    /// Économie d'énergie
    PowerSaver,
    /// Équilibré (défaut Windows)
    Balanced,
    /// Hautes performances
    HighPerformance,
    /// Performances ultimes (Windows 10+)
    UltimatePerformance,
}

impl PowerPlan {
    pub fn guid(&self) -> &'static str {
        match self {
            PowerPlan::PowerSaver => "a1841308-3541-4fab-bc81-f71556f20b4a",
            PowerPlan::Balanced => "381b4222-f694-41f0-9685-ff5bb260df2e",
            PowerPlan::HighPerformance => "8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c",
            PowerPlan::UltimatePerformance => "e9a42b02-d5df-448d-aa00-03f14749eb61",
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            PowerPlan::PowerSaver => "Power Saver",
            PowerPlan::Balanced => "Balanced",
            PowerPlan::HighPerformance => "High Performance",
            PowerPlan::UltimatePerformance => "Ultimate Performance",
        }
    }
    
    /// Convertit le GUID string en struct GUID Windows
    pub fn as_guid(&self) -> GUID {
        parse_guid(self.guid())
    }
}

/// Parse un GUID string en struct GUID
fn parse_guid(s: &str) -> GUID {
    // Format: "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 5 {
        return GUID::zeroed();
    }
    
    let data1 = u32::from_str_radix(parts[0], 16).unwrap_or(0);
    let data2 = u16::from_str_radix(parts[1], 16).unwrap_or(0);
    let data3 = u16::from_str_radix(parts[2], 16).unwrap_or(0);
    
    let data4_part1 = u16::from_str_radix(parts[3], 16).unwrap_or(0);
    let data4_part2 = u64::from_str_radix(parts[4], 16).unwrap_or(0);
    
    let data4: [u8; 8] = [
        (data4_part1 >> 8) as u8,
        data4_part1 as u8,
        (data4_part2 >> 40) as u8,
        (data4_part2 >> 32) as u8,
        (data4_part2 >> 24) as u8,
        (data4_part2 >> 16) as u8,
        (data4_part2 >> 8) as u8,
        data4_part2 as u8,
    ];
    
    GUID { data1, data2, data3, data4 }
}

/// Convertit un GUID Windows en string formatté
fn guid_to_string(guid: &GUID) -> String {
    format!(
        "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        guid.data1,
        guid.data2,
        guid.data3,
        guid.data4[0],
        guid.data4[1],
        guid.data4[2],
        guid.data4[3],
        guid.data4[4],
        guid.data4[5],
        guid.data4[6],
        guid.data4[7],
    )
}

/// Récupère le plan d'alimentation actif via API native
pub fn get_active_power_plan() -> Result<String> {
    unsafe {
        let mut scheme_guid: *mut GUID = std::ptr::null_mut();
        
        let result = PowerGetActiveScheme(None, &mut scheme_guid);
        
        if result.is_err() {
            return Err(PieuvreError::Unsupported("PowerGetActiveScheme failed".to_string()));
        }
        
        if scheme_guid.is_null() {
            return Err(PieuvreError::Unsupported("No active power scheme".to_string()));
        }
        
        let guid = *scheme_guid;
        let guid_str = guid_to_string(&guid);
        
        // Libérer la mémoire allouée par Windows
        let _ = LocalFree(Some(HLOCAL(scheme_guid as *mut std::ffi::c_void)));
        
        // Convertir en nom lisible si connu
        let name = match guid_str.as_str() {
            s if s == PowerPlan::PowerSaver.guid() => PowerPlan::PowerSaver.name().to_string(),
            s if s == PowerPlan::Balanced.guid() => PowerPlan::Balanced.name().to_string(),
            s if s == PowerPlan::HighPerformance.guid() => PowerPlan::HighPerformance.name().to_string(),
            s if s == PowerPlan::UltimatePerformance.guid() => PowerPlan::UltimatePerformance.name().to_string(),
            _ => guid_str,
        };
        
        Ok(name)
    }
}

/// Définit le plan d'alimentation actif via API native
pub fn set_power_plan(plan: PowerPlan) -> Result<()> {
    unsafe {
        let guid = plan.as_guid();
        
        let result = PowerSetActiveScheme(None, Some(&guid));
        
        if result.is_err() {
            // En cas d'échec, le plan n'existe peut-être pas
            if plan == PowerPlan::UltimatePerformance {
                create_ultimate_performance_plan()?;
                
                // Réessayer
                let guid = plan.as_guid();
                if PowerSetActiveScheme(None, Some(&guid)).is_err() {
                    // Fallback vers High Performance
                    let hp_guid = PowerPlan::HighPerformance.as_guid();
                    let _ = PowerSetActiveScheme(None, Some(&hp_guid));
                    tracing::warn!("Ultimate Performance non disponible, utilisation High Performance");
                }
            } else {
                return Err(PieuvreError::Unsupported(format!(
                    "PowerSetActiveScheme failed for {}", plan.name()
                )));
            }
        }
        
        tracing::info!(plan = %plan.name(), "Plan d'alimentation activé");
        Ok(())
    }
}

/// Crée le plan Ultimate Performance via powercfg (nécessaire car pas d'API native pour duplicate)
fn create_ultimate_performance_plan() -> Result<()> {
    use std::process::Command;
    
    let output = Command::new("powercfg")
        .args(["-duplicatescheme", "e9a42b02-d5df-448d-aa00-03f14749eb61"])
        .output()
        .map_err(PieuvreError::Io)?;
    
    if !output.status.success() {
        // Créer depuis High Performance
        let _ = Command::new("powercfg")
            .args(["-duplicatescheme", "8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c", "e9a42b02-d5df-448d-aa00-03f14749eb61"])
            .output();
    }
    
    Ok(())
}

/// Configure les paramètres d'alimentation spécifiques
/// Note: Utilise powercfg car PowerWriteACValueIndex nécessite des GUIDs complexes
pub fn configure_power_settings(
    usb_selective_suspend: bool,
    pci_aspm: bool,
    processor_min: u8,
    processor_max: u8,
) -> Result<()> {
    use std::process::Command;
    
    // USB Selective Suspend
    let usb_value = if usb_selective_suspend { "1" } else { "0" };
    let _ = Command::new("powercfg")
        .args(["/setacvalueindex", "scheme_current", "2a737441-1930-4402-8d77-b2bebba308a3", "48e6b7a6-50f5-4782-a5d4-53bb8f07e226", usb_value])
        .output();
    
    // PCI Express Link State Power Management
    let aspm_value = if pci_aspm { "1" } else { "0" };
    let _ = Command::new("powercfg")
        .args(["/setacvalueindex", "scheme_current", "501a4d13-42af-4429-9fd1-a8218c268e20", "ee12f906-d277-404b-b6da-e5fa1a576df5", aspm_value])
        .output();
    
    // Processor Min/Max State
    let min_str = processor_min.to_string();
    let max_str = processor_max.to_string();
    let _ = Command::new("powercfg")
        .args(["/setacvalueindex", "scheme_current", "54533251-82be-4824-96c1-47b60b740d00", "893dee8e-2bef-41e0-89c6-b55d0929964c", &min_str])
        .output();
    let _ = Command::new("powercfg")
        .args(["/setacvalueindex", "scheme_current", "54533251-82be-4824-96c1-47b60b740d00", "bc5038f7-23e0-4960-96da-33abaf5935ec", &max_str])
        .output();
    
    // Appliquer les changements
    let _ = Command::new("powercfg")
        .args(["/setactive", "scheme_current"])
        .output();
    
    tracing::info!(
        usb_suspend = usb_selective_suspend,
        pci_aspm = pci_aspm,
        cpu_min = processor_min,
        cpu_max = processor_max,
        "Paramètres power configurés"
    );
    
    Ok(())
}

/// Désactive l'économie d'énergie du CPU (performance max)
pub fn disable_cpu_throttling() -> Result<()> {
    configure_power_settings(false, false, 100, 100)
}

/// Configuration gaming optimale
pub fn apply_gaming_power_config() -> Result<()> {
    set_power_plan(PowerPlan::UltimatePerformance)?;
    configure_power_settings(false, false, 100, 100)?;
    Ok(())
}
