//! Exécuteurs d'optimisations pour le mode interactif
//!
//! Module SOTA 2026: Trait pattern pour exécution polymorphe des optimisations.

use anyhow::Result;
use pieuvre_common::ChangeRecord;
use tracing::{info, warn, instrument};

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
        Self { affected_count: 1, message: message.into() }
    }

    pub fn ok_count(count: usize, message: impl Into<String>) -> Self {
        Self { affected_count: count, message: message.into() }
    }
}

/// Trait pour exécuter une optimisation
pub trait OptExecutor {
    /// Exécute l'optimisation et retourne le résultat
    fn execute(&self, id: &str, changes: &mut Vec<ChangeRecord>) -> Result<ExecutionResult>;
}

// ============================================================================
// TÉLÉMÉTRIE EXECUTOR
// ============================================================================

pub struct TelemetryExecutor;

impl OptExecutor for TelemetryExecutor {
    #[instrument(skip(self, changes), fields(category = "telemetry"))]
    fn execute(&self, id: &str, changes: &mut Vec<ChangeRecord>) -> Result<ExecutionResult> {
        use pieuvre_sync::{firewall, hosts, onedrive, scheduled_tasks, services};

        match id {
            "diagtrack" => {
                capture_service_state("DiagTrack", changes);
                services::disable_service("DiagTrack")?;
                info!(service = "DiagTrack", "Service disabled");
                Ok(ExecutionResult::ok("DiagTrack disabled"))
            }
            "dmwappush" => {
                capture_service_state("dmwappushservice", changes);
                services::disable_service("dmwappushservice")?;
                info!(service = "dmwappushservice", "Service disabled");
                Ok(ExecutionResult::ok("dmwappushservice disabled"))
            }
            "wersvc" => {
                capture_service_state("WerSvc", changes);
                services::disable_service("WerSvc")?;
                Ok(ExecutionResult::ok("WerSvc disabled"))
            }
            "wercplsupport" => {
                capture_service_state("wercplsupport", changes);
                services::disable_service("wercplsupport")?;
                Ok(ExecutionResult::ok("wercplsupport disabled"))
            }
            "pcasvc" => {
                capture_service_state("PcaSvc", changes);
                services::disable_service("PcaSvc")?;
                Ok(ExecutionResult::ok("PcaSvc disabled"))
            }
            "wdisystem" => {
                capture_service_state("WdiSystemHost", changes);
                services::disable_service("WdiSystemHost")?;
                Ok(ExecutionResult::ok("WdiSystemHost disabled"))
            }
            "wdiservice" => {
                capture_service_state("WdiServiceHost", changes);
                services::disable_service("WdiServiceHost")?;
                Ok(ExecutionResult::ok("WdiServiceHost disabled"))
            }
            "lfsvc" => {
                capture_service_state("lfsvc", changes);
                services::disable_service("lfsvc")?;
                Ok(ExecutionResult::ok("lfsvc disabled"))
            }
            "mapsbroker" => {
                capture_service_state("MapsBroker", changes);
                services::disable_service("MapsBroker")?;
                Ok(ExecutionResult::ok("MapsBroker disabled"))
            }
            "firewall" => {
                let rules = firewall::create_telemetry_block_rules()?;
                info!(rules_count = rules.len(), "Firewall rules created");
                Ok(ExecutionResult::ok_count(rules.len(), format!("{} firewall rules", rules.len())))
            }
            "sched_tasks" => {
                let tasks = scheduled_tasks::disable_telemetry_tasks()?;
                info!(tasks_count = tasks.len(), "Scheduled tasks disabled");
                Ok(ExecutionResult::ok_count(tasks.len(), format!("{} tasks disabled", tasks.len())))
            }
            "hosts" => {
                let count = hosts::add_telemetry_blocks()?;
                info!(domains_count = count, "Hosts entries added");
                Ok(ExecutionResult::ok_count(count as usize, format!("{} domains blocked", count)))
            }
            "onedrive" => {
                onedrive::uninstall_onedrive()?;
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

impl OptExecutor for PrivacyExecutor {
    #[instrument(skip(self, _changes), fields(category = "privacy"))]
    fn execute(&self, id: &str, _changes: &mut Vec<ChangeRecord>) -> Result<ExecutionResult> {
        use pieuvre_sync::{context_menu, registry, widgets, windows_update};

        match id {
            "telemetry_level" => {
                registry::set_telemetry_level(0)?;
                Ok(ExecutionResult::ok("Telemetry level 0"))
            }
            "advertising_id" => {
                registry::disable_advertising_id()?;
                Ok(ExecutionResult::ok("Advertising ID disabled"))
            }
            "location" => {
                registry::disable_location()?;
                Ok(ExecutionResult::ok("Location disabled"))
            }
            "activity_history" => {
                registry::disable_activity_history()?;
                Ok(ExecutionResult::ok("Activity history disabled"))
            }
            "cortana" => {
                registry::disable_cortana()?;
                Ok(ExecutionResult::ok("Cortana disabled"))
            }
            "context_menu" => {
                let n = context_menu::remove_context_menu_clutter()?;
                Ok(ExecutionResult::ok_count(n as usize, format!("{} items removed", n)))
            }
            "widgets" => {
                widgets::disable_widgets()?;
                Ok(ExecutionResult::ok("Widgets disabled"))
            }
            "pause_updates" => {
                windows_update::pause_updates()?;
                Ok(ExecutionResult::ok("Updates paused 35 days"))
            }
            "driver_updates" => {
                windows_update::disable_driver_updates()?;
                Ok(ExecutionResult::ok("Driver updates disabled"))
            }
            "recall" => {
                registry::disable_recall()?;
                Ok(ExecutionResult::ok("Windows Recall blocked"))
            }
            "group_policy_telem" => {
                registry::set_group_policy_telemetry(0)?;
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

impl OptExecutor for PerformanceExecutor {
    #[instrument(skip(self, _changes), fields(category = "performance"))]
    fn execute(&self, id: &str, _changes: &mut Vec<ChangeRecord>) -> Result<ExecutionResult> {
        use pieuvre_sync::{edge, explorer, game_mode, msi, network, power, registry, services, timer};

        match id {
            "timer" => {
                timer::set_timer_resolution(5000)?;
                Ok(ExecutionResult::ok("Timer 0.5ms"))
            }
            "power_ultimate" => {
                power::set_power_plan(power::PowerPlan::UltimatePerformance)?;
                Ok(ExecutionResult::ok("Ultimate Performance"))
            }
            "power_high" => {
                power::set_power_plan(power::PowerPlan::HighPerformance)?;
                Ok(ExecutionResult::ok("High Performance"))
            }
            "cpu_throttle" => {
                power::disable_cpu_throttling()?;
                Ok(ExecutionResult::ok("CPU throttling disabled"))
            }
            "usb_suspend" => {
                power::configure_power_settings(false, false, 100, 100)?;
                Ok(ExecutionResult::ok("USB suspend disabled"))
            }
            "msi" => {
                let devices = msi::list_msi_eligible_devices()?;
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
                Ok(ExecutionResult::ok_count(enabled, format!("{}/{} MSI devices", enabled, devices.len())))
            }
            "sysmain" => {
                services::disable_service("SysMain")?;
                Ok(ExecutionResult::ok("SysMain disabled"))
            }
            "wsearch" => {
                services::disable_service("WSearch")?;
                Ok(ExecutionResult::ok("WSearch disabled"))
            }
            "edge_disable" => {
                edge::disable_edge()?;
                let _ = edge::remove_edge_tasks();
                Ok(ExecutionResult::ok("Edge features disabled"))
            }
            "explorer_tweaks" => {
                explorer::apply_explorer_tweaks()?;
                Ok(ExecutionResult::ok("Explorer tweaks applied"))
            }
            "game_bar" => {
                game_mode::disable_game_bar()?;
                Ok(ExecutionResult::ok("Game Bar disabled"))
            }
            "fullscreen_opt" => {
                game_mode::disable_fullscreen_optimizations()?;
                Ok(ExecutionResult::ok("Fullscreen optimizations disabled"))
            }
            "hags" => {
                game_mode::disable_hags()?;
                Ok(ExecutionResult::ok("HAGS disabled"))
            }
            "nagle" => {
                let n = network::disable_nagle_algorithm()?;
                Ok(ExecutionResult::ok_count(n as usize, format!("{} interfaces", n)))
            }
            "power_throttle" => {
                registry::disable_power_throttling()?;
                Ok(ExecutionResult::ok("Power throttling disabled"))
            }
            _ => anyhow::bail!("Unknown performance option: {}", id),
        }
    }
}

// ============================================================================
// SCHEDULER EXECUTOR
// ============================================================================

pub struct SchedulerExecutor;

impl OptExecutor for SchedulerExecutor {
    #[instrument(skip(self, _changes), fields(category = "scheduler"))]
    fn execute(&self, id: &str, _changes: &mut Vec<ChangeRecord>) -> Result<ExecutionResult> {
        use pieuvre_sync::registry;

        match id {
            "priority_sep" => {
                registry::set_priority_separation(0x26)?;
                Ok(ExecutionResult::ok("Win32PrioritySeparation 0x26"))
            }
            "mmcss" => {
                registry::configure_mmcss_gaming()?;
                Ok(ExecutionResult::ok("MMCSS configured"))
            }
            "games_priority" => {
                registry::configure_games_priority()?;
                Ok(ExecutionResult::ok("GPU=8, Priority=6"))
            }
            "global_timer" => {
                registry::enable_global_timer_resolution()?;
                Ok(ExecutionResult::ok("Global timer (reboot required)"))
            }
            "startup_delay" => {
                registry::disable_startup_delay()?;
                Ok(ExecutionResult::ok("Startup delay 0ms"))
            }
            "shutdown_timeout" => {
                registry::reduce_shutdown_timeout()?;
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

impl OptExecutor for AppxExecutor {
    #[instrument(skip(self, _changes), fields(category = "appx"))]
    fn execute(&self, id: &str, _changes: &mut Vec<ChangeRecord>) -> Result<ExecutionResult> {
        use pieuvre_sync::appx;

        match id {
            "bing_apps" => {
                let r = appx::remove_bing_apps()?;
                Ok(ExecutionResult::ok_count(r.len(), format!("{} Bing apps removed", r.len())))
            }
            "ms_productivity" => {
                let r = appx::remove_ms_productivity()?;
                Ok(ExecutionResult::ok_count(r.len(), format!("{} apps removed", r.len())))
            }
            "ms_media" => {
                let r = appx::remove_ms_media()?;
                Ok(ExecutionResult::ok_count(r.len(), format!("{} apps removed", r.len())))
            }
            "ms_communication" => {
                let r = appx::remove_ms_communication()?;
                Ok(ExecutionResult::ok_count(r.len(), format!("{} apps removed", r.len())))
            }
            "ms_legacy" => {
                let r = appx::remove_ms_legacy()?;
                Ok(ExecutionResult::ok_count(r.len(), format!("{} apps removed", r.len())))
            }
            "ms_tools" => {
                let r = appx::remove_ms_tools()?;
                Ok(ExecutionResult::ok_count(r.len(), format!("{} apps removed", r.len())))
            }
            "third_party" => {
                let r = appx::remove_third_party()?;
                Ok(ExecutionResult::ok_count(r.len(), format!("{} apps removed", r.len())))
            }
            "copilot" => {
                let r = appx::remove_copilot()?;
                Ok(ExecutionResult::ok_count(r.len(), format!("{} removed", r.len())))
            }
            "cortana_app" => {
                let r = appx::remove_cortana()?;
                Ok(ExecutionResult::ok_count(r.len(), format!("{} removed", r.len())))
            }
            "xbox" => {
                let r = appx::remove_xbox_packages()?;
                Ok(ExecutionResult::ok_count(r.len(), format!("{} Xbox apps removed", r.len())))
            }
            _ => anyhow::bail!("Unknown appx option: {}", id),
        }
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Capture l'état original d'un service pour rollback
fn capture_service_state(service_name: &str, changes: &mut Vec<ChangeRecord>) {
    if let Ok(original) = pieuvre_sync::services::get_service_start_type(service_name) {
        changes.push(ChangeRecord::Service {
            name: service_name.to_string(),
            original_start_type: original,
        });
    }
}

/// Dispatcher: sélectionne le bon executor selon la catégorie
pub fn get_executor(category: &str) -> Box<dyn OptExecutor> {
    match category {
        "telemetry" => Box::new(TelemetryExecutor),
        "privacy" => Box::new(PrivacyExecutor),
        "performance" => Box::new(PerformanceExecutor),
        "scheduler" => Box::new(SchedulerExecutor),
        "appx" => Box::new(AppxExecutor),
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
    }
}
