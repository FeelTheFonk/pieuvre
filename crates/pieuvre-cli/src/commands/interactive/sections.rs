//! Section and option definitions for interactive mode
//!
//! 2026 Module: Modular structure with explicit types.

use serde::{Deserialize, Serialize};

/// Optimization option with metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OptItem {
    /// Unique identifier for the option
    pub id: &'static str,
    /// Label displayed to the user
    pub label: &'static str,
    /// Selected by default
    pub default: bool,
    /// Risk level
    pub risk: RiskLevel,
}

/// Risk level of an option
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    /// No risk, recommended
    Safe,
    /// Conditional, depends on context
    Conditional,
    /// Performance, may impact battery
    Performance,
    /// Caution required
    Warning,
    /// Critical risk - system security compromised
    Critical,
}

impl OptItem {
    /// Creates a safe option enabled by default
    pub const fn safe(id: &'static str, label: &'static str) -> Self {
        Self {
            id,
            label,
            default: true,
            risk: RiskLevel::Safe,
        }
    }

    /// Creates a safe option disabled by default
    pub const fn safe_off(id: &'static str, label: &'static str) -> Self {
        Self {
            id,
            label,
            default: false,
            risk: RiskLevel::Safe,
        }
    }

    /// Creates a conditional option
    pub const fn conditional(id: &'static str, label: &'static str, default: bool) -> Self {
        Self {
            id,
            label,
            default,
            risk: RiskLevel::Conditional,
        }
    }

    /// Creates a performance option
    pub const fn perf(id: &'static str, label: &'static str) -> Self {
        Self {
            id,
            label,
            default: true,
            risk: RiskLevel::Performance,
        }
    }

    /// Creates a warning option (laptop)
    pub const fn warning(id: &'static str, label: &'static str) -> Self {
        Self {
            id,
            label,
            default: false,
            risk: RiskLevel::Warning,
        }
    }

    /// Creates a critical option (system security)
    pub const fn critical(id: &'static str, label: &'static str) -> Self {
        Self {
            id,
            label,
            default: false,
            risk: RiskLevel::Critical,
        }
    }
}

// ============================================================================
// SECTION 1: TELEMETRY
// ============================================================================

/// Returns options for the Telemetry section
pub fn telemetry_section() -> Vec<OptItem> {
    vec![
        OptItem::safe("diagtrack", "DiagTrack - Main Telemetry"),
        OptItem::safe("dmwappush", "dmwappushservice - WAP Push"),
        OptItem::safe("wersvc", "WerSvc - Windows Error Reporting"),
        OptItem::safe("wercplsupport", "wercplsupport - Error Reports support"),
        OptItem::safe_off("pcasvc", "PcaSvc - Program Compatibility"),
        OptItem::safe_off("wdisystem", "WdiSystemHost - Diagnostic Host"),
        OptItem::safe_off("wdiservice", "WdiServiceHost - Diagnostic Service"),
        OptItem::conditional("lfsvc", "lfsvc - Geolocation", true),
        OptItem::safe("mapsbroker", "MapsBroker - Maps download"),
        OptItem::safe("firewall", "Firewall - Block telemetry domains"),
        OptItem::safe("sched_tasks", "Scheduled Tasks - Disable telemetry tasks"),
        OptItem::safe_off("hosts", "Hosts file - Block native DNS domains"),
        OptItem::conditional("onedrive", "OneDrive - Uninstall completely", false),
    ]
}

// ============================================================================
// SECTION 2: PRIVACY
// ============================================================================

/// Returns options for the Privacy section
pub fn privacy_section() -> Vec<OptItem> {
    vec![
        OptItem::safe("telemetry_level", "Telemetry Level 0 (Security only)"),
        OptItem::safe("advertising_id", "Disable Advertising ID"),
        OptItem::safe("location", "Disable Location"),
        OptItem::safe("activity_history", "Disable Activity History"),
        OptItem::safe("cortana", "Disable Cortana"),
        OptItem::safe("context_menu", "Classic context menu + clean clutter"),
        OptItem::safe("widgets", "Disable Win11 Widgets"),
        OptItem::conditional("pause_updates", "Pause Windows Updates 35 days", false),
        OptItem::conditional("driver_updates", "Disable auto driver updates", false),
        OptItem::safe("recall", "Block Windows Recall (24H2 AI)"),
        OptItem::safe("group_policy_telem", "Group Policy Telemetry (enterprise)"),
    ]
}

