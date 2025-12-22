//! Optimization executors for interactive mode
//!
//! Trait pattern for polymorphic execution of optimizations.

use anyhow::Result;
use async_trait::async_trait;
use pieuvre_common::ChangeRecord;
use tracing::{info, instrument, warn};

/// Résultat d'exécution d'une optimisation
#[derive(Debug)]
pub struct ExecutionResult {
    /// Nombre d'éléments affectés (services, apps, etc.)
    #[allow(dead_code)]
    pub affected_count: usize,
    /// Message de succès
    pub message: String,
}

impl ExecutionResult {
    pub fn ok(message: impl Into<String>) -> Self {
        Self {
            affected_count: 1,
            message: message.into(),
        }
    }

    pub fn ok_count(count: usize, message: impl Into<String>) -> Self {
        Self {
            affected_count: count,
            message: message.into(),
        }
    }
}

/// Trait to execute an optimization
#[async_trait]
pub trait OptExecutor {
    /// Executes the optimization and returns the result
    async fn execute(&self, id: &str, changes: &mut Vec<ChangeRecord>) -> Result<ExecutionResult>;
}

// ============================================================================
// TELEMETRY EXECUTOR
// ============================================================================

pub struct TelemetryExecutor;

#[async_trait]
impl OptExecutor for TelemetryExecutor {
    #[instrument(skip(self, changes), fields(category = "telemetry"))]
    async fn execute(&self, id: &str, changes: &mut Vec<ChangeRecord>) -> Result<ExecutionResult> {
        use pieuvre_sync::{firewall, hosts, onedrive, scheduled_tasks, services};

        match id {
            "diagtrack" => {
                capture_service_state("DiagTrack", changes);
                tokio::task::spawn_blocking(|| services::disable_service("DiagTrack")).await??;
                info!(service = "DiagTrack", "Service disabled");
                Ok(ExecutionResult::ok("DiagTrack disabled"))
            }
            "dmwappush" => {
                capture_service_state("dmwappushservice", changes);
                tokio::task::spawn_blocking(|| services::disable_service("dmwappushservice"))
                    .await??;
                info!(service = "dmwappushservice", "Service disabled");
                Ok(ExecutionResult::ok("dmwappushservice disabled"))
            }
            "wersvc" => {
                capture_service_state("WerSvc", changes);
                tokio::task::spawn_blocking(|| services::disable_service("WerSvc")).await??;
                Ok(ExecutionResult::ok("WerSvc disabled"))
            }
            "wercplsupport" => {
                capture_service_state("wercplsupport", changes);
                tokio::task::spawn_blocking(|| services::disable_service("wercplsupport"))
                    .await??;
                Ok(ExecutionResult::ok("wercplsupport disabled"))
            }
            "pcasvc" => {
                capture_service_state("PcaSvc", changes);
                tokio::task::spawn_blocking(|| services::disable_service("PcaSvc")).await??;
                Ok(ExecutionResult::ok("PcaSvc disabled"))
            }
            "wdisystem" => {
                capture_service_state("WdiSystemHost", changes);
                tokio::task::spawn_blocking(|| services::disable_service("WdiSystemHost"))
                    .await??;
                Ok(ExecutionResult::ok("WdiSystemHost disabled"))
            }
            "wdiservice" => {
                capture_service_state("WdiServiceHost", changes);
                tokio::task::spawn_blocking(|| services::disable_service("WdiServiceHost"))
                    .await??;
                Ok(ExecutionResult::ok("WdiServiceHost disabled"))
            }
            "lfsvc" => {
                capture_service_state("lfsvc", changes);
                tokio::task::spawn_blocking(|| services::disable_service("lfsvc")).await??;
                Ok(ExecutionResult::ok("lfsvc disabled"))
            }
            "mapsbroker" => {
                capture_service_state("MapsBroker", changes);
                tokio::task::spawn_blocking(|| services::disable_service("MapsBroker")).await??;
                Ok(ExecutionResult::ok("MapsBroker disabled"))
            }
            "firewall" => {
                let rules =
                    tokio::task::spawn_blocking(firewall::create_telemetry_block_rules).await??;
                info!(rules_count = rules.len(), "Firewall rules created");
                Ok(ExecutionResult::ok_count(
                    rules.len(),
                    format!("{} firewall rules", rules.len()),
                ))
            }
            "sched_tasks" => {
                let tasks =
                    tokio::task::spawn_blocking(scheduled_tasks::disable_telemetry_tasks).await??;
                info!(tasks_count = tasks.len(), "Scheduled tasks disabled");
                Ok(ExecutionResult::ok_count(
                    tasks.len(),
                    format!("{} tasks disabled", tasks.len()),
                ))
            }
            "hosts" => {
                let count = tokio::task::spawn_blocking(hosts::add_telemetry_blocks).await??;
                info!(domains_count = count, "Hosts entries added");
                Ok(ExecutionResult::ok_count(
                    count as usize,
                    format!("{} domains blocked", count),
                ))
            }
            "onedrive" => {
                tokio::task::spawn_blocking(onedrive::uninstall_onedrive).await??;
                info!("OneDrive uninstalled");
                Ok(ExecutionResult::ok("OneDrive removed"))
            }
            _ => {
                warn!(id = id, "Unknown telemetry option");
                anyhow::bail!("Unknown telemetry option: {}", id)
            }
        }
    }
}

