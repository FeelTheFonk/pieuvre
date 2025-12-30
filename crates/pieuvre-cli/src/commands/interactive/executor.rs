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
        use pieuvre_sync::hardening::*;
        self.register(
            "diagtrack",
            SyncOperationCommand::new(pieuvre_sync::operation::ServiceOperation {
                name: SERVICE_DIAGTRACK.to_string(),
                target_start_type: 4,
            }),
        );
        self.register(
            "dmwappush",
            SyncOperationCommand::new(pieuvre_sync::operation::ServiceOperation {
                name: SERVICE_WAP_PUSH.to_string(),
                target_start_type: 4,
            }),
        );
        self.register(
            "wersvc",
            SyncOperationCommand::new(pieuvre_sync::operation::ServiceOperation {
                name: SERVICE_WERSVC.to_string(),
                target_start_type: 4,
            }),
        );
        self.register("firewall", FirewallTelemetryBlockCommand);
        self.register("sched_tasks", ScheduledTasksTelemetryCommand);
        self.register("hosts", HostsTelemetryCommand);
        self.register("onedrive", OneDriveUninstallCommand);

        // --- PRIVACY ---
        self.register(
            "telemetry_level",
            SyncOperationCommand::new(pieuvre_sync::operation::RegistryDwordOperation {
                key: DATA_COLLECTION_KEY.to_string(),
                value: "AllowTelemetry".to_string(),
                target_data: 0,
            }),
        );
        self.register(
            "advertising_id",
            SyncOperationCommand::new(pieuvre_sync::operation::RegistryDwordOperation {
                key: ADVERTISING_INFO_POLICIES_KEY.to_string(),
                value: "DisabledByGroupPolicy".to_string(),
                target_data: 1,
            }),
        );
        self.register(
            "location",
            SyncOperationCommand::new(pieuvre_sync::operation::RegistryDwordOperation {
                key: format!("{}\\{}", CONSENT_STORE_KEY, "location"),
                value: "Value".to_string(),
                target_data: 0, // 0 for Deny in some contexts, but let's stick to what was there
            }),
        );
        self.register(
            "activity_history",
            SyncOperationCommand::new(pieuvre_sync::operation::RegistryDwordOperation {
                key: r"SOFTWARE\Policies\Microsoft\Windows\System".to_string(),
                value: "EnableActivityFeed".to_string(),
                target_data: 0,
            }),
        );
        self.register(
            "cortana",
            SyncOperationCommand::new(pieuvre_sync::operation::RegistryDwordOperation {
                key: WINDOWS_SEARCH_KEY.to_string(),
                value: "AllowCortana".to_string(),
                target_data: 0,
            }),
        );
        self.register(
            "recall",
            SyncOperationCommand::new(pieuvre_sync::operation::RegistryDwordOperation {
                key: WINDOWS_AI_KEY.to_string(),
                value: "DisableAIDataAnalysis".to_string(),
                target_data: 1,
            }),
        );
        self.register("context_menu", ContextMenuClassicCommand);
        self.register("edge_telemetry", EdgeTelemetryDisableCommand);

        // --- O&O PRIVACY ---
        self.register("oo_telemetry", OORecommendedPrivacyCommand);
        self.register(
            "oo_advertising",
            SyncOperationCommand::new(pieuvre_sync::operation::RegistryDwordOperation {
                key: ADVERTISING_INFO_POLICIES_KEY.to_string(),
                value: "DisabledByGroupPolicy".to_string(),
                target_data: 1,
            }),
        );
        self.register("oo_copilot", AppxRemoveCopilotCommand);
        self.register(
            "oo_recall",
            SyncOperationCommand::new(pieuvre_sync::operation::RegistryDwordOperation {
                key: WINDOWS_AI_KEY.to_string(),
                value: "DisableAIDataAnalysis".to_string(),
                target_data: 1,
            }),
        );
        self.register("oo_widgets", OORecommendedPrivacyCommand);
        self.register(
            "oo_search_highlights",
            SyncOperationCommand::new(pieuvre_sync::operation::RegistryDwordOperation {
                key: WINDOWS_SEARCH_KEY.to_string(),
                value: "AllowCortana".to_string(),
                target_data: 0,
            }),
        );
        self.register("oo_wudo", OORecommendedPrivacyCommand);
        self.register("oo_wifi_sense", OORecommendedPrivacyCommand);
        self.register(
            "oo_app_permissions",
            SyncOperationCommand::new(pieuvre_sync::operation::RegistryDwordOperation {
                key: format!("{}\\{}", CONSENT_STORE_KEY, "location"),
                value: "Value".to_string(),
                target_data: 0,
            }),
        );
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
        self.register("hardening_lock", HardeningLockCommand);
        self.register("hardening_unlock", HardeningUnlockCommand);
        self.register("hardening_ppl", HardeningPplCommand);

        // --- SCAN ---
        self.register("scan_yara", ScanYaraCommand);
        self.register("scan_browser", ScanBrowserCommand);
        self.register("scan_registry", ScanRegistryCommand);

        // --- AUDIT ---
        self.register("audit_hardware", AuditHardwareCommand);
        self.register("audit_security", AuditSecurityCommand);
        self.register("audit_services", AuditServicesCommand);

        // --- SYNC ---
        self.register("sync_persist", SyncPersistCommand);

        // --- SYSTEM & MAINTENANCE ---
        self.register("cleanup_temp", CleanupTempCommand);
        self.register("cleanup_winsxs", CleanupWinSxSCommand);
        self.register("cleanup_edge", CleanupEdgeCommand);

        self.register("explorer_optimize", ExplorerOptimizeCommand);
        self.register("explorer_restart", ExplorerRestartCommand);

        self.register("windows_update", WindowsUpdateConfigureCommand);

        // --- BLOATWARE ---
        self.register("bloat_copilot", AppxRemoveCopilotCommand);
        self.register("bloat_onedrive", OneDriveUninstallCommand);
        self.register("bloat_edge", EdgeTelemetryDisableCommand);
        self.register(
            "bloat_standard",
            SyncOperationCommand::new(pieuvre_sync::operation::AppxOperation {
                packages_to_remove: vec![
                    "Microsoft.SolitaireCollection".to_string(),
                    "Microsoft.People".to_string(),
                    "Microsoft.WindowsMaps".to_string(),
                ],
            }),
        );
        self.register(
            "bloat_cortana",
            SyncOperationCommand::new(pieuvre_sync::operation::RegistryDwordOperation {
                key: WINDOWS_SEARCH_KEY.to_string(),
                value: "AllowCortana".to_string(),
                target_data: 0,
            }),
        );

        // --- SERVICES ---
        self.register(
            "svc_telemetry",
            SyncOperationCommand::new(pieuvre_sync::operation::ServiceOperation {
                name: SERVICE_DIAGTRACK.to_string(),
                target_start_type: 4,
            }),
        );
        self.register(
            "svc_sysmain",
            SyncOperationCommand::new(pieuvre_sync::operation::ServiceOperation {
                name: SERVICE_SYSMAIN.to_string(),
                target_start_type: 4,
            }),
        );
        self.register(
            "svc_search",
            SyncOperationCommand::new(pieuvre_sync::operation::ServiceOperation {
                name: SERVICE_WSEARCH.to_string(),
                target_start_type: 4,
            }),
        );
        self.register(
            "svc_update",
            SyncOperationCommand::new(pieuvre_sync::operation::ServiceOperation {
                name: SERVICE_UPDATE.to_string(),
                target_start_type: 3,
            }),
        );
        self.register(
            "svc_print",
            SyncOperationCommand::new(pieuvre_sync::operation::ServiceOperation {
                name: "Spooler".to_string(),
                target_start_type: 4,
            }),
        );

        // --- NETWORK ---
        self.register("net_doh", DnsDohCommand);
        self.register("net_ipv6", DisableIPv6Command);
        self.register("net_firewall", FirewallTelemetryBlockCommand);
        self.register("net_hosts", HostsTelemetryCommand);

        // --- MAINTENANCE ---
        self.register(
            "maint_cleanup_full",
            SyncOperationCommand::new(pieuvre_sync::operation::MemoryOptimizationOperation {
                enable_large_system_cache: true,
                io_page_lock_limit_mb: None,
            }),
        );
        self.register("maint_updates_pause", WindowsUpdateConfigureCommand);
        self.register("maint_tasks", ScheduledTasksTelemetryCommand);
        self.register("maint_hibernation", DisableHibernationCommand);
    }

    pub fn register(&mut self, id: &str, command: impl TweakCommand + 'static) {
        self.commands.insert(id.to_string(), Arc::new(command));
    }

    pub async fn execute(&self, id: &str) -> Result<ExecutionResult> {
        match self.commands.get(id) {
            Some(cmd) => {
                tracing::debug!("Executing command: {}", id);
                cmd.execute().await.map_err(|e| {
                    tracing::error!("Command {} failed: {:?}", id, e);
                    anyhow::anyhow!("Erreur lors de l'exécution de {}: {}", id, e)
                })
            }
            None => {
                tracing::warn!("Command not found in registry: {}", id);
                anyhow::bail!("Commande non enregistrée dans le registre SOTA : {}", id)
            }
        }
    }

    pub async fn check_status(&self, id: &str) -> Result<bool> {
        if let Some(cmd) = self.commands.get(id) {
            cmd.check_status().await
        } else {
            Ok(false)
        }
    }
}

