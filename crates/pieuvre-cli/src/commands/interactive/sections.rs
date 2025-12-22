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
}

impl OptItem {
    /// Crée une option safe par défaut activée
    pub const fn safe(id: &'static str, label: &'static str) -> Self {
        Self { id, label, default: true, risk: RiskLevel::Safe }
    }

    /// Crée une option safe par défaut désactivée
    pub const fn safe_off(id: &'static str, label: &'static str) -> Self {
        Self { id, label, default: false, risk: RiskLevel::Safe }
    }

    /// Crée une option conditionnelle
    pub const fn conditional(id: &'static str, label: &'static str, default: bool) -> Self {
        Self { id, label, default, risk: RiskLevel::Conditional }
    }

    /// Crée une option performance
    pub const fn perf(id: &'static str, label: &'static str) -> Self {
        Self { id, label, default: true, risk: RiskLevel::Performance }
    }

    /// Crée une option warning (laptop)
    pub const fn warning(id: &'static str, label: &'static str) -> Self {
        Self { id, label, default: false, risk: RiskLevel::Warning }
    }
}

// ============================================================================
// SECTION 1: TÉLÉMÉTRIE
// ============================================================================

/// Retourne les options de la section Télémétrie
pub fn telemetry_section() -> Vec<OptItem> {
    vec![
        OptItem::safe("diagtrack", "[SAFE] DiagTrack - Télémétrie principale"),
        OptItem::safe("dmwappush", "[SAFE] dmwappushservice - Push WAP"),
        OptItem::safe("wersvc", "[SAFE] WerSvc - Windows Error Reporting"),
        OptItem::safe("wercplsupport", "[SAFE] wercplsupport - Error Reports support"),
        OptItem::safe_off("pcasvc", "[SAFE] PcaSvc - Program Compatibility"),
        OptItem::safe_off("wdisystem", "[SAFE] WdiSystemHost - Diagnostic Host"),
        OptItem::safe_off("wdiservice", "[SAFE] WdiServiceHost - Diagnostic Service"),
        OptItem::conditional("lfsvc", "[COND] lfsvc - Geolocation", true),
        OptItem::safe("mapsbroker", "[SAFE] MapsBroker - Maps download"),
        OptItem::safe("firewall", "[SAFE] Firewall - Bloquer 47 domaines télémétrie"),
        OptItem::safe("sched_tasks", "[SAFE] Scheduled Tasks - Désactiver 25 tâches télémétrie"),
        OptItem::safe_off("hosts", "[SAFE] Hosts file - Bloquer 50+ domaines DNS natif"),
        OptItem::conditional("onedrive", "[COND] OneDrive - Désinstaller complètement", false),
    ]
}

// ============================================================================
// SECTION 2: PRIVACY
// ============================================================================

/// Retourne les options de la section Privacy
pub fn privacy_section() -> Vec<OptItem> {
    vec![
        OptItem::safe("telemetry_level", "[SAFE] Telemetry Level 0 (Security only)"),
        OptItem::safe("advertising_id", "[SAFE] Désactiver Advertising ID"),
        OptItem::safe("location", "[SAFE] Désactiver Localisation"),
        OptItem::safe("activity_history", "[SAFE] Désactiver Historique activité"),
        OptItem::safe("cortana", "[SAFE] Désactiver Cortana"),
        OptItem::safe("context_menu", "[SAFE] Classic context menu + nettoyer clutter"),
        OptItem::safe("widgets", "[SAFE] Désactiver Widgets Win11"),
        OptItem::conditional("pause_updates", "[COND] Pause Windows Updates 35 jours", false),
        OptItem::conditional("driver_updates", "[COND] Désactiver maj drivers auto", false),
        OptItem::safe("recall", "[SAFE] Bloquer Windows Recall (24H2 AI)"),
        OptItem::safe("group_policy_telem", "[SAFE] Group Policy Telemetry (enterprise)"),
    ]
}

// ============================================================================
// SECTION 3: PERFORMANCE
// ============================================================================