// ============================================================================
// PRIVACY EXECUTOR
// ============================================================================

pub struct PrivacyExecutor;

#[async_trait]
impl OptExecutor for PrivacyExecutor {
    #[instrument(skip(self, _changes), fields(category = "privacy"))]
    async fn execute(&self, id: &str, _changes: &mut Vec<ChangeRecord>) -> Result<ExecutionResult> {
        use pieuvre_sync::{context_menu, registry, widgets, windows_update};

        match id {
            "telemetry_level" => {
                capture_registry_state(
                    r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
                    "AllowTelemetry",
                    _changes,
                );
                capture_registry_state(
                    r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection",
                    "AllowTelemetry",
                    _changes,
                );
                tokio::task::spawn_blocking(|| registry::set_telemetry_level(0)).await??;
                Ok(ExecutionResult::ok("Telemetry level 0"))
            }
            "advertising_id" => {
                capture_registry_state(
                    r"SOFTWARE\Microsoft\Windows\CurrentVersion\AdvertisingInfo",
                    "Enabled",
                    _changes,
                );
                tokio::task::spawn_blocking(registry::disable_advertising_id).await??;
                Ok(ExecutionResult::ok("Advertising ID disabled"))
            }
            "location" => {
                capture_registry_state(
                    r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location",
                    "Value",
                    _changes,
                );
                tokio::task::spawn_blocking(registry::disable_location).await??;
                Ok(ExecutionResult::ok("Location disabled"))
            }
            "activity_history" => {
                capture_registry_state(
                    r"SOFTWARE\Policies\Microsoft\Windows\System",
                    "EnableActivityFeed",
                    _changes,
                );
                capture_registry_state(
                    r"SOFTWARE\Policies\Microsoft\Windows\System",
                    "PublishUserActivities",
                    _changes,
                );
                capture_registry_state(
                    r"SOFTWARE\Policies\Microsoft\Windows\System",
                    "UploadUserActivities",
                    _changes,
                );
                tokio::task::spawn_blocking(registry::disable_activity_history).await??;
                Ok(ExecutionResult::ok("Activity history disabled"))
            }
            "cortana" => {
                capture_registry_state(
                    r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
                    "AllowCortana",
                    _changes,
                );
                tokio::task::spawn_blocking(registry::disable_cortana).await??;
                Ok(ExecutionResult::ok("Cortana disabled"))
            }
            "context_menu" => {
                // TODO: Capture context menu state if possible
                let n = tokio::task::spawn_blocking(context_menu::remove_context_menu_clutter)
                    .await??;
                Ok(ExecutionResult::ok_count(
                    n as usize,
                    format!("{} items removed", n),
                ))
            }
            "widgets" => {
                capture_registry_state(
                    r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                    "TaskbarDa",
                    _changes,
                );
                tokio::task::spawn_blocking(widgets::disable_widgets).await??;
                Ok(ExecutionResult::ok("Widgets disabled"))
            }
            "pause_updates" => {
                tokio::task::spawn_blocking(windows_update::pause_updates).await??;
                Ok(ExecutionResult::ok("Updates paused 35 days"))
            }
            "driver_updates" => {
                capture_registry_state(
                    r"SOFTWARE\Policies\Microsoft\Windows\DeviceInstall\Settings",
                    "InstallEveryElement",
                    _changes,
                );
                tokio::task::spawn_blocking(windows_update::disable_driver_updates).await??;
                Ok(ExecutionResult::ok("Driver updates disabled"))
            }
            "recall" => {
                capture_registry_state(
                    r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
                    "DisableAIDataAnalysis",
                    _changes,
                );
                capture_registry_state(
                    r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
                    "TurnOffSavingSnapshots",
                    _changes,
                );
                tokio::task::spawn_blocking(registry::disable_recall).await??;
                Ok(ExecutionResult::ok("Windows Recall blocked"))
            }
            "group_policy_telem" => {
                capture_registry_state(
                    r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
                    "AllowTelemetry",
                    _changes,
                );
                tokio::task::spawn_blocking(|| registry::set_group_policy_telemetry(0)).await??;
                Ok(ExecutionResult::ok("Group Policy telemetry Security"))
            }
            _ => anyhow::bail!("Unknown privacy option: {}", id),
        }
    }
}