// ============================================================================
// SECTION 3: PERFORMANCE
// ============================================================================

/// Returns options for the Performance section
/// `is_laptop` adapts options for battery
pub fn performance_section(is_laptop: bool) -> Vec<OptItem> {
    let mut opts = Vec::with_capacity(20);

    // Timer 0.5ms
    if is_laptop {
        opts.push(OptItem::warning("timer", "Timer 0.5ms - Battery impact"));
    } else {
        opts.push(OptItem::safe("timer", "Timer Resolution 0.5ms (Input lag)"));
    }

    // Power Plan
    if is_laptop {
        opts.push(OptItem::warning(
            "power_ultimate",
            "Ultimate Performance - Battery wear",
        ));
        opts.push(OptItem::safe(
            "power_high",
            "High Performance - Recommended for laptop",
        ));
    } else {
        opts.push(OptItem::safe(
            "power_ultimate",
            "Power Plan: Ultimate Performance",
        ));
    }

    // Common options
    opts.extend([
        OptItem::perf("cpu_throttle", "Disable CPU Throttling"),
        OptItem::perf("usb_suspend", "Disable USB Selective Suspend"),
        OptItem::safe("msi", "Enable MSI (Message Signaled Interrupts)"),
        OptItem::safe("sysmain", "Disable SysMain (Superfetch)"),
        OptItem::conditional("wsearch", "Disable Windows Search Indexer", false),
        OptItem::safe("edge_disable", "Disable Edge background/bloat"),
        OptItem::safe("explorer_tweaks", "Windows Explorer optimizations"),
        OptItem::safe("game_bar", "Disable Game Bar & DVR"),
        OptItem::perf("fullscreen_opt", "Disable Fullscreen Optimizations"),
        OptItem::conditional("hags", "Disable HAGS (Hardware GPU Scheduling)", false),
        OptItem::perf("nagle", "Disable Nagle Algorithm (Network Latency)"),
        OptItem::perf("power_throttle", "Disable Power Throttling"),
    ]);

    // Advanced GPU - minimal input lag
    opts.extend([
        OptItem::safe("enable_game_mode", "Enable Windows Game Mode"),
        OptItem::perf("prerendered_frames", "Low Latency: Pre-rendered frames = 1"),
        OptItem::conditional("vrr_opt", "Disable VRR Optimizations", false),
        OptItem::safe_off("shader_cache", "Shader Cache: 256MB (Stutter reduction)"),
    ]);

    opts
}

// ============================================================================
// SECTION 4: SCHEDULER
// ============================================================================

/// Returns options for the Scheduler section
pub fn scheduler_section() -> Vec<OptItem> {
    vec![
        OptItem::perf("priority_sep", "Win32PrioritySeparation 0x26 (Fixed)"),
        OptItem::perf("mmcss", "MMCSS Gaming Profile (Priority 6)"),
        OptItem::perf("games_priority", "GPU Priority 8 / Scheduling 6"),
        OptItem::perf("global_timer", "Global Timer Resolution (Reboot)"),
        OptItem::safe("startup_delay", "Disable Startup Delay (0ms)"),
        OptItem::safe("shutdown_timeout", "Shutdown Timeout 2000ms"),
    ]
}

// ============================================================================
// SECTION 5: APPX BLOATWARE
// ============================================================================

/// Returns options for the AppX section
pub fn appx_section() -> Vec<OptItem> {
    vec![
        OptItem::safe("bing_apps", "Bing Apps (News, Weather, Finance, Sports)"),
        OptItem::safe(
            "ms_productivity",
            "Productivity Apps (Todos, People, OfficeHub)",
        ),
        OptItem::safe("ms_media", "Media Apps (ZuneMusic, ZuneVideo, Clipchamp)"),
        OptItem::conditional("ms_communication", "Mail/Calendar, Skype, Teams", false),
        OptItem::safe("ms_legacy", "Legacy Apps (Paint3D, 3DBuilder, Print3D)"),
        OptItem::safe("ms_tools", "Tools (FeedbackHub, GetHelp, QuickAssist)"),
        OptItem::safe("third_party", "Third-party (Spotify, Disney+, Facebook)"),
        OptItem::conditional("copilot", "Microsoft Copilot - Disable AI", false),
        OptItem::safe("cortana_app", "Cortana"),
        OptItem::conditional("xbox", "Xbox apps (caution Game Pass)", false),
    ]
}

