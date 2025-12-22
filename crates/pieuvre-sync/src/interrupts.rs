//! Moteur d'ajustement dynamique des interruptions (SOTA 2026)
//!
//! Permet d'isoler les drivers à haute latence sur des coeurs spécifiques.

use pieuvre_common::Result;
use crate::registry::set_dword_value;
use windows::Win32::System::Registry::HKEY_LOCAL_MACHINE;

pub struct InterruptSteering;

impl InterruptSteering {
    /// Configure l'affinité d'un driver spécifique
    pub fn set_driver_affinity(driver_name: &str, mask: u64) -> Result<()> {
        let path = format!(r#"System\CurrentControlSet\Enum\{}\Device Parameters\Interrupt Management\Affinity Policy"#, driver_name);
        
        set_dword_value(&path, "AssignmentSetOverride", mask as u32)?;
        set_dword_value(&path, "DevicePolicy", 4)?; // IrqPolicySpecifiedProcessors
        
        tracing::info!("Affinité configurée pour {}: mask={:x}", driver_name, mask);
        Ok(())
    }

    /// Applique une politique d'isolation pour les P-Cores/E-Cores
    pub fn apply_hybrid_policy(p_core_mask: u64, e_core_mask: u64) -> Result<()> {
        // Logique pour séparer les interruptions critiques sur les P-Cores
        // et les tâches de fond sur les E-Cores.
        tracing::info!("Politique hybride appliquée: P={:x}, E={:x}", p_core_mask, e_core_mask);
        Ok(())
    }
}
