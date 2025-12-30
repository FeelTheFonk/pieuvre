//! pieuvre Sync Engine
//!
//! Synchronization module: applying optimizations.

pub mod cleanup;
pub mod dns;
pub mod interrupts;
pub mod sentinel {
    pub mod monitor;
}
pub mod appx;
pub mod context_menu;
pub mod cpu;
pub mod dpc;
pub mod memory;

pub mod edge;
pub mod explorer;
pub mod firewall;
pub mod game_mode;
pub mod hardening;
pub mod hosts;
pub mod msi;
pub mod network;
pub mod onedrive;
pub mod operation;
pub mod power;
pub mod privacy_o_o;
pub mod registry;
pub mod rollback;
pub mod scheduled_tasks;
pub mod security;
pub mod services;
pub mod timer;
pub mod windows_update;

#[cfg(test)]
mod tests;

use crate::operation::SyncOperation;
use pieuvre_common::Result;
use tracing::instrument;

// apply_profile has been removed in v0.5.0 in favor of granular interactive execution.

#[instrument]
pub async fn reset_to_defaults() -> Result<()> {
    tracing::info!("Resetting to defaults...");

    use crate::operation::{RegistryDwordOperation, ServiceOperation};
    use tokio::task::JoinSet;

    let operations: Vec<Box<dyn SyncOperation>> = vec![
        // 1. Services in automatic mode (or manual depending on service)
        Box::new(ServiceOperation {
            name: "DiagTrack".to_string(),
            target_start_type: 2,
        }),
        Box::new(ServiceOperation {
            name: "dmwappushservice".to_string(),
            target_start_type: 3,
        }),
        Box::new(ServiceOperation {
            name: "WerSvc".to_string(),
            target_start_type: 3,
        }),
        Box::new(ServiceOperation {
            name: "SysMain".to_string(),
            target_start_type: 2,
        }),
        Box::new(ServiceOperation {
            name: "WSearch".to_string(),
            target_start_type: 2,
        }),
        // 2. Default registry
        Box::new(RegistryDwordOperation {
            key: r"SYSTEM\CurrentControlSet\Control\PriorityControl".to_string(),
            value: "Win32PrioritySeparation".to_string(),
            target_data: 0x2,
        }),
    ];

    let mut set = JoinSet::new();
    for op in operations {
        set.spawn(async move { op.apply().await });
    }

    let mut all_changes = Vec::new();
    while let Some(res) = set.join_next().await {
        if let Ok(Ok(changes)) = res {
            all_changes.extend(changes);
        }
    }

    // 3. Power plan Balanced
    let _ = tokio::task::spawn_blocking(|| power::set_power_plan(power::PowerPlan::Balanced)).await;

    tracing::info!("Reset completed ({} changes)", all_changes.len());
    Ok(())
}