// ============================================================================
// SECTION 6: CPU / MEMORY
// ============================================================================

/// Returns options for the CPU/Memory section
/// `is_laptop` adapts options for battery
pub fn cpu_section(is_laptop: bool) -> Vec<OptItem> {
    let mut opts = Vec::with_capacity(5);

    if is_laptop {
        opts.push(OptItem::warning(
            "core_parking",
            "Disable Core Parking - Battery impact",
        ));
    } else {
        opts.push(OptItem::safe(
            "core_parking",
            "Disable Core Parking - All cores active",
        ));
    }

    opts.extend([
        OptItem::safe_off(
            "memory_compression",
            "Disable Memory Compression (16GB+ RAM)",
        ),
        OptItem::safe_off(
            "superfetch_registry",
            "Disable Superfetch/Prefetch via registry",
        ),
        OptItem::conditional("static_pagefile", "Static Page File (1.5x RAM)", false),
    ]);

    opts
}

// ============================================================================
// SECTION 7: DPC LATENCY
// ============================================================================

/// Returns options for the DPC Latency section (micro-stuttering)
pub fn dpc_section() -> Vec<OptItem> {
    vec![
        OptItem::perf("paging_executive", "DisablePagingExecutive - Kernel in RAM"),
        OptItem::conditional(
            "dynamic_tick",
            "Disable Dynamic Tick - Reboot required",
            false,
        ),
        OptItem::perf("tsc_sync", "TSC Sync Enhanced - Precision timer"),
        OptItem::conditional("hpet", "Disable HPET - Test with LatencyMon", false),
        OptItem::safe_off(
            "interrupt_affinity",
            "Interrupt Affinity Spread - Core distribution",
        ),
    ]
}

// ============================================================================
// SECTION 8: SECURITY (CAUTION)
// ============================================================================

/// Returns options for the Security section
/// WARNING: Security risk options - isolated gaming systems only
pub fn security_section() -> Vec<OptItem> {
    vec![
        OptItem::warning("hvci", "Memory Integrity (HVCI) - Off (+5% FPS)"),
        OptItem::warning("vbs", "Virtualization Based Security (VBS) - Off"),
        OptItem::critical("spectre", "Spectre/Meltdown Mitigations - Off (RISK)"),
    ]
}

// ============================================================================
// SECTION 9: ADVANCED NETWORK
// ============================================================================

/// Returns options for the Advanced Network section
pub fn network_advanced_section() -> Vec<OptItem> {
    vec![
        OptItem::perf(
            "interrupt_moderation",
            "Disable Interrupt Moderation - Latency",
        ),
        OptItem::safe_off("lso", "Disable Large Send Offload (LSO)"),
        OptItem::safe_off("eee", "Disable Energy Efficient Ethernet"),
        OptItem::safe("rss", "Enable Receive Side Scaling (RSS)"),
        OptItem::safe_off("rsc", "Disable Receive Segment Coalescing"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telemetry_section_not_empty() {
        let section = telemetry_section();
        assert!(!section.is_empty());
    }

    #[test]
    fn test_all_sections_have_unique_ids() {
        let mut all_ids = Vec::new();
        all_ids.extend(telemetry_section().iter().map(|o| o.id));
        all_ids.extend(privacy_section().iter().map(|o| o.id));
        all_ids.extend(performance_section(false).iter().map(|o| o.id));
        all_ids.extend(scheduler_section().iter().map(|o| o.id));
        all_ids.extend(appx_section().iter().map(|o| o.id));
        all_ids.extend(cpu_section(false).iter().map(|o| o.id));
        all_ids.extend(dpc_section().iter().map(|o| o.id));
        all_ids.extend(security_section().iter().map(|o| o.id));
        all_ids.extend(network_advanced_section().iter().map(|o| o.id));

        let unique_count = {
            let mut sorted = all_ids.clone();
            sorted.sort();
            sorted.dedup();
            sorted.len()
        };

        assert_eq!(
            all_ids.len(),
            unique_count,
            "Duplicate IDs found in sections"
        );
    }
}