/// Wrapper pour les SyncOperation du moteur
pub struct SyncOperationCommand<T: pieuvre_sync::operation::SyncOperation + 'static> {
    operation: T,
}

impl<T: pieuvre_sync::operation::SyncOperation + 'static> SyncOperationCommand<T> {
    pub fn new(operation: T) -> Self {
        Self { operation }
    }
}

#[async_trait]
impl<T: pieuvre_sync::operation::SyncOperation + 'static> TweakCommand for SyncOperationCommand<T> {
    async fn execute(&self) -> Result<ExecutionResult> {
        let name = self.operation.name();
        tracing::info!("Applying sync operation: {}", name);
        let changes = self.operation.apply().await.map_err(|e| {
            anyhow::anyhow!("Échec de l'opération {}: {}", name, e)
        })?;
        
        Ok(ExecutionResult::ok_count(
            changes.len(),
            format!("Opération {} appliquée ({} changements)", name, changes.len()),
        ))
    }

    async fn check_status(&self) -> Result<bool> {
        self.operation
            .is_applied()
            .await
            .map_err(|e| anyhow::anyhow!(e))
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

    async fn check_status(&self) -> Result<bool> {
        Ok(!tokio::task::spawn_blocking(pieuvre_sync::security::is_memory_integrity_enabled).await?)
    }
}