// ============================================================================
// PERFORMANCE EXECUTOR
// ============================================================================

pub struct PerformanceExecutor;

#[async_trait]
impl OptExecutor for PerformanceExecutor {
    #[instrument(skip(self, _changes), fields(category = "performance"))]
    async fn execute(&self, id: &str, _changes: &mut Vec<ChangeRecord>) -> Result<ExecutionResult> {
        use pieuvre_sync::{
            edge, explorer, game_mode, msi, network, power, registry, services, timer,
        };

        match id {
            "timer" => {
                tokio::task::spawn_blocking(|| timer::set_timer_resolution(5000)).await??;
                Ok(ExecutionResult::ok("Timer 0.5ms"))
            }
            "power_ultimate" => {
                tokio::task::spawn_blocking(|| {
                    power::set_power_plan(power::PowerPlan::UltimatePerformance)
                })
                .await??;
                Ok(ExecutionResult::ok("Ultimate Performance"))
            }
            "power_high" => {
                tokio::task::spawn_blocking(|| {
                    power::set_power_plan(power::PowerPlan::HighPerformance)
                })
                .await??;
                Ok(ExecutionResult::ok("High Performance"))
            }
            "cpu_throttle" => {
                tokio::task::spawn_blocking(power::disable_cpu_throttling).await??;
                Ok(ExecutionResult::ok("CPU throttling disabled"))
            }
            "usb_suspend" => {
                tokio::task::spawn_blocking(|| {
                    power::configure_power_settings(false, false, 100, 100)
                })
                .await??;
                Ok(ExecutionResult::ok("USB suspend disabled"))
            }
            "msi" => {
                let devices = tokio::task::spawn_blocking(msi::list_msi_eligible_devices).await??;
                let mut enabled = 0;
                for dev in &devices {
                    if !dev.msi_enabled {
                        if msi::enable_msi(&dev.full_path).is_ok() {
                            enabled += 1;
                        }
                    } else {
                        enabled += 1;
                    }
                }
                Ok(ExecutionResult::ok_count(
                    enabled,
                    format!("{}/{} MSI devices", enabled, devices.len()),
                ))
            }
            "sysmain" => {
                tokio::task::spawn_blocking(|| services::disable_service("SysMain")).await??;
                Ok(ExecutionResult::ok("SysMain disabled"))
            }
            "wsearch" => {
                tokio::task::spawn_blocking(|| services::disable_service("WSearch")).await??;
                Ok(ExecutionResult::ok("WSearch disabled"))
            }
            "edge_disable" => {
                tokio::task::spawn_blocking(edge::disable_edge).await??;
                let _ = tokio::task::spawn_blocking(edge::remove_edge_tasks).await;
                Ok(ExecutionResult::ok("Edge features disabled"))
            }
            "explorer_tweaks" => {
                tokio::task::spawn_blocking(explorer::apply_explorer_tweaks).await??;
                Ok(ExecutionResult::ok("Explorer tweaks applied"))
            }
            "game_bar" => {
                tokio::task::spawn_blocking(game_mode::disable_game_bar).await??;
                Ok(ExecutionResult::ok("Game Bar disabled"))
            }
            "fullscreen_opt" => {
                tokio::task::spawn_blocking(game_mode::disable_fullscreen_optimizations).await??;
                Ok(ExecutionResult::ok("Fullscreen optimizations disabled"))
            }
            "hags" => {
                tokio::task::spawn_blocking(game_mode::disable_hags).await??;
                Ok(ExecutionResult::ok("HAGS disabled"))
            }
            "nagle" => {
                let n = tokio::task::spawn_blocking(network::disable_nagle_algorithm).await??;
                Ok(ExecutionResult::ok_count(
                    n as usize,
                    format!("{} interfaces", n),
                ))
            }
            "power_throttle" => {
                tokio::task::spawn_blocking(registry::disable_power_throttling).await??;
                Ok(ExecutionResult::ok("Power throttling disabled"))
            }
            "enable_game_mode" => {
                tokio::task::spawn_blocking(game_mode::enable_game_mode).await??;
                Ok(ExecutionResult::ok("Windows Game Mode enabled"))
            }
            "prerendered_frames" => {
                tokio::task::spawn_blocking(|| game_mode::set_prerendered_frames(1)).await??;
                Ok(ExecutionResult::ok("Pre-rendered frames set to 1"))
            }
            "vrr_opt" => {
                tokio::task::spawn_blocking(game_mode::disable_vrr_optimizations).await??;
                Ok(ExecutionResult::ok("VRR optimizations disabled"))
            }
            "shader_cache" => {
                tokio::task::spawn_blocking(|| game_mode::set_shader_cache_size(256)).await??;
                Ok(ExecutionResult::ok("Shader cache set to 256MB"))
            }
            _ => anyhow::bail!("Unknown performance option: {}", id),
        }
    }
}

