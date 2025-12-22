//! Power Manager SOTA 2026
//!
//! Gestion des plans d'alimentation et paramètres énergétiques.
//! Utilise les APIs Windows natives (PowerGetActiveScheme, PowerSetActiveScheme).

use pieuvre_common::{PieuvreError, Result};
use windows::core::GUID;
use windows::Win32::Foundation::{LocalFree, HLOCAL};
use windows::Win32::System::Power::{
    PowerGetActiveScheme, PowerSetActiveScheme, PowerWriteACValueIndex,
};

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

    GUID {
        data1,
        data2,
        data3,
        data4,
    }
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
            return Err(PieuvreError::Unsupported(
                "PowerGetActiveScheme failed".to_string(),
            ));
        }

        if scheme_guid.is_null() {
            return Err(PieuvreError::Unsupported(
                "No active power scheme".to_string(),
            ));
        }

        let guid = *scheme_guid;
        let guid_str = guid_to_string(&guid);

        // Libérer la mémoire allouée par Windows
        let _ = LocalFree(Some(HLOCAL(scheme_guid as *mut std::ffi::c_void)));

        // Convertir en nom lisible si connu
        let name = match guid_str.as_str() {
            s if s == PowerPlan::PowerSaver.guid() => PowerPlan::PowerSaver.name().to_string(),
            s if s == PowerPlan::Balanced.guid() => PowerPlan::Balanced.name().to_string(),
            s if s == PowerPlan::HighPerformance.guid() => {
                PowerPlan::HighPerformance.name().to_string()
            }
            s if s == PowerPlan::UltimatePerformance.guid() => {
                PowerPlan::UltimatePerformance.name().to_string()
            }
            "bd0b9fdc-5b4f-49d2-b2d0-76c179101054" => "Bitsum Highest Performance".to_string(),
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
                    tracing::warn!(
                        "Ultimate Performance non disponible, utilisation High Performance"
                    );
                }
            } else {
                return Err(PieuvreError::Unsupported(format!(
                    "PowerSetActiveScheme failed for {}",
                    plan.name()
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
            .args([
                "-duplicatescheme",
                "8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c",
                "e9a42b02-d5df-448d-aa00-03f14749eb61",
            ])
            .output();
    }

    Ok(())
}

/// Configure les paramètres d'alimentation spécifiques via API native
pub fn configure_power_settings(
    usb_selective_suspend: bool,
    pci_aspm: bool,
    processor_min: u8,
    processor_max: u8,
) -> Result<()> {
    unsafe {
        let mut scheme_guid_ptr: *mut GUID = std::ptr::null_mut();
        if PowerGetActiveScheme(None, &mut scheme_guid_ptr).is_err() {
            return Err(PieuvreError::Unsupported(
                "Cannot get active power scheme".to_string(),
            ));
        }
        let scheme_guid = *scheme_guid_ptr;

        // GUIDs pour les paramètres (Source: Microsoft Documentation)
        let subgroup_usb = parse_guid("2a737441-1930-4402-8d77-b2bebba308a3");
        let setting_usb_suspend = parse_guid("48e6b7a6-50f5-4782-a5d4-53bb8f07e226");

        let subgroup_pci = parse_guid("501a4d13-42af-4429-9fd1-a8218c268e20");
        let setting_pci_aspm = parse_guid("ee12f906-d277-404b-b6da-e5fa1a576df5");

        let subgroup_cpu = parse_guid("54533251-82be-4824-96c1-47b60b740d00");
        let setting_cpu_min = parse_guid("893dee8e-2bef-41e0-89c6-b55d0929964c");
        let setting_cpu_max = parse_guid("bc5038f7-23e0-4960-96da-33abaf5935ec");

        // Appliquer les valeurs
        let _ = PowerWriteACValueIndex(
            None,
            &scheme_guid,
            Some(&subgroup_usb),
            Some(&setting_usb_suspend),
            if usb_selective_suspend { 1 } else { 0 },
        );
        let _ = PowerWriteACValueIndex(
            None,
            &scheme_guid,
            Some(&subgroup_pci),
            Some(&setting_pci_aspm),
            if pci_aspm { 1 } else { 0 },
        );
        let _ = PowerWriteACValueIndex(
            None,
            &scheme_guid,
            Some(&subgroup_cpu),
            Some(&setting_cpu_min),
            processor_min as u32,
        );
        let _ = PowerWriteACValueIndex(
            None,
            &scheme_guid,
            Some(&subgroup_cpu),
            Some(&setting_cpu_max),
            processor_max as u32,
        );

        // Appliquer les changements
        let _ = PowerSetActiveScheme(None, Some(&scheme_guid));

        let _ = LocalFree(Some(HLOCAL(scheme_guid_ptr as *mut std::ffi::c_void)));

        tracing::info!(
            usb_suspend = usb_selective_suspend,
            pci_aspm = pci_aspm,
            cpu_min = processor_min,
            cpu_max = processor_max,
            "Paramètres power configurés via API native"
        );

        Ok(())
    }
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
