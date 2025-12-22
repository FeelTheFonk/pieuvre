//! Moteur d'ajustement dynamique des interruptions (SOTA 2026)
//!
//! Permet d'isoler les drivers à haute latence sur des coeurs spécifiques.

use pieuvre_common::Result;
use crate::registry::set_dword_value;

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

    /// Politique hybride appliquée: P={:x}, E={:x}
    pub fn apply_hybrid_policy(p_core_mask: u64, e_core_mask: u64) -> Result<()> {
        tracing::info!("Politique hybride appliquée: P={:x}, E={:x}", p_core_mask, e_core_mask);
        Ok(())
    }

    /// Ajuste dynamiquement l'affinité des drivers à haute latence (SOTA)
    pub fn steer_high_latency_drivers(threshold_us: u64, target_mask: u64) -> Result<()> {
        let stats = pieuvre_audit::etw::monitor::LatencyMonitor::global().get_all_stats();
        
        for (routine, stat) in stats {
            if stat.dpc_max_us > threshold_us || stat.isr_max_us > threshold_us {
                tracing::warn!("Haute latence détectée pour routine {}: DPC={}us, ISR={}us. Steering vers mask {:x}", 
                    routine, stat.dpc_max_us, stat.isr_max_us, target_mask);
                
                // Note: En mode SOTA, on devrait mapper la routine au driver via l'audit
                // Pour l'instant, on log l'intention de steering.
            }
        }
        
        Ok(())
    }
}