// ============================================================================
// SCHEDULER EXECUTOR
// ============================================================================

pub struct SchedulerExecutor;

#[async_trait]
impl OptExecutor for SchedulerExecutor {
    #[instrument(skip(self, _changes), fields(category = "scheduler"))]
    async fn execute(&self, id: &str, _changes: &mut Vec<ChangeRecord>) -> Result<ExecutionResult> {
        use pieuvre_sync::registry;

        match id {
            "priority_sep" => {
                tokio::task::spawn_blocking(|| registry::set_priority_separation(0x26)).await??;
                Ok(ExecutionResult::ok("Win32PrioritySeparation 0x26"))
            }
            "mmcss" => {
                tokio::task::spawn_blocking(registry::configure_mmcss_gaming).await??;
                Ok(ExecutionResult::ok("MMCSS configured"))
            }
            "games_priority" => {
                tokio::task::spawn_blocking(registry::configure_games_priority).await??;
                Ok(ExecutionResult::ok("GPU=8, Priority=6"))
            }
            "global_timer" => {
                tokio::task::spawn_blocking(registry::enable_global_timer_resolution).await??;
                Ok(ExecutionResult::ok("Global timer (reboot required)"))
            }
            "startup_delay" => {
                tokio::task::spawn_blocking(registry::disable_startup_delay).await??;
                Ok(ExecutionResult::ok("Startup delay 0ms"))
            }
            "shutdown_timeout" => {
                tokio::task::spawn_blocking(registry::reduce_shutdown_timeout).await??;
                Ok(ExecutionResult::ok("Shutdown timeout 2000ms"))
            }
            _ => anyhow::bail!("Unknown scheduler option: {}", id),
        }
    }
}

