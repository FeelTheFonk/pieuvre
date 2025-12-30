use crate::commands::interactive::types::{ExecutionResult, TweakCommand};
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

/// Registre central des commandes de tweaks (SOTA v0.7.0)
pub struct CommandRegistry {
    commands: HashMap<String, Arc<dyn TweakCommand>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            commands: HashMap::new(),
        };
        registry.register_all();
        registry
    }

    fn register_all(&mut self) {
        // --- TÉLÉMÉTRIE ---
        self.register("diagtrack", ServiceDisableCommand::new("DiagTrack"));
        self.register("dmwappush", ServiceDisableCommand::new("dmwappushservice"));
        self.register("wersvc", ServiceDisableCommand::new("WerSvc"));
        self.register("firewall", FirewallTelemetryBlockCommand);
        self.register("sched_tasks", ScheduledTasksTelemetryCommand);
        self.register("hosts", HostsTelemetryCommand);
        self.register("onedrive", OneDriveUninstallCommand);

        // --- PRIVACY ---
        self.register(
            "telemetry_level",
            RegistryDwordCommand::new(
                r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
                "AllowTelemetry",
                0,
            ),
        );
        self.register("advertising_id", RegistryDisableAdvIdCommand);
        self.register("location", RegistryDisableLocationCommand);
        self.register("activity_history", RegistryDisableActivityHistoryCommand);
        self.register("cortana", RegistryDisableCortanaCommand);
        self.register("recall", RegistryDisableRecallCommand);
        self.register("context_menu", ContextMenuClassicCommand);
        self.register("edge_telemetry", EdgeTelemetryDisableCommand);

        // --- O&O PRIVACY ---
        self.register("oo_telemetry", OORecommendedPrivacyCommand);
        self.register("oo_advertising", RegistryDisableAdvIdCommand);
        self.register("oo_copilot", AppxRemoveCopilotCommand);
        self.register("oo_recall", RegistryDisableRecallCommand);
        self.register("oo_widgets", OORecommendedPrivacyCommand);
        self.register("oo_search_highlights", RegistryDisableCortanaCommand);
        self.register("oo_wudo", OORecommendedPrivacyCommand);
        self.register("oo_wifi_sense", OORecommendedPrivacyCommand);
        self.register("oo_app_permissions", RegistryDisableLocationCommand);
        self.register("oo_bg_apps", OORecommendedPrivacyCommand);

        // --- PERFORMANCE ---
        self.register("timer", TimerResolutionCommand::new(5000));
        self.register(
            "power_ultimate",
            PowerPlanCommand::new(pieuvre_sync::power::PowerPlan::UltimatePerformance),
        );
        self.register("cpu_throttle", CpuThrottlingDisableCommand);
        self.register("msi", MsiEnableAllCommand);
        self.register("hags", HagsDisableCommand);
        self.register("nagle", NagleDisableCommand);
        self.register("interrupts", InterruptsOptimizeCommand);
        self.register("memory", MemoryOptimizeCommand);

        // --- SECURITY ---
        self.register("hvci", SecurityDisableHvciCommand);
        self.register("vbs", SecurityDisableVbsCommand);
        self.register("spectre", SecurityDisableSpectreCommand);
        self.register("uac_level", SecurityDisableUacCommand);

        // --- SYSTEM & MAINTENANCE ---
        self.register("cleanup_temp", CleanupTempCommand);
        self.register("cleanup_winsxs", CleanupWinSxSCommand);
        self.register("cleanup_edge", CleanupEdgeCommand);
        self.register("dns_doh", DnsDohCommand);
        self.register("dns_flush", DnsFlushCommand);
        self.register("explorer_optimize", ExplorerOptimizeCommand);
        self.register("explorer_restart", ExplorerRestartCommand);
        self.register("hardening_lock", HardeningLockCommand);
        self.register("hardening_unlock", HardeningUnlockCommand);
        self.register("hardening_ppl", HardeningPplCommand);
        self.register("windows_update", WindowsUpdateConfigureCommand);
    }

    pub fn register(&mut self, id: &str, command: impl TweakCommand + 'static) {
        self.commands.insert(id.to_string(), Arc::new(command));
    }

    pub async fn execute(&self, id: &str) -> Result<ExecutionResult> {
        if let Some(cmd) = self.commands.get(id) {
            cmd.execute().await
        } else {
            // Fallback pour les commandes non encore migrées vers le nouveau système
            anyhow::bail!("Command not yet migrated to SOTA Registry: {}", id)
        }
    }
}

