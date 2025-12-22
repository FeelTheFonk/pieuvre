//! Définition des sections et options pour le mode interactif
//!
//! Module SOTA 2026: Structure modulaire avec types explicites.

use serde::{Deserialize, Serialize};

/// Option d'optimisation avec métadonnées
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OptItem {
    /// Identifiant unique de l'option
    pub id: &'static str,
    /// Label affiché à l'utilisateur
    pub label: &'static str,
    /// Sélectionné par défaut
    pub default: bool,
    /// Niveau de risque
    pub risk: RiskLevel,
}

/// Niveau de risque d'une option
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Sans risque, recommandé
    Safe,
    /// Conditionnel, dépend du contexte
    Conditional,
    /// Performance, peut impacter batterie
    Performance,
    /// Attention requise
    Warning,
    /// Risque critique - sécurité système compromise
    Critical,
}

impl OptItem {
    /// Crée une option safe par défaut activée
    pub const fn safe(id: &'static str, label: &'static str) -> Self {
        Self {
            id,
            label,
            default: true,
            risk: RiskLevel::Safe,
        }
    }

    /// Crée une option safe par défaut désactivée
    pub const fn safe_off(id: &'static str, label: &'static str) -> Self {
        Self {
            id,
            label,
            default: false,
            risk: RiskLevel::Safe,
        }
    }

    /// Crée une option conditionnelle
    pub const fn conditional(id: &'static str, label: &'static str, default: bool) -> Self {
        Self {
            id,
            label,
            default,
            risk: RiskLevel::Conditional,
        }
    }

    /// Crée une option performance
    pub const fn perf(id: &'static str, label: &'static str) -> Self {
        Self {
            id,
            label,
            default: true,
            risk: RiskLevel::Performance,
        }
    }

    /// Crée une option warning (laptop)
    pub const fn warning(id: &'static str, label: &'static str) -> Self {
        Self {
            id,
            label,
            default: false,
            risk: RiskLevel::Warning,
        }
    }