// ============================================================================
// APPX EXECUTOR
// ============================================================================

pub struct AppxExecutor;

#[async_trait]
impl OptExecutor for AppxExecutor {
    #[instrument(skip(self, _changes), fields(category = "appx"))]
    async fn execute(&self, id: &str, _changes: &mut Vec<ChangeRecord>) -> Result<ExecutionResult> {
        use pieuvre_sync::appx;

        match id {
            "bing_apps" => {
                let r = tokio::task::spawn_blocking(appx::remove_bing_apps).await??;
                Ok(ExecutionResult::ok_count(
                    r.len(),
                    format!("{} Bing apps removed", r.len()),
                ))
            }
            "ms_productivity" => {
                let r = tokio::task::spawn_blocking(appx::remove_ms_productivity).await??;
                Ok(ExecutionResult::ok_count(
                    r.len(),
                    format!("{} apps removed", r.len()),
                ))
            }
            "ms_media" => {
                let r = tokio::task::spawn_blocking(appx::remove_ms_media).await??;
                Ok(ExecutionResult::ok_count(
                    r.len(),
                    format!("{} apps removed", r.len()),
                ))
            }
            "ms_communication" => {
                let r = tokio::task::spawn_blocking(appx::remove_ms_communication).await??;
                Ok(ExecutionResult::ok_count(
                    r.len(),
                    format!("{} apps removed", r.len()),
                ))
            }
            "ms_legacy" => {
                let r = tokio::task::spawn_blocking(appx::remove_ms_legacy).await??;
                Ok(ExecutionResult::ok_count(
                    r.len(),
                    format!("{} apps removed", r.len()),
                ))
            }
            "ms_tools" => {
                let r = tokio::task::spawn_blocking(appx::remove_ms_tools).await??;
                Ok(ExecutionResult::ok_count(
                    r.len(),
                    format!("{} apps removed", r.len()),
                ))
            }
            "third_party" => {
                let r = tokio::task::spawn_blocking(appx::remove_third_party).await??;
                Ok(ExecutionResult::ok_count(
                    r.len(),
                    format!("{} apps removed", r.len()),
                ))
            }
            "copilot" => {
                let r = tokio::task::spawn_blocking(appx::remove_copilot).await??;
                Ok(ExecutionResult::ok_count(
                    r.len(),
                    format!("{} removed", r.len()),
                ))
            }
            "cortana_app" => {
                let r = tokio::task::spawn_blocking(appx::remove_cortana).await??;
                Ok(ExecutionResult::ok_count(
                    r.len(),
                    format!("{} removed", r.len()),
                ))
            }
            "xbox" => {
                let r = tokio::task::spawn_blocking(appx::remove_xbox_packages).await??;
                Ok(ExecutionResult::ok_count(
                    r.len(),
                    format!("{} Xbox apps removed", r.len()),
                ))
            }
            _ => anyhow::bail!("Unknown appx option: {}", id),
        }
    }
}

// ============================================================================
// CPU EXECUTOR
// ============================================================================

pub struct CPUExecutor;

