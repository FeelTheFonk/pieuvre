//! Power Manager
//!
//! Gestion des plans d'alimentation et paramètres énergétiques.

use pieuvre_common::{PieuvreError, Result};
use std::process::Command;

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
}

/// Récupère le plan d'alimentation actif
pub fn get_active_power_plan() -> Result<String> {
    let output = Command::new("powercfg")
        .args(["/getactivescheme"])
        .output()
        .map_err(|e| PieuvreError::Io(e))?;
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    
    // Extraire le GUID du plan actif
    if let Some(start) = output_str.find('(') {
        if let Some(end) = output_str.find(')') {
            return Ok(output_str[start+1..end].to_string());
        }
    }
    
    Ok("Unknown".to_string())
}

/// Définit le plan d'alimentation actif
pub fn set_power_plan(plan: PowerPlan) -> Result<()> {
    let output = Command::new("powercfg")
        .args(["/setactive", plan.guid()])
        .output()
        .map_err(|e| PieuvreError::Io(e))?;
    
    if !output.status.success() {
        // Le plan n'existe peut-être pas, essayer de le créer
        if plan == PowerPlan::UltimatePerformance {
            create_ultimate_performance_plan()?;
            
            // Réessayer
            let _ = Command::new("powercfg")
                .args(["/setactive", plan.guid()])
                .output();
        }
    }
    
    tracing::info!("Plan d'alimentation activé: {}", plan.name());
    Ok(())
}

/// Crée le plan Ultimate Performance s'il n'existe pas
fn create_ultimate_performance_plan() -> Result<()> {
    let output = Command::new("powercfg")
        .args(["-duplicatescheme", "e9a42b02-d5df-448d-aa00-03f14749eb61"])
        .output()
        .map_err(|e| PieuvreError::Io(e))?;
    
    if !output.status.success() {
        // Créer depuis High Performance
        let _ = Command::new("powercfg")
            .args(["-duplicatescheme", "8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c", "e9a42b02-d5df-448d-aa00-03f14749eb61"])
            .output();
    }
    
    Ok(())
}

/// Configure les paramètres d'alimentation spécifiques
pub fn configure_power_settings(
    usb_selective_suspend: bool,
    pci_aspm: bool,
    processor_min: u8,
    processor_max: u8,
) -> Result<()> {
    // USB Selective Suspend
    // GUID: 2a737441-1930-4402-8d77-b2bebba308a3
    // 0 = Disabled, 1 = Enabled
    let usb_value = if usb_selective_suspend { "1" } else { "0" };
    let _ = Command::new("powercfg")
        .args(["/setacvalueindex", "scheme_current", "2a737441-1930-4402-8d77-b2bebba308a3", "48e6b7a6-50f5-4782-a5d4-53bb8f07e226", usb_value])
        .output();
    
    // PCI Express Link State Power Management
    // 0 = Off, 1 = Moderate, 2 = Maximum
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
    
    tracing::info!("Paramètres power configurés: USB_SS={}, ASPM={}, CPU={}%-{}%", 
        usb_selective_suspend, pci_aspm, processor_min, processor_max);
    
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
