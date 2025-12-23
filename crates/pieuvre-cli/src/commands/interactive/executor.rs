use anyhow::Result;
use async_trait::async_trait;
use tracing::instrument;

pub struct ExecutionResult {
    pub _affected_count: usize,
    #[allow(dead_code)]
    pub message: String,
}
impl ExecutionResult {
    pub fn ok(msg: impl Into<String>) -> Self {
        Self {
            _affected_count: 1,
            message: msg.into(),
        }
    }
    pub fn ok_count(count: usize, msg: impl Into<String>) -> Self {
        Self {
            _affected_count: count,
            message: msg.into(),
        }
    }
}

#[async_trait]
pub trait OptExecutor: Send + Sync {
    async fn execute(&self, id: &str) -> Result<ExecutionResult>;
}

async fn svc_disable(name: &str) -> Result<ExecutionResult> {
    let n = name.to_string();
    tokio::task::spawn_blocking(move || pieuvre_sync::services::disable_service(&n)).await??;
    Ok(ExecutionResult::ok(format!("{} disabled", name)))
}

async fn reg_set_dword(key: &str, val: &str, data: u32) -> Result<ExecutionResult> {
    let (k, v) = (key.to_string(), val.to_string());
    tokio::task::spawn_blocking(move || pieuvre_sync::registry::set_dword_value(&k, &v, data))
        .await??;
    Ok(ExecutionResult::ok(format!("{} set to {}", val, data)))
}

macro_rules! impl_exec {
    ($name:ident, $cat:expr, { $($id:pat => $body:expr),* $(,)? }) => {
        pub struct $name;
        #[async_trait]
        impl OptExecutor for $name {
            #[instrument(skip(self), fields(category = $cat))]
            async fn execute(&self, id: &str) -> Result<ExecutionResult> {
                match id { $($id => $body),*, _ => anyhow::bail!("Unknown {} option: {}", $cat, id) }
            }
        }
    };
}

impl_exec!(TelemetryExecutor, "telemetry", {
    "diagtrack" => svc_disable("DiagTrack").await,
    "dmwappush" => svc_disable("dmwappushservice").await,
    "wersvc" => svc_disable("WerSvc").await,
    "wercplsupport" => svc_disable("wercplsupport").await,
    "pcasvc" => svc_disable("PcaSvc").await,
    "wdisystem" => svc_disable("WdiSystemHost").await,
    "wdiservice" => svc_disable("WdiServiceHost").await,
    "lfsvc" => svc_disable("lfsvc").await,
    "mapsbroker" => svc_disable("MapsBroker").await,
    "firewall" => {
        let r = tokio::task::spawn_blocking(pieuvre_sync::firewall::create_telemetry_block_rules).await??;
        Ok(ExecutionResult::ok_count(r.len(), format!("{} firewall rules", r.len())))
    },
    "sched_tasks" => {
        let t = tokio::task::spawn_blocking(pieuvre_sync::scheduled_tasks::disable_telemetry_tasks).await??;
        Ok(ExecutionResult::ok_count(t.len(), format!("{} tasks disabled", t.len())))
    },
    "hosts" => {
        let c = tokio::task::spawn_blocking(pieuvre_sync::hosts::add_telemetry_blocks).await??;
        Ok(ExecutionResult::ok_count(c as usize, format!("{} domains blocked", c)))
    },
    "onedrive" => {
        tokio::task::spawn_blocking(pieuvre_sync::onedrive::uninstall_onedrive).await??;
        Ok(ExecutionResult::ok("OneDrive removed"))
    },
});