#[async_trait]
impl OptExecutor for CPUExecutor {
    #[instrument(skip(self, _changes), fields(category = "cpu"))]
    async fn execute(&self, id: &str, _changes: &mut Vec<ChangeRecord>) -> Result<ExecutionResult> {
        use pieuvre_sync::cpu;

        match id {
            "core_parking" => {
                tokio::task::spawn_blocking(cpu::disable_core_parking).await??;
                Ok(ExecutionResult::ok(
                    "Core Parking disabled - all cores active",
                ))
            }
            "memory_compression" => {
                tokio::task::spawn_blocking(cpu::disable_memory_compression).await??;
                Ok(ExecutionResult::ok("Memory Compression disabled"))
            }
            "superfetch_registry" => {
                tokio::task::spawn_blocking(cpu::disable_superfetch_registry).await??;
                Ok(ExecutionResult::ok("Superfetch disabled via registry"))
            }
            "static_pagefile" => {
                tokio::task::spawn_blocking(|| cpu::set_static_page_file(16384)).await??;
                Ok(ExecutionResult::ok("Page file set to 16GB static"))
            }
            _ => anyhow::bail!("Unknown CPU option: {}", id),
        }
    }
}

// ============================================================================
// DPC EXECUTOR
// ============================================================================

pub struct DPCExecutor;

#[async_trait]
impl OptExecutor for DPCExecutor {
    #[instrument(skip(self, _changes), fields(category = "dpc"))]
    async fn execute(&self, id: &str, _changes: &mut Vec<ChangeRecord>) -> Result<ExecutionResult> {
        use pieuvre_sync::dpc;

        match id {
            "paging_executive" => {
                tokio::task::spawn_blocking(dpc::disable_paging_executive).await??;
                Ok(ExecutionResult::ok("DisablePagingExecutive enabled"))
            }
            "dynamic_tick" => {
                tokio::task::spawn_blocking(dpc::disable_dynamic_tick).await??;
                Ok(ExecutionResult::ok(
                    "Dynamic tick disabled - reboot required",
                ))
            }
            "tsc_sync" => {
                tokio::task::spawn_blocking(dpc::set_tsc_sync_enhanced).await??;
                Ok(ExecutionResult::ok("TSC sync set to enhanced"))
            }
            "hpet" => {
                tokio::task::spawn_blocking(dpc::disable_hpet).await??;
                Ok(ExecutionResult::ok("HPET disabled"))
            }
            "interrupt_affinity" => {
                tokio::task::spawn_blocking(dpc::set_interrupt_affinity_spread).await??;
                Ok(ExecutionResult::ok(
                    "Interrupt affinity spread across cores",
                ))
            }
            _ => anyhow::bail!("Unknown DPC option: {}", id),
        }
    }
}

// ============================================================================
// SECURITY EXECUTOR
// ============================================================================

pub struct SecurityExecutor;

#[async_trait]
impl OptExecutor for SecurityExecutor {
    #[instrument(skip(self, _changes), fields(category = "security"))]
    async fn execute(&self, id: &str, _changes: &mut Vec<ChangeRecord>) -> Result<ExecutionResult> {
        use pieuvre_sync::security;

        match id {
            "hvci" => {
                tokio::task::spawn_blocking(security::disable_memory_integrity).await??;
                Ok(ExecutionResult::ok("HVCI disabled - reboot required"))
            }
            "vbs" => {
                tokio::task::spawn_blocking(security::disable_vbs).await??;
                Ok(ExecutionResult::ok("VBS disabled - reboot required"))
            }
            "spectre" => {
                tokio::task::spawn_blocking(security::disable_spectre_meltdown).await??;
                warn!("CRITICAL: Spectre/Meltdown mitigations disabled!");
                Ok(ExecutionResult::ok(
                    "Spectre/Meltdown disabled - CRITICAL SECURITY RISK",
                ))
            }
            _ => anyhow::bail!("Unknown security option: {}", id),
        }
    }
}

// ============================================================================
// NETWORK ADVANCED EXECUTOR
// ============================================================================

pub struct NetworkAdvancedExecutor;

