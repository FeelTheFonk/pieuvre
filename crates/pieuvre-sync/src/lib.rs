//! pieuvre Sync Engine
//!
//! Synchronization module: applying optimizations.

pub mod ai;
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
pub mod registry;
pub mod rollback;
pub mod scheduled_tasks;
pub mod security;
pub mod services;
pub mod timer;
pub mod widgets;
pub mod windows_update;

#[cfg(test)]
mod tests;

use crate::operation::{RegistryDwordOperation, ServiceOperation, SyncOperation};
use pieuvre_common::Result;
use tracing::instrument;

/// Applies an optimization profile
pub async fn apply_profile(profile_name: &str, dry_run: bool) -> Result<()> {
    tracing::info!("Applying profile: {} (dry_run: {})", profile_name, dry_run);

    // Load TOML profile
    let mut profile_path =
        std::path::PathBuf::from(format!("config/profiles/{}.toml", profile_name));

    // If we are in a sub-crate (e.g. tests), go up one level
    if !profile_path.exists() {
        let alt_path =
            std::path::PathBuf::from(format!("../../config/profiles/{}.toml", profile_name));
        if alt_path.exists() {
            profile_path = alt_path;
        }
    }

    let content = match std::fs::read_to_string(&profile_path) {
        Ok(c) => c,
        Err(e) if dry_run && e.kind() == std::io::ErrorKind::NotFound => {
            tracing::warn!(
                "Profile {} not found in dry-run mode, continuing...",
                profile_name
            );
            return Ok(());
        }
        Err(e) => {
            return Err(pieuvre_common::PieuvreError::Internal(format!(
                "Error reading profile {} ({:?}): {}",
                profile_name, profile_path, e
            )));
        }
    };

    let profile: pieuvre_common::Profile = ::toml::from_str(&content).map_err(|e| {
        pieuvre_common::PieuvreError::Internal(format!(
            "Error parsing profile {}: {}",
            profile_name, e
        ))
    })?;

    if dry_run {
        tracing::info!(
            "[DRY-RUN] Simulating application of profile {}",
            profile.name
        );
    }

    // 0. Hardware probing
    let hw = tokio::task::spawn_blocking(pieuvre_audit::hardware::probe_hardware)
        .await
        .map_err(|e| pieuvre_common::PieuvreError::Internal(e.to_string()))??;

    let mut operations: Vec<Box<dyn SyncOperation>> = Vec::new();

    // 1. Timer & Scheduler
    if let Some(timer) = &profile.timer {
        operations.push(Box::new(RegistryDwordOperation {
            key: r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel".to_string(),
            value: "GlobalTimerResolutionRequests".to_string(),
            target_data: if timer.force_high_precision { 1 } else { 0 },
        }));
    }

    if let Some(scheduler) = &profile.scheduler {
        operations.push(Box::new(RegistryDwordOperation {
            key: r"SYSTEM\CurrentControlSet\Control\PriorityControl".to_string(),
            value: "Win32PrioritySeparation".to_string(),
            target_data: scheduler.win32_priority_separation,
        }));
    }

    // 2. Services
    if let Some(services) = &profile.services {
        for name in &services.disable {
            operations.push(Box::new(ServiceOperation {
                name: name.clone(),
                target_start_type: 4,
            }));
        }
        for name in &services.manual {
            operations.push(Box::new(ServiceOperation {
                name: name.clone(),
                target_start_type: 3,
            }));
        }
    }

    // 3. Telemetry & Registry
    if let Some(telemetry) = &profile.telemetry {
        operations.push(Box::new(RegistryDwordOperation {
            key: r"SOFTWARE\Policies\Microsoft\Windows\DataCollection".to_string(),
            value: "AllowTelemetry".to_string(),
            target_data: telemetry.level,
        }));
    }

    if let Some(reg) = &profile.registry {
        let mut add_reg = |key: &str, value: &str, val: Option<bool>| {
            if let Some(v) = val {
                operations.push(Box::new(RegistryDwordOperation {
                    key: key.to_string(),
                    value: value.to_string(),
                    target_data: if v { 1 } else { 0 },
                }));
            }
        };

        add_reg(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\AdvertisingInfo",
            "Enabled",
            reg.advertising_id,
        );
        add_reg(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location",
            "Value",
            reg.location_tracking,
        );
        add_reg(
            r"SOFTWARE\Policies\Microsoft\Windows\System",
            "PublishUserActivities",
            reg.activity_history,
        );
        add_reg(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Search",
            "CanCortanaBeEnabled",
            reg.cortana,
        );
        add_reg(
            r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
            "DisableSearchBoxSuggestions",
            reg.web_search.map(|v| !v),
        );
        add_reg(
            r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
            "DisableTailoredExperiences",
            reg.tailored_experiences.map(|v| !v),
        );
        add_reg(
            r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
            "AllowDiagnosticDataViewer",
            reg.diagnostic_data_viewer,
        );
        add_reg(
            r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
            "DisableInventory",
            reg.app_launch_tracking.map(|v| !v),
        );
        add_reg(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
            "SystemPaneSuggestionsEnabled",
            reg.suggested_content,
        );
        add_reg(
            r"SOFTWARE\Policies\Microsoft\Windows\System",
            "EnableActivityFeed",
            reg.timeline,
        );
        add_reg(
            r"SOFTWARE\Microsoft\InputPersonalization",
            "RestrictImplicitTextCollection",
            reg.input_personalization.map(|v| !v),
        );
        add_reg(
            r"SOFTWARE\Microsoft\InputPersonalization",
            "RestrictImplicitInkCollection",
            reg.input_personalization.map(|v| !v),
        );

        if let Some(freq) = reg.feedback_frequency {
            operations.push(Box::new(RegistryDwordOperation {
                key: r"SOFTWARE\Policies\Microsoft\Windows\DataCollection".to_string(),
                value: "DoNotShowFeedbackNotifications".to_string(),
                target_data: if freq == 0 { 1 } else { 0 },
            }));
        }
    }

    // 4. MSI
    if let Some(msi) = &profile.msi {
        operations.push(Box::new(crate::operation::MsiOperation {
            devices: msi.enable_for.clone(),
            priority: msi.priority.clone(),
        }));
    }

    // 5. AppX
    if let Some(appx) = &profile.appx {
        operations.push(Box::new(crate::operation::AppxOperation {
            packages_to_remove: appx.remove.clone(),
        }));
    }

    // 6. GPU
    if let Some(gpu) = &profile.gpu {
        operations.push(Box::new(RegistryDwordOperation {
            key: r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers".to_string(),
            value: "HwSchMode".to_string(),
            target_data: if gpu.hardware_accelerated_gpu_scheduling {
                2
            } else {
                1
            },
        }));
    }

    // 7. AI (Recall & CoPilot) - SOTA 2026
    operations.push(Box::new(crate::ai::DisableRecallOperation));
    operations.push(Box::new(crate::ai::DisableCoPilotOperation));

    // 8. DNS SOTA 2026
    operations.push(Box::new(crate::dns::ConfigureDohOperation {
        provider: crate::dns::DNSProvider::Cloudflare,
    }));
    operations.push(Box::new(crate::dns::FlushDnsOperation));

    // 9. Cleanup SOTA 2026
    operations.push(Box::new(crate::cleanup::CleanupTempOperation));
    operations.push(Box::new(crate::cleanup::CleanupWinSxSOperation));
    operations.push(Box::new(crate::cleanup::CleanupEdgeCacheOperation));

    // --- DYNAMIC ADAPTATION ---
    // A. Optimisation NVIDIA
    if hw.gpu.iter().any(|g| g.vendor == "NVIDIA") {
        tracing::info!("GPU NVIDIA détecté : application des tweaks spécifiques");
        operations.push(Box::new(RegistryDwordOperation {
            key: r"SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}\0000".to_string(),
            value: "PowerMizerEnable".to_string(),
            target_data: 1,
        }));
    }

    // B. Hybrid CPU Optimization
    if hw.cpu.is_hybrid {
        tracing::info!("Hybrid CPU detected: optimizing P-Cores scheduling");
        operations.push(Box::new(RegistryDwordOperation {
            key: r"SYSTEM\CurrentControlSet\Control\Session Manager\Kernel".to_string(),
            value: "DistributeTimers".to_string(),
            target_data: 1,
        }));
    }

    // Execute operations
    let mut set = tokio::task::JoinSet::new();
    for op in operations {
        if dry_run {
            tracing::info!("[DRY-RUN] Operation: {}", op.name());
            continue;
        }
        set.spawn(async move { op.apply().await });
    }

    let mut all_changes = Vec::new();
    while let Some(res) = set.join_next().await {
        if let Ok(Ok(changes)) = res {
            all_changes.extend(changes);
        }
    }

    // 4. Power plan
    if let Some(power_cfg) = &profile.power {
        if !dry_run {
            let plan_name = power_cfg.plan.clone();
            let _ = tokio::task::spawn_blocking(move || match plan_name.as_str() {
                "ultimate_performance" => crate::power::apply_gaming_power_config(),
                "high_performance" => {
                    crate::power::set_power_plan(crate::power::PowerPlan::HighPerformance)
                }
                _ => crate::power::set_power_plan(crate::power::PowerPlan::Balanced),
            })
            .await;
        }
    }

    // Hardening
    if !dry_run {
        tracing::info!("Applying Hardening...");
        for key in crate::hardening::CRITICAL_KEYS {
            let _ = crate::hardening::lock_registry_key(key);
        }
    }

    tracing::info!(
        "Profile {} applied successfully ({} changes)",
        profile.name,
        all_changes.len()
    );
    Ok(())
}

#[instrument]
pub async fn reset_to_defaults() -> Result<()> {
    tracing::info!("Resetting to defaults...");

    use crate::operation::{RegistryDwordOperation, ServiceOperation, SyncOperation};
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