impl_exec!(PrivacyExecutor, "privacy", {
    "telemetry_level" => {
        reg_set_dword(r"SOFTWARE\Policies\Microsoft\Windows\DataCollection", "AllowTelemetry", 0).await?;
        reg_set_dword(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection", "AllowTelemetry", 0).await
    },
    "advertising_id" => reg_set_dword(r"SOFTWARE\Microsoft\Windows\CurrentVersion\AdvertisingInfo", "Enabled", 0).await,
    "location" => reg_set_dword(r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location", "Value", 0).await,
    "activity_history" => {
        reg_set_dword(r"SOFTWARE\Policies\Microsoft\Windows\System", "EnableActivityFeed", 0).await?;
        reg_set_dword(r"SOFTWARE\Policies\Microsoft\Windows\System", "PublishUserActivities", 0).await?;
        reg_set_dword(r"SOFTWARE\Policies\Microsoft\Windows\System", "UploadUserActivities", 0).await
    },
    "cortana" => reg_set_dword(r"SOFTWARE\Policies\Microsoft\Windows\Windows Search", "AllowCortana", 0).await,
    "widgets" => reg_set_dword(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "TaskbarDa", 0).await,
    "recall" => {
        reg_set_dword(r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI", "DisableAIDataAnalysis", 1).await?;
        reg_set_dword(r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI", "TurnOffSavingSnapshots", 1).await
    },
    "context_menu" => {
        tokio::task::spawn_blocking(pieuvre_sync::context_menu::remove_context_menu_clutter).await??;
        Ok(ExecutionResult::ok("Classic Context Menu enabled & Clutter removed"))
    },
    "pause_updates" => {
        tokio::task::spawn_blocking(pieuvre_sync::windows_update::pause_updates).await??;
        Ok(ExecutionResult::ok("Windows Updates paused for 35 days"))
    },
    "driver_updates" => {
        tokio::task::spawn_blocking(pieuvre_sync::windows_update::disable_driver_updates).await??;
        Ok(ExecutionResult::ok("Automatic Driver Updates disabled"))
    },
    "group_policy_telem" => {
        tokio::task::spawn_blocking(|| pieuvre_sync::registry::set_group_policy_telemetry(0)).await??;
        Ok(ExecutionResult::ok("Enterprise Telemetry Policy applied"))
    },
});

pub fn get_executor(cat: &str) -> Result<Box<dyn OptExecutor + Send + Sync>> {
    match cat.to_lowercase().as_str() {
        "telemetry" => Ok(Box::new(TelemetryExecutor)),
        "privacy" => Ok(Box::new(PrivacyExecutor)),
        "performance" => Ok(Box::new(PerformanceExecutor)),
        "scheduler" => Ok(Box::new(SchedulerExecutor)),
        "appx" | "appx bloat" => Ok(Box::new(AppxExecutor)),
        "cpu" | "cpu/mem" => Ok(Box::new(CPUExecutor)),
        "dpc" | "dpc latency" => Ok(Box::new(DPCExecutor)),
        "security" => Ok(Box::new(SecurityExecutor)),
        "network" => Ok(Box::new(NetworkExecutor)),
        "dns" => Ok(Box::new(DNSExecutor)),
        "cleanup" => Ok(Box::new(CleanupExecutor)),

        "audit" => Ok(Box::new(AuditExecutor)),
        _ => anyhow::bail!("Unknown category: {}", cat),
    }
}

impl_exec!(PerformanceExecutor, "performance", {
    "timer" => {
        tokio::task::spawn_blocking(|| pieuvre_sync::timer::set_timer_resolution(5000)).await??;
        Ok(ExecutionResult::ok("Timer resolution set to 0.5ms"))
    },
    "power_ultimate" => {
        tokio::task::spawn_blocking(|| pieuvre_sync::power::set_power_plan(pieuvre_sync::power::PowerPlan::UltimatePerformance)).await??;
        Ok(ExecutionResult::ok("Ultimate Performance plan active"))
    },
    "power_high" => {
        tokio::task::spawn_blocking(|| pieuvre_sync::power::set_power_plan(pieuvre_sync::power::PowerPlan::HighPerformance)).await??;
        Ok(ExecutionResult::ok("High Performance plan active"))
    },
    "cpu_throttle" => {
        tokio::task::spawn_blocking(pieuvre_sync::power::disable_cpu_throttling).await??;
        Ok(ExecutionResult::ok("CPU Throttling disabled"))
    },
    "usb_suspend" => {
        tokio::task::spawn_blocking(|| pieuvre_sync::power::configure_power_settings(false, false, 100, 100)).await??;
        Ok(ExecutionResult::ok("USB Selective Suspend disabled"))
    },
    "msi" => {
        let devices = tokio::task::spawn_blocking(pieuvre_sync::msi::list_msi_eligible_devices).await??;
        let mut count = 0;
        for dev in &devices {
            if !dev.msi_enabled && pieuvre_sync::msi::enable_msi(&dev.full_path).is_ok() {
                count += 1;
            }
        }
        Ok(ExecutionResult::ok_count(count, format!("MSI enabled for {} devices", count)))
    },
    "sysmain" => svc_disable("SysMain").await,
    "wsearch" => svc_disable("WSearch").await,
    "edge_disable" => {
        tokio::task::spawn_blocking(pieuvre_sync::edge::disable_edge).await??;
        Ok(ExecutionResult::ok("Edge bloatware disabled"))
    },
    "explorer_tweaks" => {
        tokio::task::spawn_blocking(pieuvre_sync::explorer::apply_explorer_tweaks).await??;
        Ok(ExecutionResult::ok("Explorer optimizations applied"))
    },
    "game_bar" => {
        tokio::task::spawn_blocking(pieuvre_sync::game_mode::disable_game_bar).await??;
        Ok(ExecutionResult::ok("Game Bar & DVR disabled"))
    },
    "fullscreen_opt" => {
        tokio::task::spawn_blocking(pieuvre_sync::game_mode::disable_fullscreen_optimizations).await??;
        Ok(ExecutionResult::ok("Fullscreen optimizations disabled"))
    },
    "hags" => {
        tokio::task::spawn_blocking(pieuvre_sync::game_mode::disable_hags).await??;
        Ok(ExecutionResult::ok("HAGS disabled"))
    },
    "nagle" => {
        let n = tokio::task::spawn_blocking(pieuvre_sync::network::disable_nagle_algorithm).await??;
        Ok(ExecutionResult::ok_count(n as usize, format!("Nagle disabled on {} interfaces", n)))
    },
    "power_throttle" => {
        tokio::task::spawn_blocking(pieuvre_sync::registry::disable_power_throttling).await??;
        Ok(ExecutionResult::ok("Power Throttling disabled"))
    },
    "enable_game_mode" => {
        tokio::task::spawn_blocking(pieuvre_sync::game_mode::enable_game_mode).await??;
        Ok(ExecutionResult::ok("Windows Game Mode enabled"))
    },
    "prerendered_frames" => {
        tokio::task::spawn_blocking(|| pieuvre_sync::game_mode::set_prerendered_frames(1)).await??;
        Ok(ExecutionResult::ok("Pre-rendered frames set to 1"))
    },
    "vrr_opt" => {
        tokio::task::spawn_blocking(pieuvre_sync::game_mode::disable_vrr_optimizations).await??;
        Ok(ExecutionResult::ok("VRR optimizations disabled"))
    },
    "shader_cache" => {
        tokio::task::spawn_blocking(|| pieuvre_sync::game_mode::set_shader_cache_size(256)).await??;
        Ok(ExecutionResult::ok("Shader cache set to 256MB"))
    },
});

impl_exec!(SchedulerExecutor, "scheduler", {
    "priority_sep" => reg_set_dword(r"SYSTEM\CurrentControlSet\Control\PriorityControl", "Win32PrioritySeparation", 0x26).await,
    "mmcss" => {
        tokio::task::spawn_blocking(pieuvre_sync::registry::configure_mmcss_gaming).await??;
        Ok(ExecutionResult::ok("MMCSS gaming profile configured"))
    },
    "games_priority" => {
        tokio::task::spawn_blocking(pieuvre_sync::registry::configure_games_priority).await??;
        Ok(ExecutionResult::ok("GPU & Games priority optimized"))
    },
    "global_timer" => {
        tokio::task::spawn_blocking(pieuvre_sync::registry::enable_global_timer_resolution).await??;
        Ok(ExecutionResult::ok("Global timer resolution enabled (Reboot required)"))
    },
    "startup_delay" => {
        tokio::task::spawn_blocking(pieuvre_sync::registry::disable_startup_delay).await??;
        Ok(ExecutionResult::ok("Startup delay removed"))
    },
    "shutdown_timeout" => {
        tokio::task::spawn_blocking(pieuvre_sync::registry::reduce_shutdown_timeout).await??;
        Ok(ExecutionResult::ok("Shutdown timeout reduced to 2s"))
    },
});

impl_exec!(AppxExecutor, "appx", {
    "bing_apps" => {
        let r = tokio::task::spawn_blocking(pieuvre_sync::appx::remove_bing_apps).await??;
        Ok(ExecutionResult::ok_count(r.len(), format!("Removed {} Bing apps", r.len())))
    },
    "ms_productivity" => {
        let r = tokio::task::spawn_blocking(pieuvre_sync::appx::remove_ms_productivity).await??;
        Ok(ExecutionResult::ok_count(r.len(), format!("Removed {} productivity apps", r.len())))
    },
    "ms_media" => {
        let r = tokio::task::spawn_blocking(pieuvre_sync::appx::remove_ms_media).await??;
        Ok(ExecutionResult::ok_count(r.len(), format!("Removed {} media apps", r.len())))
    },
    "ms_communication" => {
        let r = tokio::task::spawn_blocking(pieuvre_sync::appx::remove_ms_communication).await??;
        Ok(ExecutionResult::ok_count(r.len(), format!("Removed {} communication apps", r.len())))
    },
    "ms_legacy" => {
        let r = tokio::task::spawn_blocking(pieuvre_sync::appx::remove_ms_legacy).await??;
        Ok(ExecutionResult::ok_count(r.len(), format!("Removed {} legacy apps", r.len())))
    },
    "ms_tools" => {
        let r = tokio::task::spawn_blocking(pieuvre_sync::appx::remove_ms_tools).await??;
        Ok(ExecutionResult::ok_count(r.len(), format!("Removed {} tools", r.len())))
    },
    "third_party" => {
        let r = tokio::task::spawn_blocking(pieuvre_sync::appx::remove_third_party).await??;
        Ok(ExecutionResult::ok_count(r.len(), format!("Removed {} third-party apps", r.len())))
    },
    "copilot" => {
        let r = tokio::task::spawn_blocking(pieuvre_sync::appx::remove_copilot).await??;
        Ok(ExecutionResult::ok_count(r.len(), format!("Removed {} Copilot components", r.len())))
    },
    "cortana_app" => {
        let r = tokio::task::spawn_blocking(pieuvre_sync::appx::remove_cortana).await??;
        Ok(ExecutionResult::ok_count(r.len(), format!("Removed {} Cortana components", r.len())))
    },
    "xbox" => {
        let r = tokio::task::spawn_blocking(pieuvre_sync::appx::remove_xbox_packages).await??;
        Ok(ExecutionResult::ok_count(r.len(), format!("Removed {} Xbox apps", r.len())))
    },
});

impl_exec!(CPUExecutor, "cpu", {
    "core_parking" => {
        tokio::task::spawn_blocking(pieuvre_sync::cpu::disable_core_parking).await??;
        Ok(ExecutionResult::ok("Core Parking disabled"))
    },
    "memory_compression" => {
        tokio::task::spawn_blocking(pieuvre_sync::cpu::disable_memory_compression).await??;
        Ok(ExecutionResult::ok("Memory Compression disabled"))
    },
    "superfetch_registry" => {
        tokio::task::spawn_blocking(pieuvre_sync::cpu::disable_superfetch_registry).await??;
        Ok(ExecutionResult::ok("Superfetch/Prefetch registry tweaks applied"))
    },
    "static_pagefile" => {
        tokio::task::spawn_blocking(|| pieuvre_sync::cpu::set_static_page_file(16384)).await??;
        Ok(ExecutionResult::ok("Static Page File set to 16GB"))
    },
});

impl_exec!(DPCExecutor, "dpc", {
    "paging_executive" => {
        tokio::task::spawn_blocking(pieuvre_sync::dpc::disable_paging_executive).await??;
        Ok(ExecutionResult::ok("DisablePagingExecutive enabled"))
    },
    "dynamic_tick" => {
        tokio::task::spawn_blocking(pieuvre_sync::dpc::disable_dynamic_tick).await??;
        Ok(ExecutionResult::ok("Dynamic Tick disabled (Reboot required)"))
    },
    "tsc_sync" => {
        tokio::task::spawn_blocking(pieuvre_sync::dpc::set_tsc_sync_enhanced).await??;
        Ok(ExecutionResult::ok("TSC Sync set to Enhanced"))
    },
    "hpet" => {
        tokio::task::spawn_blocking(pieuvre_sync::dpc::disable_hpet).await??;
        Ok(ExecutionResult::ok("HPET disabled"))
    },
    "interrupt_affinity" => {
        tokio::task::spawn_blocking(pieuvre_sync::dpc::set_interrupt_affinity_spread).await??;
        Ok(ExecutionResult::ok("Interrupt Affinity spread across cores"))
    },
});

impl_exec!(SecurityExecutor, "security", {
    "hvci" => {
        tokio::task::spawn_blocking(pieuvre_sync::security::disable_memory_integrity).await??;
        Ok(ExecutionResult::ok("HVCI disabled (Reboot required)"))
    },
    "vbs" => {
        tokio::task::spawn_blocking(pieuvre_sync::security::disable_vbs).await??;
        Ok(ExecutionResult::ok("VBS disabled (Reboot required)"))
    },
    "spectre" => {
        tokio::task::spawn_blocking(pieuvre_sync::security::disable_spectre_meltdown).await??;
        Ok(ExecutionResult::ok("Spectre/Meltdown mitigations disabled (RISK)"))
    },
    "defender_realtime" => svc_disable("WinDefend").await,
    "uac_level" => reg_set_dword(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System", "ConsentPromptBehaviorAdmin", 0).await,
    "firewall_status" => svc_disable("mpssvc").await,
    "smartscreen" => {
        tokio::task::spawn_blocking(|| pieuvre_sync::registry::set_string_value(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer", "SmartScreenEnabled", "Off")).await??;
        Ok(ExecutionResult::ok("SmartScreen disabled"))
    },
});

impl_exec!(NetworkExecutor, "network", {
    "interrupt_moderation" => {
        let n = tokio::task::spawn_blocking(pieuvre_sync::network::disable_interrupt_moderation).await??;
        Ok(ExecutionResult::ok_count(n as usize, format!("Disabled on {} interfaces", n)))
    },
    "lso" => {
        tokio::task::spawn_blocking(pieuvre_sync::network::disable_lso).await??;
        Ok(ExecutionResult::ok("Large Send Offload disabled"))
    },
    "eee" => {
        tokio::task::spawn_blocking(pieuvre_sync::network::disable_eee).await??;
        Ok(ExecutionResult::ok("Energy Efficient Ethernet disabled"))
    },
    "rss" => {
        tokio::task::spawn_blocking(pieuvre_sync::network::enable_rss).await??;
        Ok(ExecutionResult::ok("Receive Side Scaling enabled"))
    },
    "rsc" => {
        tokio::task::spawn_blocking(pieuvre_sync::network::disable_rsc).await??;
        Ok(ExecutionResult::ok("Receive Segment Coalescing disabled"))
    },
});

impl_exec!(DNSExecutor, "dns", {
    "doh_cloudflare" => {
        tokio::task::spawn_blocking(|| pieuvre_sync::dns::set_doh_provider(pieuvre_sync::dns::DNSProvider::Cloudflare)).await??;
        Ok(ExecutionResult::ok("DoH (Cloudflare) enabled"))
    },
    "doh_google" => {
        tokio::task::spawn_blocking(|| pieuvre_sync::dns::set_doh_provider(pieuvre_sync::dns::DNSProvider::Google)).await??;
        Ok(ExecutionResult::ok("DoH (Google) enabled"))
    },
    "doh_quad9" => {
        tokio::task::spawn_blocking(|| pieuvre_sync::dns::set_doh_provider(pieuvre_sync::dns::DNSProvider::Quad9)).await??;
        Ok(ExecutionResult::ok("DoH (Quad9) enabled"))
    },
    "dns_flush" => {
        tokio::task::spawn_blocking(pieuvre_sync::dns::flush_dns_cache).await??;
        Ok(ExecutionResult::ok("DNS cache flushed"))
    },
});

impl_exec!(CleanupExecutor, "cleanup", {
    "cleanup_temp" => {
        tokio::task::spawn_blocking(pieuvre_sync::cleanup::cleanup_temp_files).await??;
        Ok(ExecutionResult::ok("Temporary files cleaned"))
    },
    "cleanup_winsxs" => {
        tokio::task::spawn_blocking(pieuvre_sync::cleanup::cleanup_winsxs).await??;
        Ok(ExecutionResult::ok("WinSxS cleanup completed"))
    },
    "cleanup_edge" => {
        tokio::task::spawn_blocking(pieuvre_sync::cleanup::cleanup_edge_cache).await??;
        Ok(ExecutionResult::ok("Edge cache cleaned"))
    },
});

impl_exec!(AuditExecutor, "audit", {
    "audit_hardware" => {
        let hw = tokio::task::spawn_blocking(pieuvre_audit::hardware::probe_hardware).await??;
        Ok(ExecutionResult::ok(format!("CPU: {} | RAM: {}GB", hw.cpu.model_name, hw.memory.total_bytes / 1024 / 1024 / 1024)))
    },
    "audit_security" => {
        let sec = tokio::task::spawn_blocking(pieuvre_audit::security::run_security_audit).await??;
        Ok(ExecutionResult::ok(format!("SecureBoot: {} | Defender: {}", sec.secure_boot, sec.defender_enabled)))
    },
    "audit_services" => {
        let svcs = tokio::task::spawn_blocking(pieuvre_audit::services::inspect_services).await??;
        Ok(ExecutionResult::ok(format!("Total Services: {}", svcs.len())))
    },
    "audit_network" => {
        let net = tokio::task::spawn_blocking(pieuvre_audit::network::inspect_network).await??;
        Ok(ExecutionResult::ok(format!("Endpoints: {}", net.telemetry_endpoints.len())))
    },
    "audit_software" => {
        let apps = tokio::task::spawn_blocking(pieuvre_audit::appx::scan_packages).await??;
        Ok(ExecutionResult::ok(format!("AppX Packages: {}", apps.len())))
    },
});