    /// Crée une option critique (sécurité système)
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
// SECTION 1: TÉLÉMÉTRIE
// ============================================================================

/// Retourne les options de la section Télémétrie
pub fn telemetry_section() -> Vec<OptItem> {
    vec![
        OptItem::safe("diagtrack", "DiagTrack - Telemetrie principale"),
        OptItem::safe("dmwappush", "dmwappushservice - Push WAP"),
        OptItem::safe("wersvc", "WerSvc - Windows Error Reporting"),
        OptItem::safe("wercplsupport", "wercplsupport - Error Reports support"),
        OptItem::safe_off("pcasvc", "PcaSvc - Program Compatibility"),
        OptItem::safe_off("wdisystem", "WdiSystemHost - Diagnostic Host"),
        OptItem::safe_off("wdiservice", "WdiServiceHost - Diagnostic Service"),
        OptItem::conditional("lfsvc", "lfsvc - Geolocation", true),
        OptItem::safe("mapsbroker", "MapsBroker - Maps download"),
        OptItem::safe("firewall", "Firewall - Bloquer domaines telemetrie"),
        OptItem::safe(
            "sched_tasks",
            "Scheduled Tasks - Desactiver taches telemetrie",
        ),
        OptItem::safe_off("hosts", "Hosts file - Bloquer domaines DNS natif"),
        OptItem::conditional("onedrive", "OneDrive - Desinstaller completement", false),
    ]
}

// ============================================================================
// SECTION 2: PRIVACY
// ============================================================================

/// Retourne les options de la section Privacy
pub fn privacy_section() -> Vec<OptItem> {
    vec![
        OptItem::safe("telemetry_level", "Telemetry Level 0 (Security only)"),
        OptItem::safe("advertising_id", "Desactiver Advertising ID"),
        OptItem::safe("location", "Desactiver Localisation"),
        OptItem::safe("activity_history", "Desactiver Historique activite"),
        OptItem::safe("cortana", "Desactiver Cortana"),
        OptItem::safe("context_menu", "Classic context menu + nettoyer clutter"),
        OptItem::safe("widgets", "Desactiver Widgets Win11"),
        OptItem::conditional("pause_updates", "Pause Windows Updates 35 jours", false),
        OptItem::conditional("driver_updates", "Desactiver maj drivers auto", false),
        OptItem::safe("recall", "Bloquer Windows Recall (24H2 AI)"),
        OptItem::safe("group_policy_telem", "Group Policy Telemetry (enterprise)"),
    ]
}

// ============================================================================
// SECTION 3: PERFORMANCE
// ============================================================================

/// Retourne les options de la section Performance
/// `is_laptop` adapte les options pour batterie
pub fn performance_section(is_laptop: bool) -> Vec<OptItem> {
    let mut opts = Vec::with_capacity(20);

    // Timer 0.5ms
    if is_laptop {
        opts.push(OptItem::warning("timer", "Timer 0.5ms - Impact batterie"));
    } else {
        opts.push(OptItem::safe("timer", "Timer Resolution 0.5ms (Input lag)"));
    }

    // Power Plan
    if is_laptop {
        opts.push(OptItem::warning(
            "power_ultimate",
            "Ultimate Performance - Usure batterie",
        ));
        opts.push(OptItem::safe(
            "power_high",
            "High Performance - Recommande laptop",
        ));
    } else {
        opts.push(OptItem::safe(
            "power_ultimate",
            "Power Plan: Ultimate Performance",
        ));
    }

    // Options communes
    opts.extend([
        OptItem::perf("cpu_throttle", "Desactiver CPU Throttling"),
        OptItem::perf("usb_suspend", "Desactiver USB Selective Suspend"),
        OptItem::safe("msi", "Activer MSI (Message Signaled Interrupts)"),
        OptItem::safe("sysmain", "Desactiver SysMain (Superfetch)"),
        OptItem::conditional("wsearch", "Desactiver Windows Search Indexer", false),
        OptItem::safe("edge_disable", "Desactiver Edge background/bloat"),
        OptItem::safe("explorer_tweaks", "Optimisations Windows Explorer"),
        OptItem::safe("game_bar", "Desactiver Game Bar & DVR"),
        OptItem::perf("fullscreen_opt", "Desactiver Fullscreen Optimizations"),
        OptItem::conditional("hags", "Desactiver HAGS (Hardware GPU Scheduling)", false),
        OptItem::perf("nagle", "Desactiver Nagle Algorithm (Network Latency)"),
        OptItem::perf("power_throttle", "Desactiver Power Throttling"),
    ]);

    // GPU avancé - input lag minimal
    opts.extend([
        OptItem::safe("enable_game_mode", "Activer Windows Game Mode"),
        OptItem::perf("prerendered_frames", "Low Latency: Pre-rendered frames = 1"),
        OptItem::conditional("vrr_opt", "Desactiver VRR Optimizations", false),
        OptItem::safe_off("shader_cache", "Shader Cache: 256MB (Stutter reduction)"),
    ]);

    opts
}

// ============================================================================
// SECTION 4: SCHEDULER
// ============================================================================

/// Retourne les options de la section Scheduler
pub fn scheduler_section() -> Vec<OptItem> {
    vec![
        OptItem::perf("priority_sep", "Win32PrioritySeparation 0x26 (Fixed)"),
        OptItem::perf("mmcss", "MMCSS Gaming Profile (Priority 6)"),
        OptItem::perf("games_priority", "GPU Priority 8 / Scheduling 6"),
        OptItem::perf("global_timer", "Global Timer Resolution (Reboot)"),
        OptItem::safe("startup_delay", "Desactiver Startup Delay (0ms)"),
        OptItem::safe("shutdown_timeout", "Shutdown Timeout 2000ms"),
    ]
}

// ============================================================================
// SECTION 5: APPX BLOATWARE
// ============================================================================

/// Retourne les options de la section AppX
pub fn appx_section() -> Vec<OptItem> {
    vec![
        OptItem::safe("bing_apps", "Apps Bing (News, Weather, Finance, Sports)"),
        OptItem::safe(
            "ms_productivity",
            "Apps productivite (Todos, People, OfficeHub)",
        ),
        OptItem::safe("ms_media", "Apps media (ZuneMusic, ZuneVideo, Clipchamp)"),
        OptItem::conditional("ms_communication", "Mail/Calendar, Skype, Teams", false),
        OptItem::safe("ms_legacy", "Apps legacy (Paint3D, 3DBuilder, Print3D)"),
        OptItem::safe("ms_tools", "Outils (FeedbackHub, GetHelp, QuickAssist)"),
        OptItem::safe("third_party", "Third-party (Spotify, Disney+, Facebook)"),
        OptItem::conditional("copilot", "Microsoft Copilot - Desactiver AI", false),
        OptItem::safe("cortana_app", "Cortana"),
        OptItem::conditional("xbox", "Xbox apps (attention Game Pass)", false),
    ]
}

// ============================================================================
// SECTION 6: CPU / MEMORY
// ============================================================================

/// Retourne les options de la section CPU/Memory
/// `is_laptop` adapte les options pour batterie
pub fn cpu_section(is_laptop: bool) -> Vec<OptItem> {
    let mut opts = Vec::with_capacity(5);

    if is_laptop {
        opts.push(OptItem::warning(
            "core_parking",
            "Desactiver Core Parking - Impact batterie",
        ));
    } else {
        opts.push(OptItem::safe(
            "core_parking",
            "Desactiver Core Parking - Tous cores actifs",
        ));
    }

    opts.extend([
        OptItem::safe_off(
            "memory_compression",
            "Desactiver Memory Compression (16GB+ RAM)",
        ),
        OptItem::safe_off(
            "superfetch_registry",
            "Desactiver Superfetch/Prefetch via registre",
        ),
        OptItem::conditional("static_pagefile", "Page File statique (1.5x RAM)", false),
    ]);

    opts
}

// ============================================================================
// SECTION 7: DPC LATENCY
// ============================================================================

/// Retourne les options de la section DPC Latency (micro-stuttering)
pub fn dpc_section() -> Vec<OptItem> {
    vec![
        OptItem::perf("paging_executive", "DisablePagingExecutive - Kernel en RAM"),
        OptItem::conditional(
            "dynamic_tick",
            "Desactiver Dynamic Tick - Reboot requis",
            false,
        ),
        OptItem::perf("tsc_sync", "TSC Sync Enhanced - Precision timer"),
        OptItem::conditional("hpet", "Desactiver HPET - Tester avec LatencyMon", false),
        OptItem::safe_off(
            "interrupt_affinity",
            "Interrupt Affinity Spread - Distribution cores",
        ),
    ]
}

// ============================================================================
// SECTION 8: SECURITY (CAUTION)
// ============================================================================

/// Retourne les options de la section Security
/// WARNING: Options à risque de sécurité - systèmes gaming isolés uniquement
pub fn security_section() -> Vec<OptItem> {
    vec![
        OptItem::warning("hvci", "Memory Integrity (HVCI) - Off (+5% FPS)"),
        OptItem::warning("vbs", "Virtualization Based Security (VBS) - Off"),
        OptItem::critical("spectre", "Spectre/Meltdown Mitigations - Off (RISK)"),
    ]
}

// ============================================================================
// SECTION 9: NETWORK AVANCÉ
// ============================================================================

/// Retourne les options de la section Network Avancé
pub fn network_advanced_section() -> Vec<OptItem> {
    vec![
        OptItem::perf(
            "interrupt_moderation",
            "Desactiver Interrupt Moderation - Latence",
        ),
        OptItem::safe_off("lso", "Desactiver Large Send Offload (LSO)"),
        OptItem::safe_off("eee", "Desactiver Energy Efficient Ethernet"),
        OptItem::safe("rss", "Activer Receive Side Scaling (RSS)"),
        OptItem::safe_off("rsc", "Desactiver Receive Segment Coalescing"),
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