/// Retourne les options de la section Performance
/// `is_laptop` adapte les options pour batterie
pub fn performance_section(is_laptop: bool) -> Vec<OptItem> {
    let mut opts = Vec::with_capacity(15);

    // Timer 0.5ms
    if is_laptop {
        opts.push(OptItem::warning("timer", "[WARN][LAPTOP] Timer 0.5ms - +25% conso batterie"));
    } else {
        opts.push(OptItem::safe("timer", "[SAFE] Timer 0.5ms - Latence réduite (gaming)"));
    }

    // Power Plan
    if is_laptop {
        opts.push(OptItem::warning("power_ultimate", "[WARN][LAPTOP] Ultimate Performance - Usure batterie"));
        opts.push(OptItem::safe("power_high", "[SAFE] High Performance - Recommandé laptop"));
    } else {
        opts.push(OptItem::safe("power_ultimate", "[SAFE] Ultimate Performance - Max performance desktop"));
    }

    // Options communes
    opts.extend([
        OptItem::perf("cpu_throttle", "[PERF] Désactiver CPU Throttling"),
        OptItem::perf("usb_suspend", "[PERF] Désactiver USB Selective Suspend"),
        OptItem::safe("msi", "[SAFE] Activer MSI Mode GPU/NVMe"),
        OptItem::safe("sysmain", "[SAFE] Désactiver SysMain (SSD recommandé)"),
        OptItem::conditional("wsearch", "[COND] Désactiver WSearch - Recherche plus lente", false),
        OptItem::safe("edge_disable", "[SAFE] Désactiver features Edge (sidebar, DVR, shopping)"),
        OptItem::safe("explorer_tweaks", "[SAFE] Explorer tweaks (extensions, This PC, no recent)"),
        OptItem::safe("game_bar", "[SAFE] Désactiver Game Bar/DVR (alt+tab plus rapide)"),
        OptItem::perf("fullscreen_opt", "[PERF] Désactiver Fullscreen Optimizations"),
        OptItem::conditional("hags", "[COND] Désactiver HAGS (meilleur pour anciens jeux)", false),
        OptItem::perf("nagle", "[PERF] Désactiver Nagle Algorithm (latence réseau)"),
        OptItem::perf("power_throttle", "[PERF] Désactiver CPU Power Throttling"),
    ]);

    opts
}

// ============================================================================
// SECTION 4: SCHEDULER
// ============================================================================

/// Retourne les options de la section Scheduler
pub fn scheduler_section() -> Vec<OptItem> {
    vec![
        OptItem::safe("priority_sep", "[SAFE] Win32PrioritySeparation 0x26 - Short quantum, foreground boost"),
        OptItem::safe("mmcss", "[SAFE] MMCSS Gaming - SystemResponsiveness 10%, Network throttling OFF"),
        OptItem::safe("games_priority", "[SAFE] GPU Priority 8, Task Priority 6 - Gaming boost"),
        OptItem::perf("global_timer", "[PERF] Timer resolution permanente - Reboot recommandé"),
        OptItem::safe("startup_delay", "[SAFE] Désactiver délai startup apps"),
        OptItem::safe("shutdown_timeout", "[SAFE] Shutdown rapide (2s timeout)"),
    ]
}

// ============================================================================
// SECTION 5: APPX BLOATWARE
// ============================================================================

/// Retourne les options de la section AppX
pub fn appx_section() -> Vec<OptItem> {
    vec![
        OptItem::safe("bing_apps", "[SAFE] Apps Bing (News, Weather, Finance, Sports, Search)"),
        OptItem::safe("ms_productivity", "[SAFE] Apps productivité (Todos, People, OfficeHub)"),
        OptItem::safe("ms_media", "[SAFE] Apps media (ZuneMusic, ZuneVideo, Clipchamp)"),
        OptItem::conditional("ms_communication", "[COND] Mail/Calendar, Skype, Teams - Attention si utilisés", false),
        OptItem::safe("ms_legacy", "[SAFE] Apps legacy (Paint3D, 3DBuilder, Print3D, MixedReality)"),
        OptItem::safe("ms_tools", "[SAFE] Outils (FeedbackHub, GetHelp, GetStarted, QuickAssist)"),
        OptItem::safe("third_party", "[SAFE] Third-party (Spotify, Disney+, CandyCrush, Facebook)"),
        OptItem::conditional("copilot", "[COND] Microsoft Copilot - Désactiver AI intégrée", false),
        OptItem::safe("cortana_app", "[SAFE] Cortana"),
        OptItem::conditional("xbox", "[COND] Xbox apps (attention Game Pass)", false),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telemetry_section_not_empty() {
        let section = telemetry_section();
        assert!(!section.is_empty());
        assert!(section.len() >= 10);
    }

    #[test]
    fn test_privacy_section_not_empty() {
        let section = privacy_section();
        assert!(!section.is_empty());
    }

    #[test]
    fn test_performance_section_desktop() {
        let section = performance_section(false);
        assert!(section.iter().any(|o| o.id == "power_ultimate" && o.risk == RiskLevel::Safe));
    }

    #[test]
    fn test_performance_section_laptop() {
        let section = performance_section(true);
        assert!(section.iter().any(|o| o.id == "power_ultimate" && o.risk == RiskLevel::Warning));
    }

    #[test]
    fn test_all_sections_have_unique_ids() {
        let mut all_ids = Vec::new();
        all_ids.extend(telemetry_section().iter().map(|o| o.id));
        all_ids.extend(privacy_section().iter().map(|o| o.id));
        all_ids.extend(performance_section(false).iter().map(|o| o.id));
        all_ids.extend(scheduler_section().iter().map(|o| o.id));
        all_ids.extend(appx_section().iter().map(|o| o.id));
        
        let unique_count = {
            let mut sorted = all_ids.clone();
            sorted.sort();
            sorted.dedup();
            sorted.len()
        };
        
        assert_eq!(all_ids.len(), unique_count, "Duplicate IDs found in sections");
    }
}