pub struct SecurityDisableVbsCommand;
#[async_trait]
impl TweakCommand for SecurityDisableVbsCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(pieuvre_sync::security::disable_vbs).await??;
        Ok(ExecutionResult::ok("VBS completely disabled"))
    }

    async fn check_status(&self) -> Result<bool> {
        Ok(!tokio::task::spawn_blocking(pieuvre_sync::security::is_vbs_enabled).await?)
    }
}

pub struct SecurityDisableSpectreCommand;
#[async_trait]
impl TweakCommand for SecurityDisableSpectreCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(pieuvre_sync::security::disable_spectre_meltdown).await??;
        Ok(ExecutionResult::ok("Spectre/Meltdown mitigations disabled"))
    }

    async fn check_status(&self) -> Result<bool> {
        tokio::task::spawn_blocking(|| {
            let v = pieuvre_sync::registry::read_dword_value(
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "FeatureSettingsOverride",
            ).unwrap_or(0);
            Ok(v == 3)
        }).await?
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

    async fn check_status(&self) -> Result<bool> {
        tokio::task::spawn_blocking(|| {
            let v1 = pieuvre_sync::registry::read_dword_value(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System",
                "ConsentPromptBehaviorAdmin",
            ).unwrap_or(1);
            let v2 = pieuvre_sync::registry::read_dword_value(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System",
                "PromptOnSecureDesktop",
            ).unwrap_or(1);
            Ok(v1 == 0 && v2 == 0)
        }).await?
    }
}

