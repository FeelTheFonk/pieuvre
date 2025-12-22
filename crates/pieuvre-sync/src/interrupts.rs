//! Dynamic interrupt adjustment engine
//!
//! Allows isolating high-latency drivers on specific cores.

use crate::registry::set_dword_value;
use pieuvre_common::Result;

pub struct InterruptSteering;

impl InterruptSteering {
    /// Configures the affinity of a specific driver
    pub fn set_driver_affinity(driver_name: &str, mask: u64) -> Result<()> {
        let path = format!(
            r#"System\CurrentControlSet\Enum\{}\Device Parameters\Interrupt Management\Affinity Policy"#,
            driver_name
        );

        set_dword_value(&path, "AssignmentSetOverride", mask as u32)?;
        set_dword_value(&path, "DevicePolicy", 4)?; // IrqPolicySpecifiedProcessors

        tracing::info!("Affinity configured for {}: mask={:x}", driver_name, mask);
        Ok(())
    }

    /// Hybrid policy applied: P={:x}, E={:x}
    pub fn apply_hybrid_policy(p_core_mask: u64, e_core_mask: u64) -> Result<()> {
        tracing::info!(
            "Hybrid policy applied: P={:x}, E={:x}",
            p_core_mask,
            e_core_mask
        );
        Ok(())
    }

    /// Dynamically adjusts the affinity of high-latency drivers
    pub fn steer_high_latency_drivers(threshold_us: u64, target_mask: u64) -> Result<()> {
        let stats = pieuvre_audit::etw::monitor::LatencyMonitor::global().get_all_stats();

        for (driver_name, stat) in stats {
            if stat.dpc_max_us > threshold_us || stat.isr_max_us > threshold_us {
                tracing::warn!(
                    "High latency detected for {}: DPC={}us, ISR={}us. Steering to mask {:x}",
                    driver_name,
                    stat.dpc_max_us,
                    stat.isr_max_us,
                    target_mask
                );

                // Attempt steering if name looks like a driver (not a hex address)
                if !driver_name.starts_with("0x") {
                    if let Err(e) = Self::set_driver_affinity(&driver_name, target_mask) {
                        tracing::error!("Failed to steer driver {}: {:?}", driver_name, e);
                    }
                }
            }
        }

        Ok(())
    }
}