// --- COMMANDES DE SERVICES ---

pub struct ServiceDisableCommand {
    name: String,
}
impl ServiceDisableCommand {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
#[async_trait]
impl TweakCommand for ServiceDisableCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        let name = self.name.clone();
        tokio::task::spawn_blocking(move || pieuvre_sync::services::disable_service(&name))
            .await??;
        Ok(ExecutionResult::ok(format!(
            "Service {} disabled",
            self.name
        )))
    }
}

// --- COMMANDES DE REGISTRE ---

pub struct RegistryDwordCommand {
    key: String,
    value: String,
    data: u32,
}
impl RegistryDwordCommand {
    pub fn new(key: &str, value: &str, data: u32) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
            data,
        }
    }
}
#[async_trait]
impl TweakCommand for RegistryDwordCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        let (k, v, d) = (self.key.clone(), self.value.clone(), self.data);
        tokio::task::spawn_blocking(move || pieuvre_sync::registry::set_dword_value(&k, &v, d))
            .await??;
        Ok(ExecutionResult::ok(format!(
            "Registry {} set to {}",
            self.value, self.data
        )))
    }
}

pub struct RegistryDisableAdvIdCommand;
#[async_trait]
impl TweakCommand for RegistryDisableAdvIdCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(pieuvre_sync::registry::disable_advertising_id).await??;
        Ok(ExecutionResult::ok("Advertising ID disabled"))
    }
}

pub struct RegistryDisableLocationCommand;
#[async_trait]
impl TweakCommand for RegistryDisableLocationCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(pieuvre_sync::registry::disable_location).await??;
        Ok(ExecutionResult::ok("Location tracking disabled"))
    }
}

pub struct RegistryDisableRecallCommand;
#[async_trait]
impl TweakCommand for RegistryDisableRecallCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(pieuvre_sync::registry::disable_recall).await??;
        Ok(ExecutionResult::ok("Windows Recall blocked"))
    }
}

pub struct RegistryDisableCortanaCommand;
#[async_trait]
impl TweakCommand for RegistryDisableCortanaCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(pieuvre_sync::registry::disable_cortana).await??;
        Ok(ExecutionResult::ok("Cortana & Search Highlights disabled"))
    }
}

pub struct RegistryDisableActivityHistoryCommand;
#[async_trait]
impl TweakCommand for RegistryDisableActivityHistoryCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(pieuvre_sync::registry::disable_activity_history).await??;
        Ok(ExecutionResult::ok("Activity history disabled"))
    }
}

// --- COMMANDES DE SÉCURITÉ (PERFORMANCE) ---

pub struct SecurityDisableHvciCommand;
#[async_trait]
impl TweakCommand for SecurityDisableHvciCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(pieuvre_sync::security::disable_memory_integrity).await??;
        Ok(ExecutionResult::ok("Memory Integrity (HVCI) disabled"))
    }
}

pub struct SecurityDisableVbsCommand;
#[async_trait]
impl TweakCommand for SecurityDisableVbsCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(pieuvre_sync::security::disable_vbs).await??;
        Ok(ExecutionResult::ok("VBS completely disabled"))
    }
}

pub struct SecurityDisableSpectreCommand;
#[async_trait]
impl TweakCommand for SecurityDisableSpectreCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(pieuvre_sync::security::disable_spectre_meltdown).await??;
        Ok(ExecutionResult::ok("Spectre/Meltdown mitigations disabled"))
    }
}

pub struct SecurityDisableUacCommand;
#[async_trait]
impl TweakCommand for SecurityDisableUacCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(|| {
            pieuvre_sync::registry::set_dword_value(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System",
                "ConsentPromptBehaviorAdmin",
                0,
            )?;
            pieuvre_sync::registry::set_dword_value(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System",
                "PromptOnSecureDesktop",
                0,
            )
        })
        .await??;
        Ok(ExecutionResult::ok("UAC disabled (Never Notify)"))
    }
}

// --- COMMANDES SPÉCIALISÉES ---

pub struct OORecommendedPrivacyCommand;
#[async_trait]
impl TweakCommand for OORecommendedPrivacyCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(pieuvre_sync::privacy_o_o::apply_all_recommended_privacy)
            .await??;
        Ok(ExecutionResult::ok(
            "O&O: Recommended privacy settings applied",
        ))
    }
}