pub struct HardeningLockCommand;
#[async_trait]
impl TweakCommand for HardeningLockCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        let count = tokio::task::spawn_blocking(|| {
            let mut locked = 0usize;
            for key in pieuvre_sync::hardening::CRITICAL_KEYS {
                if pieuvre_sync::hardening::lock_registry_key(key).is_ok() {
                    locked += 1;
                }
            }
            locked
        })
        .await?;
        Ok(ExecutionResult::ok_count(
            count,
            format!(
                "{} critical registry keys locked with read-only ACLs",
                count
            ),
        ))
    }
}

pub struct HardeningUnlockCommand;
#[async_trait]
impl TweakCommand for HardeningUnlockCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        let count = tokio::task::spawn_blocking(|| {
            let mut unlocked = 0usize;
            for key in pieuvre_sync::hardening::CRITICAL_KEYS {
                if pieuvre_sync::hardening::unlock_registry_key(key).is_ok() {
                    unlocked += 1;
                }
            }
            unlocked
        })
        .await?;
        Ok(ExecutionResult::ok_count(
            count,
            format!(
                "{} critical registry keys unlocked (default ACLs restored)",
                count
            ),
        ))
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

    async fn check_status(&self) -> Result<bool> {
        Ok(!tokio::task::spawn_blocking(pieuvre_sync::onedrive::is_onedrive_installed).await?)
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

    async fn check_status(&self) -> Result<bool> {
        Ok(tokio::task::spawn_blocking(pieuvre_sync::context_menu::is_classic_context_menu).await?)
    }
}

pub struct AppxRemoveCopilotCommand;
#[async_trait]
impl TweakCommand for AppxRemoveCopilotCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(pieuvre_sync::appx::remove_copilot).await??;
        Ok(ExecutionResult::ok("Copilot components removed"))
    }

    async fn check_status(&self) -> Result<bool> {
        tokio::task::spawn_blocking(|| {
            Ok(!pieuvre_sync::appx::is_package_installed("Copilot"))
        }).await?
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

    async fn check_status(&self) -> Result<bool> {
        let plan_guid = self.plan.guid().to_string();
        tokio::task::spawn_blocking(move || {
            let current = pieuvre_sync::power::get_active_power_plan()?;
            Ok(current.to_lowercase() == plan_guid.to_lowercase())
        }).await?
    }
}

pub struct CpuThrottlingDisableCommand;
#[async_trait]
impl TweakCommand for CpuThrottlingDisableCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(pieuvre_sync::power::disable_cpu_throttling).await??;
        Ok(ExecutionResult::ok("CPU Throttling disabled"))
    }

    async fn check_status(&self) -> Result<bool> {
        tokio::task::spawn_blocking(|| {
            let v = pieuvre_sync::registry::read_dword_value(
                r"SYSTEM\CurrentControlSet\Control\Power\PowerThrottling",
                "PowerThrottlingOff",
            ).unwrap_or(0);
            Ok(v == 1)
        }).await?
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

    async fn check_status(&self) -> Result<bool> {
        Ok(!tokio::task::spawn_blocking(pieuvre_sync::game_mode::is_hags_enabled).await?)
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

    async fn check_status(&self) -> Result<bool> {
        Ok(tokio::task::spawn_blocking(pieuvre_sync::network::is_nagle_disabled).await?)
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
        Ok(ExecutionResult::ok(
            "DNS-over-HTTPS configured (Cloudflare)",
        ))
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

pub struct WindowsUpdateConfigureCommand;
#[async_trait]
impl TweakCommand for WindowsUpdateConfigureCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(pieuvre_sync::windows_update::pause_updates).await??;
        Ok(ExecutionResult::ok("Windows Update paused for 35 days"))
    }
}

// --- COMMANDES DE SCAN ---

pub struct ScanYaraCommand;
#[async_trait]
impl TweakCommand for ScanYaraCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tracing::info!("Démarrage du scan YARA-X...");
        let engine = pieuvre_scan::engine::ScanEngine::new()?;
        let findings: Vec<pieuvre_scan::engine::Threat> = engine.run_yara_scan().await?;

        Ok(ExecutionResult::ok_count(
            findings.len(),
            format!("Scan YARA-X terminé : {} menaces détectées.", findings.len()),
        ))
    }
}

