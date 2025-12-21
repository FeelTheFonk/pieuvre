//! Commande interactive
//!
//! Selection granulaire des optimisations avec interface dialoguer.
//! Navigation: Fleches haut/bas, Espace pour cocher, Entree pour valider.

use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, MultiSelect};
use pieuvre_audit::hardware::is_laptop;
use pieuvre_sync::{appx, timer, power, firewall, msi, registry};
use pieuvre_persist::snapshot;
use pieuvre_common::ChangeRecord;

pub fn run(profile: &str) -> Result<()> {
    println!();
    println!("================================================================");
    println!("           PIEUVRE - Mode Interactif");
    println!("================================================================");
    println!();
    println!("  NAVIGATION:");
    println!("    Fleches    Haut/Bas pour naviguer");
    println!("    Espace     Cocher/Decocher une option");
    println!("    Entree     Valider la selection");
    println!();
    
    let laptop = is_laptop();
    println!("  Systeme: {}", if laptop { "LAPTOP (batterie detectee)" } else { "DESKTOP" });
    println!("  Profil:  {}", profile.to_uppercase());
    println!();
    
    if laptop {
        println!("  [!] Options avec [LAPTOP] deconseillees sur batterie");
        println!();
    }
    
    // =========================================
    // SECTION 1: TELEMETRIE (9 services total)
    // =========================================
    println!("----------------------------------------------------------------");
    println!("  1/5  TELEMETRIE - Services");
    println!("----------------------------------------------------------------");
    
    #[derive(Clone)]
    struct OptItem {
        id: &'static str,
        label: &'static str,
        default: bool,
    }
    
    let telemetry_services = vec![
        OptItem { id: "diagtrack", label: "[SAFE] DiagTrack - Telemetrie principale", default: true },
        OptItem { id: "dmwappush", label: "[SAFE] dmwappushservice - Push WAP", default: true },
        OptItem { id: "wersvc", label: "[SAFE] WerSvc - Windows Error Reporting", default: true },
        OptItem { id: "wercplsupport", label: "[SAFE] wercplsupport - Error Reports support", default: true },
        OptItem { id: "pcasvc", label: "[SAFE] PcaSvc - Program Compatibility", default: false },
        OptItem { id: "wdisystem", label: "[SAFE] WdiSystemHost - Diagnostic Host", default: false },
        OptItem { id: "wdiservice", label: "[SAFE] WdiServiceHost - Diagnostic Service", default: false },
        OptItem { id: "lfsvc", label: "[COND] lfsvc - Geolocation", default: true },
        OptItem { id: "mapsbroker", label: "[SAFE] MapsBroker - Maps download", default: true },
        OptItem { id: "firewall", label: "[SAFE] Firewall - Bloquer 47 domaines telemetrie", default: true },
        // SOTA P1
        OptItem { id: "sched_tasks", label: "[SAFE] Scheduled Tasks - Desactiver 25 taches telemetrie", default: true },
        OptItem { id: "hosts", label: "[SAFE] Hosts file - Bloquer 50+ domaines DNS natif", default: false },
        OptItem { id: "onedrive", label: "[COND] OneDrive - Desinstaller completement", default: false },
    ];
    
    let telem_labels: Vec<&str> = telemetry_services.iter().map(|o| o.label).collect();
    let telem_defaults: Vec<bool> = telemetry_services.iter().map(|o| o.default).collect();
    
    let telem_selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Services telemetrie (Espace=cocher, Entree=valider)")
        .items(&telem_labels)
        .defaults(&telem_defaults)
        .interact()?;
    
    // =========================================
    // SECTION 2: PRIVACY - Registry tweaks
    // =========================================
    println!();
    println!("----------------------------------------------------------------");
    println!("  2/5  PRIVACY - Registre");
    println!("----------------------------------------------------------------");
    
    let privacy_options = vec![
        OptItem { id: "telemetry_level", label: "[SAFE] Telemetry Level 0 (Security only)", default: true },
        OptItem { id: "advertising_id", label: "[SAFE] Desactiver Advertising ID", default: true },
        OptItem { id: "location", label: "[SAFE] Desactiver Localisation", default: true },
        OptItem { id: "activity_history", label: "[SAFE] Desactiver Historique activite", default: true },
        OptItem { id: "cortana", label: "[SAFE] Desactiver Cortana", default: true },
        // P2 SOTA
        OptItem { id: "context_menu", label: "[SAFE] Classic context menu + nettoyer clutter", default: true },
        OptItem { id: "widgets", label: "[SAFE] Desactiver Widgets Win11", default: true },
        OptItem { id: "pause_updates", label: "[COND] Pause Windows Updates 35 jours", default: false },
        OptItem { id: "driver_updates", label: "[COND] Desactiver maj drivers auto", default: false },
        // P4 SOTA
        OptItem { id: "recall", label: "[SAFE] Bloquer Windows Recall (24H2 AI)", default: true },
        OptItem { id: "group_policy_telem", label: "[SAFE] Group Policy Telemetry (enterprise)", default: true },
    ];
    
    let privacy_labels: Vec<&str> = privacy_options.iter().map(|o| o.label).collect();
    let privacy_defaults: Vec<bool> = privacy_options.iter().map(|o| o.default).collect();
    
    let privacy_selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Privacy registre (Espace=cocher, Entree=valider)")
        .items(&privacy_labels)
        .defaults(&privacy_defaults)
        .interact()?;
    
    // =========================================
    // SECTION 3: PERFORMANCE
    // =========================================
    println!();
    println!("----------------------------------------------------------------");
    println!("  3/5  PERFORMANCE");
    println!("----------------------------------------------------------------");
    
    let mut perf_options: Vec<OptItem> = Vec::new();
    
    // Timer 0.5ms
    if laptop {
        perf_options.push(OptItem {
            id: "timer",
            label: "[WARN][LAPTOP] Timer 0.5ms - +25% conso batterie",
            default: false,
        });
    } else {
        perf_options.push(OptItem {
            id: "timer",
            label: "[SAFE] Timer 0.5ms - Latence reduite (gaming)",
            default: true,
        });
    }
    
    // Power Plan
    if laptop {
        perf_options.push(OptItem {
            id: "power_ultimate",
            label: "[WARN][LAPTOP] Ultimate Performance - Usure batterie",
            default: false,
        });
        perf_options.push(OptItem {
            id: "power_high",
            label: "[SAFE] High Performance - Recommande laptop",
            default: true,
        });
    } else {
        perf_options.push(OptItem {
            id: "power_ultimate",
            label: "[SAFE] Ultimate Performance - Max performance desktop",
            default: true,
        });
    }
    
    // CPU Throttling
    perf_options.push(OptItem {
        id: "cpu_throttle",
        label: "[PERF] Desactiver CPU Throttling",
        default: true,
    });
    
    // USB Selective Suspend
    perf_options.push(OptItem {
        id: "usb_suspend",
        label: "[PERF] Desactiver USB Selective Suspend",
        default: true,
    });
    
    // MSI Mode
    perf_options.push(OptItem {
        id: "msi",
        label: "[SAFE] Activer MSI Mode GPU/NVMe",
        default: true,
    });
    
    // Services performance
    perf_options.push(OptItem {
        id: "sysmain",
        label: "[SAFE] Desactiver SysMain (SSD recommande)",
        default: true,
    });
    
    perf_options.push(OptItem {
        id: "wsearch",
        label: "[COND] Desactiver WSearch - Recherche plus lente",
        default: false,
    });
    
    // P3 SOTA
    perf_options.push(OptItem {
        id: "edge_disable",
        label: "[SAFE] Desactiver features Edge (sidebar, DVR, shopping)",
        default: true,
    });
    
    perf_options.push(OptItem {
        id: "explorer_tweaks",
        label: "[SAFE] Explorer tweaks (extensions, This PC, no recent)",
        default: true,
    });
    
    perf_options.push(OptItem {
        id: "game_bar",
        label: "[SAFE] Desactiver Game Bar/DVR (alt+tab plus rapide)",
        default: true,
    });
    
    perf_options.push(OptItem {
        id: "fullscreen_opt",
        label: "[PERF] Desactiver Fullscreen Optimizations",
        default: true,
    });
    
    perf_options.push(OptItem {
        id: "hags",
        label: "[COND] Desactiver HAGS (meilleur pour anciens jeux)",
        default: false,
    });
    
    // P4 SOTA
    perf_options.push(OptItem {
        id: "nagle",
        label: "[PERF] Desactiver Nagle Algorithm (latence reseau)",
        default: true,
    });
    
    perf_options.push(OptItem {
        id: "power_throttle",
        label: "[PERF] Desactiver CPU Power Throttling",
        default: true,
    });
    
    let perf_labels: Vec<&str> = perf_options.iter().map(|o| o.label).collect();
    let perf_defaults: Vec<bool> = perf_options.iter().map(|o| o.default).collect();
    
    let perf_selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Performance (Espace=cocher, Entree=valider)")
        .items(&perf_labels)
        .defaults(&perf_defaults)
        .interact()?;
    
    // =========================================
    // SECTION 4: SCHEDULER
    // =========================================
    println!();
    println!("----------------------------------------------------------------");
    println!("  4/5  SCHEDULER");
    println!("----------------------------------------------------------------");
    
    let sched_options = vec![
        OptItem { id: "priority_sep", label: "[SAFE] Win32PrioritySeparation 0x26 - Short quantum, foreground boost", default: true },
        OptItem { id: "mmcss", label: "[SAFE] MMCSS Gaming - SystemResponsiveness 10%, Network throttling OFF", default: true },
        OptItem { id: "games_priority", label: "[SAFE] GPU Priority 8, Task Priority 6 - Gaming boost", default: true },
        OptItem { id: "global_timer", label: "[PERF] Timer resolution permanente - Reboot recommande", default: false },
        OptItem { id: "startup_delay", label: "[SAFE] Desactiver delai startup apps", default: true },
        OptItem { id: "shutdown_timeout", label: "[SAFE] Shutdown rapide (2s timeout)", default: true },
    ];
    
    let sched_labels: Vec<&str> = sched_options.iter().map(|o| o.label).collect();
    let sched_defaults: Vec<bool> = sched_options.iter().map(|o| o.default).collect();
    
    let sched_selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Scheduler (Espace=cocher, Entree=valider)")
        .items(&sched_labels)
        .defaults(&sched_defaults)
        .interact()?;
    
    // =========================================
    // SECTION 5: APPX BLOATWARE
    // =========================================
    println!();
    println!("----------------------------------------------------------------");
    println!("  5/5  APPX - Bloatware");
    println!("----------------------------------------------------------------");
    
    let appx_options = vec![
        // Categories granulaires
        OptItem { id: "bing_apps", label: "[SAFE] Apps Bing (News, Weather, Finance, Sports, Search)", default: true },
        OptItem { id: "ms_productivity", label: "[SAFE] Apps productivite (Todos, People, OfficeHub)", default: true },
        OptItem { id: "ms_media", label: "[SAFE] Apps media (ZuneMusic, ZuneVideo, Clipchamp)", default: true },
        OptItem { id: "ms_communication", label: "[COND] Mail/Calendar, Skype, Teams - Attention si utilises", default: false },
        OptItem { id: "ms_legacy", label: "[SAFE] Apps legacy (Paint3D, 3DBuilder, Print3D, MixedReality)", default: true },
        OptItem { id: "ms_tools", label: "[SAFE] Outils (FeedbackHub, GetHelp, GetStarted, QuickAssist)", default: true },
        OptItem { id: "third_party", label: "[SAFE] Third-party (Spotify, Disney+, CandyCrush, Facebook)", default: true },
        OptItem { id: "copilot", label: "[COND] Microsoft Copilot - Desactiver AI integree", default: false },
        OptItem { id: "cortana", label: "[SAFE] Cortana", default: true },
        OptItem { id: "xbox", label: "[COND] Xbox apps (attention Game Pass)", default: false },
    ];
    
    let appx_labels: Vec<&str> = appx_options.iter().map(|o| o.label).collect();
    let appx_defaults: Vec<bool> = appx_options.iter().map(|o| o.default).collect();
    
    let appx_selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("AppX Bloatware (Espace=cocher, Entree=valider)")
        .items(&appx_labels)
        .defaults(&appx_defaults)
        .interact()?;
    
    // =========================================
    // RESUME SELECTION
    // =========================================
    println!();
    println!("================================================================");
    println!("                    RESUME SELECTION");
    println!("================================================================");
    
    let mut total = 0;
    
    if !telem_selected.is_empty() {
        println!();
        println!("  TELEMETRIE:");
        for idx in &telem_selected {
            println!("    [x] {}", telemetry_services[*idx].label);
            total += 1;
        }
    }
    
    if !privacy_selected.is_empty() {
        println!();
        println!("  PRIVACY:");
        for idx in &privacy_selected {
            println!("    [x] {}", privacy_options[*idx].label);
            total += 1;
        }
    }
    
    if !perf_selected.is_empty() {
        println!();
        println!("  PERFORMANCE:");
        for idx in &perf_selected {
            println!("    [x] {}", perf_options[*idx].label);
            total += 1;
        }
    }
    
    if !sched_selected.is_empty() {
        println!();
        println!("  SCHEDULER:");
        for idx in &sched_selected {
            println!("    [x] {}", sched_options[*idx].label);
            total += 1;
        }
    }
    
    if !appx_selected.is_empty() {
        println!();
        println!("  APPX:");
        for idx in &appx_selected {
            println!("    [x] {}", appx_options[*idx].label);
            total += 1;
        }
    }
    
    println!();
    println!("  Total: {} optimisations selectionnees", total);
    
    if total == 0 {
        println!();
        println!("[*] Aucune optimisation selectionnee. Fin.");
        return Ok(());
    }
    
    // =========================================
    // CONFIRMATION
    // =========================================
    println!();
    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Appliquer ces modifications? (y/n)")
        .default(false)
        .interact()?;
    
    if !confirm {
        println!();
        println!("[*] Annule. Aucune modification effectuee.");
        return Ok(());
    }
    
    // =========================================
    // APPLICATION DES MODIFICATIONS
    // =========================================
    println!();
    println!("================================================================");
    println!("                APPLICATION EN COURS");
    println!("================================================================");
    println!();
    
    // Vecteur pour capturer les changements (pour rollback)
    let mut changes = Vec::<ChangeRecord>::new();
    
    let mut success_count = 0;
    let mut error_count = 0;
    
    // TELEMETRIE SERVICES
    for idx in &telem_selected {
        let opt = &telemetry_services[*idx];
        
        match opt.id {
            "diagtrack" => {
                print!("[*] DiagTrack... ");
                // Capturer etat original
                if let Ok(original) = pieuvre_sync::services::get_service_start_type("DiagTrack") {
                    changes.push(ChangeRecord::Service {
                        name: "DiagTrack".to_string(),
                        original_start_type: original,
                    });
                }
                match pieuvre_sync::services::disable_service("DiagTrack") {
                    Ok(_) => { println!("OK"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "dmwappush" => {
                print!("[*] dmwappushservice... ");
                if let Ok(original) = pieuvre_sync::services::get_service_start_type("dmwappushservice") {
                    changes.push(ChangeRecord::Service {
                        name: "dmwappushservice".to_string(),
                        original_start_type: original,
                    });
                }
                match pieuvre_sync::services::disable_service("dmwappushservice") {
                    Ok(_) => { println!("OK"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "wersvc" => {
                print!("[*] WerSvc... ");
                if let Ok(original) = pieuvre_sync::services::get_service_start_type("WerSvc") {
                    changes.push(ChangeRecord::Service {
                        name: "WerSvc".to_string(),
                        original_start_type: original,
                    });
                }
                match pieuvre_sync::services::disable_service("WerSvc") {
                    Ok(_) => { println!("OK"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "wercplsupport" => {
                print!("[*] wercplsupport... ");
                if let Ok(original) = pieuvre_sync::services::get_service_start_type("wercplsupport") {
                    changes.push(ChangeRecord::Service {
                        name: "wercplsupport".to_string(),
                        original_start_type: original,
                    });
                }
                match pieuvre_sync::services::disable_service("wercplsupport") {
                    Ok(_) => { println!("OK"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "pcasvc" => {
                print!("[*] PcaSvc... ");
                if let Ok(original) = pieuvre_sync::services::get_service_start_type("PcaSvc") {
                    changes.push(ChangeRecord::Service {
                        name: "PcaSvc".to_string(),
                        original_start_type: original,
                    });
                }
                match pieuvre_sync::services::disable_service("PcaSvc") {
                    Ok(_) => { println!("OK"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "wdisystem" => {
                print!("[*] WdiSystemHost... ");
                if let Ok(original) = pieuvre_sync::services::get_service_start_type("WdiSystemHost") {
                    changes.push(ChangeRecord::Service {
                        name: "WdiSystemHost".to_string(),
                        original_start_type: original,
                    });
                }
                match pieuvre_sync::services::disable_service("WdiSystemHost") {
                    Ok(_) => { println!("OK"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "wdiservice" => {
                print!("[*] WdiServiceHost... ");
                if let Ok(original) = pieuvre_sync::services::get_service_start_type("WdiServiceHost") {
                    changes.push(ChangeRecord::Service {
                        name: "WdiServiceHost".to_string(),
                        original_start_type: original,
                    });
                }
                match pieuvre_sync::services::disable_service("WdiServiceHost") {
                    Ok(_) => { println!("OK"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "lfsvc" => {
                print!("[*] lfsvc... ");
                if let Ok(original) = pieuvre_sync::services::get_service_start_type("lfsvc") {
                    changes.push(ChangeRecord::Service {
                        name: "lfsvc".to_string(),
                        original_start_type: original,
                    });
                }
                match pieuvre_sync::services::disable_service("lfsvc") {
                    Ok(_) => { println!("OK"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "mapsbroker" => {
                print!("[*] MapsBroker... ");
                if let Ok(original) = pieuvre_sync::services::get_service_start_type("MapsBroker") {
                    changes.push(ChangeRecord::Service {
                        name: "MapsBroker".to_string(),
                        original_start_type: original,
                    });
                }
                match pieuvre_sync::services::disable_service("MapsBroker") {
                    Ok(_) => { println!("OK"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "firewall" => {
                print!("[*] Firewall rules... ");
                match firewall::create_telemetry_block_rules() {
                    Ok(rules) => { println!("OK ({} regles)", rules.len()); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "sched_tasks" => {
                print!("[*] Scheduled Tasks... ");
                match pieuvre_sync::scheduled_tasks::disable_telemetry_tasks() {
                    Ok(tasks) => { println!("OK ({} tasks disabled)", tasks.len()); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "hosts" => {
                print!("[*] Hosts file... ");
                match pieuvre_sync::hosts::add_telemetry_blocks() {
                    Ok(count) => { println!("OK ({} domains blocked)", count); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "onedrive" => {
                print!("[*] OneDrive... ");
                match pieuvre_sync::onedrive::uninstall_onedrive() {
                    Ok(_) => { println!("OK (removed)"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            _ => {}
        }
    }
    
    // PRIVACY REGISTRY
    for idx in &privacy_selected {
        let opt = &privacy_options[*idx];
        
        match opt.id {
            "telemetry_level" => {
                print!("[*] Telemetry Level 0... ");
                match registry::set_telemetry_level(0) {
                    Ok(_) => { println!("OK"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "advertising_id" => {
                print!("[*] Advertising ID... ");
                match registry::disable_advertising_id() {
                    Ok(_) => { println!("OK"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "location" => {
                print!("[*] Location... ");
                match registry::disable_location() {
                    Ok(_) => { println!("OK"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "activity_history" => {
                print!("[*] Activity History... ");
                match registry::disable_activity_history() {
                    Ok(_) => { println!("OK"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "cortana" => {
                print!("[*] Cortana... ");
                match registry::disable_cortana() {
                    Ok(_) => { println!("OK"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "context_menu" => {
                print!("[*] Context Menu... ");
                match pieuvre_sync::context_menu::remove_context_menu_clutter() {
                    Ok(n) => { println!("OK ({} items removed, classic)", n); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "widgets" => {
                print!("[*] Widgets... ");
                match pieuvre_sync::widgets::disable_widgets() {
                    Ok(_) => { println!("OK (disabled)"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "pause_updates" => {
                print!("[*] Windows Updates... ");
                match pieuvre_sync::windows_update::pause_updates() {
                    Ok(_) => { println!("OK (paused 35 days)"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "driver_updates" => {
                print!("[*] Driver Updates... ");
                match pieuvre_sync::windows_update::disable_driver_updates() {
                    Ok(_) => { println!("OK (disabled)"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "recall" => {
                print!("[*] Windows Recall... ");
                match pieuvre_sync::registry::disable_recall() {
                    Ok(_) => { println!("OK (blocked)"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "group_policy_telem" => {
                print!("[*] Group Policy Telemetry... ");
                match pieuvre_sync::registry::set_group_policy_telemetry(0) {
                    Ok(_) => { println!("OK (Security level)"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            _ => {}
        }
    }
    
    // PERFORMANCE
    for idx in &perf_selected {
        let opt = &perf_options[*idx];
        
        match opt.id {
            "timer" => {
                print!("[*] Timer Resolution... ");
                match timer::set_timer_resolution(5000) {
                    Ok(_) => { println!("OK (0.5ms)"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "power_ultimate" => {
                print!("[*] Power Plan Ultimate... ");
                match power::set_power_plan(power::PowerPlan::UltimatePerformance) {
                    Ok(_) => { println!("OK"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "power_high" => {
                print!("[*] Power Plan High... ");
                match power::set_power_plan(power::PowerPlan::HighPerformance) {
                    Ok(_) => { println!("OK"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "cpu_throttle" => {
                print!("[*] CPU Throttling disable... ");
                match power::disable_cpu_throttling() {
                    Ok(_) => { println!("OK"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "usb_suspend" => {
                print!("[*] USB Selective Suspend disable... ");
                match power::configure_power_settings(false, false, 100, 100) {
                    Ok(_) => { println!("OK"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "msi" => {
                print!("[*] MSI Mode... ");
                match msi::list_msi_eligible_devices() {
                    Ok(devices) => {
                        let mut enabled = 0;
                        for dev in &devices {
                            if !dev.msi_enabled {
                                if msi::enable_msi(&dev.full_path).is_ok() {
                                    enabled += 1;
                                }
                            } else {
                                enabled += 1; // Déjà activé
                            }
                        }
                        println!("OK ({}/{} devices)", enabled, devices.len());
                        success_count += 1;
                    }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "sysmain" => {
                print!("[*] SysMain... ");
                match pieuvre_sync::services::disable_service("SysMain") {
                    Ok(_) => { println!("OK"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "wsearch" => {
                print!("[*] WSearch... ");
                match pieuvre_sync::services::disable_service("WSearch") {
                    Ok(_) => { println!("OK"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "edge_disable" => {
                print!("[*] Edge features... ");
                match pieuvre_sync::edge::disable_edge() {
                    Ok(_) => { 
                        let _ = pieuvre_sync::edge::remove_edge_tasks();
                        println!("OK (features + tasks)"); 
                        success_count += 1; 
                    }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "explorer_tweaks" => {
                print!("[*] Explorer tweaks... ");
                match pieuvre_sync::explorer::apply_explorer_tweaks() {
                    Ok(_) => { println!("OK (8 tweaks)"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "game_bar" => {
                print!("[*] Game Bar... ");
                match pieuvre_sync::game_mode::disable_game_bar() {
                    Ok(_) => { println!("OK (disabled)"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "fullscreen_opt" => {
                print!("[*] Fullscreen Optimizations... ");
                match pieuvre_sync::game_mode::disable_fullscreen_optimizations() {
                    Ok(_) => { println!("OK (disabled)"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "hags" => {
                print!("[*] HAGS... ");
                match pieuvre_sync::game_mode::disable_hags() {
                    Ok(_) => { println!("OK (disabled)"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "nagle" => {
                print!("[*] Nagle Algorithm... ");
                match pieuvre_sync::network::disable_nagle_algorithm() {
                    Ok(n) => { println!("OK ({} interfaces)", n); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "power_throttle" => {
                print!("[*] Power Throttling... ");
                match pieuvre_sync::registry::disable_power_throttling() {
                    Ok(_) => { println!("OK (disabled)"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            _ => {}
        }
    }
    
    // SCHEDULER
    for idx in &sched_selected {
        let opt = &sched_options[*idx];
        
        match opt.id {
            "priority_sep" => {
                print!("[*] Win32PrioritySeparation... ");
                match registry::set_priority_separation(0x26) {
                    Ok(_) => { println!("OK (0x26)"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "mmcss" => {
                print!("[*] MMCSS Gaming... ");
                match registry::configure_mmcss_gaming() {
                    Ok(_) => { println!("OK"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "games_priority" => {
                print!("[*] Games Priority... ");
                match registry::configure_games_priority() {
                    Ok(_) => { println!("OK (GPU=8, Priority=6)"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "global_timer" => {
                print!("[*] Global Timer Resolution... ");
                match registry::enable_global_timer_resolution() {
                    Ok(_) => { println!("OK (reboot required)"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "startup_delay" => {
                print!("[*] Startup Delay... ");
                match registry::disable_startup_delay() {
                    Ok(_) => { println!("OK (0ms)"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "shutdown_timeout" => {
                print!("[*] Shutdown Timeout... ");
                match registry::reduce_shutdown_timeout() {
                    Ok(_) => { println!("OK (2000ms)"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            _ => {}
        }
    }
    
    // APPX
    for idx in &appx_selected {
        let opt = &appx_options[*idx];
        
        match opt.id {
            "bing_apps" => {
                print!("[*] Bing apps... ");
                match appx::remove_bing_apps() {
                    Ok(r) => { println!("OK ({} removed)", r.len()); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "ms_productivity" => {
                print!("[*] Productivity apps... ");
                match appx::remove_ms_productivity() {
                    Ok(r) => { println!("OK ({} removed)", r.len()); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "ms_media" => {
                print!("[*] Media apps... ");
                match appx::remove_ms_media() {
                    Ok(r) => { println!("OK ({} removed)", r.len()); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "ms_communication" => {
                print!("[*] Communication apps... ");
                match appx::remove_ms_communication() {
                    Ok(r) => { println!("OK ({} removed)", r.len()); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "ms_legacy" => {
                print!("[*] Legacy apps... ");
                match appx::remove_ms_legacy() {
                    Ok(r) => { println!("OK ({} removed)", r.len()); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "ms_tools" => {
                print!("[*] MS Tools... ");
                match appx::remove_ms_tools() {
                    Ok(r) => { println!("OK ({} removed)", r.len()); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "third_party" => {
                print!("[*] Third-party apps... ");
                match appx::remove_third_party() {
                    Ok(r) => { println!("OK ({} removed)", r.len()); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "copilot" => {
                print!("[*] Copilot... ");
                match appx::remove_copilot() {
                    Ok(r) => { println!("OK ({} removed)", r.len()); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "cortana" => {
                print!("[*] Cortana... ");
                match appx::remove_cortana() {
                    Ok(r) => { println!("OK ({} removed)", r.len()); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            "xbox" => {
                print!("[*] Xbox apps... ");
                match appx::remove_xbox_packages() {
                    Ok(r) => { println!("OK ({} removed)", r.len()); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            _ => {}
        }
    }
    
    // =========================================
    // CREATION SNAPSHOT AVEC CHANGEMENTS
    // =========================================
    println!();
    println!("[*] Creation snapshot de sauvegarde...");
    match snapshot::create("Avant mode interactif", changes) {
        Ok(snap) => println!("    Snapshot: {} ({} changements)", snap.id, snap.changes.len()),
        Err(e) => println!("    Snapshot erreur: {}", e),
    }
    
    // =========================================
    // RESULTAT FINAL
    // =========================================
    println!();
    println!("================================================================");
    println!("                      RESULTAT");
    println!("================================================================");
    println!();
    println!("  Succes: {}", success_count);
    println!("  Erreurs: {}", error_count);
    println!();
    
    if error_count == 0 {
        println!("[OK] Toutes les modifications appliquees avec succes.");
    } else {
        println!("[!] Certaines modifications ont echoue.");
        println!("    Executez en tant qu'administrateur si necessaire.");
    }
    
    println!();
    println!("  Pour annuler: pieuvre rollback --last");
    println!("  Pour verifier: pieuvre status");
    println!();
    
    Ok(())
}