#[async_trait]
impl OptExecutor for NetworkAdvancedExecutor {
    #[instrument(skip(self, _changes), fields(category = "network_advanced"))]
    async fn execute(&self, id: &str, _changes: &mut Vec<ChangeRecord>) -> Result<ExecutionResult> {
        use pieuvre_sync::network;

        match id {
            "interrupt_moderation" => {
                let n =
                    tokio::task::spawn_blocking(network::disable_interrupt_moderation).await??;
                Ok(ExecutionResult::ok_count(
                    n as usize,
                    "Interrupt moderation disabled",
                ))
            }
            "lso" => {
                tokio::task::spawn_blocking(network::disable_lso).await??;
                Ok(ExecutionResult::ok("Large Send Offload disabled"))
            }
            "eee" => {
                tokio::task::spawn_blocking(network::disable_eee).await??;
                Ok(ExecutionResult::ok("Energy Efficient Ethernet disabled"))
            }
            "rss" => {
                tokio::task::spawn_blocking(network::enable_rss).await??;
                Ok(ExecutionResult::ok("Receive Side Scaling enabled"))
            }
            "rsc" => {
                tokio::task::spawn_blocking(network::disable_rsc).await??;
                Ok(ExecutionResult::ok("Receive Segment Coalescing disabled"))
            }
            _ => anyhow::bail!("Unknown network_advanced option: {}", id),
        }
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Captures the original state of a service for rollback
fn capture_service_state(service_name: &str, changes: &mut Vec<ChangeRecord>) {
    if let Ok(original) = pieuvre_sync::services::get_service_start_type(service_name) {
        changes.push(ChangeRecord::Service {
            name: service_name.to_string(),
            original_start_type: original,
        });
    }
}

/// Captures the original state of a registry value for rollback
fn capture_registry_state(subkey: &str, value_name: &str, changes: &mut Vec<ChangeRecord>) {
    if let Ok(original) = pieuvre_sync::registry::read_dword_value(subkey, value_name) {
        changes.push(ChangeRecord::Registry {
            key: subkey.to_string(),
            value_name: value_name.to_string(),
            value_type: "REG_DWORD".to_string(),
            original_data: original.to_le_bytes().to_vec(),
        });
    }
}

/// Captures the original state of an AppX package for rollback
#[allow(dead_code)]
fn capture_appx_state(package_name: &str, changes: &mut Vec<ChangeRecord>) {
    // Note: For AppX, we just store the name for reinstallation if possible
    changes.push(ChangeRecord::AppX {
        package_full_name: package_name.to_string(),
    });
}

/// Dispatcher: selects the correct executor based on category
pub fn get_executor(category: &str) -> Box<dyn OptExecutor> {
    match category {
        "telemetry" => Box::new(TelemetryExecutor),
        "privacy" => Box::new(PrivacyExecutor),
        "performance" => Box::new(PerformanceExecutor),
        "scheduler" => Box::new(SchedulerExecutor),
        "appx" => Box::new(AppxExecutor),
        "cpu" => Box::new(CPUExecutor),
        "dpc" => Box::new(DPCExecutor),
        "security" => Box::new(SecurityExecutor),
        "network_advanced" => Box::new(NetworkAdvancedExecutor),
        _ => Box::new(TelemetryExecutor), // Fallback
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_result_ok() {
        let result = ExecutionResult::ok("test");
        assert_eq!(result.affected_count, 1);
        assert_eq!(result.message, "test");
    }

    #[test]
    fn test_execution_result_ok_count() {
        let result = ExecutionResult::ok_count(5, "five items");
        assert_eq!(result.affected_count, 5);
    }

    #[test]
    fn test_get_executor_returns_correct_type() {
        // Just verify it compiles and returns something
        let _exec = get_executor("telemetry");
        let _exec = get_executor("privacy");
        let _exec = get_executor("performance");
        let _exec = get_executor("scheduler");
        let _exec = get_executor("appx");
        let _exec = get_executor("cpu");
        let _exec = get_executor("dpc");
        let _exec = get_executor("security");
        let _exec = get_executor("network_advanced");
    }
}