pub struct FirewallTelemetryBlockCommand;
#[async_trait]
impl TweakCommand for FirewallTelemetryBlockCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        let r = tokio::task::spawn_blocking(pieuvre_sync::firewall::create_telemetry_block_rules)
            .await??;
        Ok(ExecutionResult::ok_count(
            r.len(),
            "Firewall telemetry rules created",
        ))
    }
}

pub struct ScheduledTasksTelemetryCommand;
#[async_trait]
impl TweakCommand for ScheduledTasksTelemetryCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        let t = tokio::task::spawn_blocking(pieuvre_sync::scheduled_tasks::disable_telemetry_tasks)
            .await??;
        Ok(ExecutionResult::ok_count(
            t.len(),
            "Telemetry scheduled tasks disabled",
        ))
    }
}

pub struct HostsTelemetryCommand;
#[async_trait]
impl TweakCommand for HostsTelemetryCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        let c = tokio::task::spawn_blocking(pieuvre_sync::hosts::add_telemetry_blocks).await??;
        Ok(ExecutionResult::ok_count(
            c as usize,
            "Telemetry domains blocked in hosts file",
        ))
    }
}

pub struct OneDriveUninstallCommand;
#[async_trait]
impl TweakCommand for OneDriveUninstallCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(pieuvre_sync::onedrive::uninstall_onedrive).await??;
        Ok(ExecutionResult::ok("OneDrive uninstalled"))
    }
}

pub struct ContextMenuClassicCommand;
#[async_trait]
impl TweakCommand for ContextMenuClassicCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(pieuvre_sync::context_menu::remove_context_menu_clutter)
            .await??;
        Ok(ExecutionResult::ok("Classic context menu enabled"))
    }
}

pub struct AppxRemoveCopilotCommand;
#[async_trait]
impl TweakCommand for AppxRemoveCopilotCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(pieuvre_sync::appx::remove_copilot).await??;
        Ok(ExecutionResult::ok("Copilot components removed"))
    }
}

pub struct TimerResolutionCommand {
    resolution: u32,
}
impl TimerResolutionCommand {
    pub fn new(res: u32) -> Self {
        Self { resolution: res }
    }
}
#[async_trait]
impl TweakCommand for TimerResolutionCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        let res = self.resolution;
        tokio::task::spawn_blocking(move || pieuvre_sync::timer::set_timer_resolution(res))
            .await??;
        Ok(ExecutionResult::ok("Timer resolution optimized"))
    }
}

pub struct PowerPlanCommand {
    plan: pieuvre_sync::power::PowerPlan,
}
impl PowerPlanCommand {
    pub fn new(plan: pieuvre_sync::power::PowerPlan) -> Self {
        Self { plan }
    }
}
#[async_trait]
impl TweakCommand for PowerPlanCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        let plan = self.plan;
        tokio::task::spawn_blocking(move || pieuvre_sync::power::set_power_plan(plan)).await??;
        Ok(ExecutionResult::ok("Power plan applied"))
    }
}

pub struct CpuThrottlingDisableCommand;
#[async_trait]
impl TweakCommand for CpuThrottlingDisableCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(pieuvre_sync::power::disable_cpu_throttling).await??;
        Ok(ExecutionResult::ok("CPU Throttling disabled"))
    }
}

pub struct MsiEnableAllCommand;
#[async_trait]
impl TweakCommand for MsiEnableAllCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        let devices =
            tokio::task::spawn_blocking(pieuvre_sync::msi::list_msi_eligible_devices).await??;
        let mut count = 0;
        for dev in &devices {
            if !dev.msi_enabled && pieuvre_sync::msi::enable_msi(&dev.full_path).is_ok() {
                count += 1;
            }
        }
        Ok(ExecutionResult::ok_count(
            count,
            "MSI mode enabled for devices",
        ))
    }
}

pub struct HagsDisableCommand;
#[async_trait]
impl TweakCommand for HagsDisableCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(pieuvre_sync::game_mode::disable_hags).await??;
        Ok(ExecutionResult::ok("HAGS disabled"))
    }
}

pub struct NagleDisableCommand;
#[async_trait]
impl TweakCommand for NagleDisableCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        let n =
            tokio::task::spawn_blocking(pieuvre_sync::network::disable_nagle_algorithm).await??;
        Ok(ExecutionResult::ok_count(
            n as usize,
            "Nagle algorithm disabled",
        ))
    }
}