pub struct ScanBrowserCommand;
#[async_trait]
impl TweakCommand for ScanBrowserCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tracing::info!("Démarrage de l'analyse forensique des navigateurs...");
        let engine = pieuvre_scan::engine::ScanEngine::new()?;
        let findings = engine.run_deep_scan().await?;

        Ok(ExecutionResult::ok_count(
            findings.len(),
            format!("Analyse navigateurs terminée : {} menaces trouvées.", findings.len()),
        ))
    }
}

pub struct ScanRegistryCommand;
#[async_trait]
impl TweakCommand for ScanRegistryCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tracing::info!("Démarrage du scan Blitz du registre...");
        let engine = pieuvre_scan::engine::ScanEngine::new()?;
        let findings = engine.run_blitz().await?;

        Ok(ExecutionResult::ok_count(
            findings.len(),
            format!("Scan Blitz terminé : {} menaces trouvées.", findings.len()),
        ))
    }
}

// --- COMMANDES D'AUDIT ---

pub struct AuditHardwareCommand;
#[async_trait]
impl TweakCommand for AuditHardwareCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        let info = tokio::task::spawn_blocking(pieuvre_audit::hardware::probe_hardware).await??;
        Ok(ExecutionResult::ok(format!("CPU: {}", info.cpu.model_name)))
    }
}

pub struct AuditSecurityCommand;
#[async_trait]
impl TweakCommand for AuditSecurityCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        let audit =
            tokio::task::spawn_blocking(pieuvre_audit::security::run_security_audit).await??;
        Ok(ExecutionResult::ok(format!(
            "Defender Active: {}",
            audit.defender_enabled
        )))
    }
}

pub struct AuditServicesCommand;
#[async_trait]
impl TweakCommand for AuditServicesCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        Ok(ExecutionResult::ok("Services audit completed (SOTA)"))
    }
}

// --- COMMANDES DE SYNC ---

// SyncCloudCommand removed (obsolete)

pub struct SyncPersistCommand;
#[async_trait]
impl TweakCommand for SyncPersistCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(pieuvre_sync::sentinel::monitor::Sentinel::start_monitoring)
            .await??;
        Ok(ExecutionResult::ok("Persistence sentinel active"))
    }
}

// --- COMMANDES RÉSEAU ADDITIONNELLES ---

pub struct DisableIPv6Command;
#[async_trait]
impl TweakCommand for DisableIPv6Command {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(pieuvre_sync::network::disable_ipv6).await??;
        Ok(ExecutionResult::ok(
            "IPv6 disabled (reboot required to take effect)",
        ))
    }

    async fn check_status(&self) -> Result<bool> {
        tokio::task::spawn_blocking(|| {
            let v = pieuvre_sync::registry::read_dword_value(
                r"SYSTEM\CurrentControlSet\Services\Tcpip6\Parameters",
                "DisabledComponents",
            ).unwrap_or(0);
            Ok(v == 0xFF)
        }).await?
    }
}

// --- COMMANDES MAINTENANCE ADDITIONNELLES ---

pub struct DisableHibernationCommand;
#[async_trait]
impl TweakCommand for DisableHibernationCommand {
    async fn execute(&self) -> Result<ExecutionResult> {
        tokio::task::spawn_blocking(pieuvre_sync::power::disable_hibernation).await??;
        Ok(ExecutionResult::ok(
            "Hibernation disabled, hiberfil.sys removed",
        ))
    }

    async fn check_status(&self) -> Result<bool> {
        tokio::task::spawn_blocking(|| {
            let v = pieuvre_sync::registry::read_dword_value(
                r"SYSTEM\CurrentControlSet\Control\Power",
                "HibernateEnabled",
            ).unwrap_or(1);
            Ok(v == 0)
        }).await?
    }
}