pub struct EdgeTelemetryDisableCommand;
#[async_trait]
impl TweakCommand for EdgeTelemetryDisableCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(pieuvre_sync::edge::disable_edge).await??;
        Ok(ExecutionResult::ok("Edge features disabled"))
    }
}

pub struct InterruptsOptimizeCommand;
#[async_trait]
impl TweakCommand for InterruptsOptimizeCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(|| {
            pieuvre_sync::interrupts::InterruptSteering::steer_high_latency_drivers(1000, 0x1)
        })
        .await??;
        Ok(ExecutionResult::ok("Interrupt moderation optimized"))
    }
}

pub struct MemoryOptimizeCommand;
#[async_trait]
impl TweakCommand for MemoryOptimizeCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(pieuvre_sync::memory::enable_large_system_cache).await??;
        Ok(ExecutionResult::ok("Large System Cache enabled"))
    }
}

pub struct CleanupTempCommand;
#[async_trait]
impl TweakCommand for CleanupTempCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(pieuvre_sync::cleanup::cleanup_temp_files).await??;
        Ok(ExecutionResult::ok("Temporary files cleaned"))
    }
}

pub struct CleanupWinSxSCommand;
#[async_trait]
impl TweakCommand for CleanupWinSxSCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(pieuvre_sync::cleanup::cleanup_winsxs).await??;
        Ok(ExecutionResult::ok("WinSxS cleanup completed"))
    }
}

pub struct CleanupEdgeCommand;
#[async_trait]
impl TweakCommand for CleanupEdgeCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(pieuvre_sync::cleanup::cleanup_edge_cache).await??;
        Ok(ExecutionResult::ok("Edge cache cleaned"))
    }
}

pub struct DnsDohCommand;
#[async_trait]
impl TweakCommand for DnsDohCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(|| {
            pieuvre_sync::dns::set_doh_provider(pieuvre_sync::dns::DNSProvider::Cloudflare)
        })
        .await??;
        Ok(ExecutionResult::ok("DNS-over-HTTPS configured (Cloudflare)"))
    }
}

pub struct DnsFlushCommand;
#[async_trait]
impl TweakCommand for DnsFlushCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(pieuvre_sync::dns::flush_dns_cache).await??;
        Ok(ExecutionResult::ok("DNS cache flushed"))
    }
}

pub struct ExplorerOptimizeCommand;
#[async_trait]
impl TweakCommand for ExplorerOptimizeCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(pieuvre_sync::explorer::apply_explorer_tweaks).await??;
        Ok(ExecutionResult::ok("Explorer settings optimized"))
    }
}

pub struct ExplorerRestartCommand;
#[async_trait]
impl TweakCommand for ExplorerRestartCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(pieuvre_sync::explorer::restart_explorer).await??;
        Ok(ExecutionResult::ok("Explorer restarted"))
    }
}

pub struct HardeningLockCommand;
#[async_trait]
impl TweakCommand for HardeningLockCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(|| {
            for key in pieuvre_sync::hardening::CRITICAL_KEYS {
                let _ = pieuvre_sync::hardening::lock_registry_key(key);
            }
            for svc in pieuvre_sync::hardening::CRITICAL_SERVICES {
                let _ = pieuvre_sync::hardening::lock_service(svc);
            }
            Ok::<(), anyhow::Error>(())
        })
        .await??;
        Ok(ExecutionResult::ok("Critical keys and services locked"))
    }
}

pub struct HardeningUnlockCommand;
#[async_trait]
impl TweakCommand for HardeningUnlockCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(|| {
            for key in pieuvre_sync::hardening::CRITICAL_KEYS {
                let _ = pieuvre_sync::hardening::unlock_registry_key(key);
            }
            Ok::<(), anyhow::Error>(())
        })
        .await??;
        Ok(ExecutionResult::ok("Critical keys unlocked"))
    }
}

pub struct HardeningPplCommand;
#[async_trait]
impl TweakCommand for HardeningPplCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(pieuvre_sync::hardening::enable_ppl_protection).await??;
        Ok(ExecutionResult::ok("PPL protection enabled"))
    }
}

pub struct WindowsUpdateConfigureCommand;
#[async_trait]
impl TweakCommand for WindowsUpdateConfigureCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(pieuvre_sync::windows_update::pause_updates).await??;
        Ok(ExecutionResult::ok("Windows Update paused for 35 days"))
    }
}

// --- FIN DES COMMANDES SOTA v0.7.0 ---
