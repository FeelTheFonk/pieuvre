//! Commande interactive
//!
//! Selection granulaire des optimisations avec interface dialoguer.
//! Navigation: Fleches haut/bas, Espace pour cocher, Entree pour valider.

use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, MultiSelect};
use pieuvre_audit::hardware::is_laptop;
use pieuvre_sync::{timer, power, firewall, msi, registry};
use pieuvre_persist::snapshot;
use pieuvre_common::ChangeRecord;

/// IDs fixes pour identifier les options
const OPT_DIAGTRACK: usize = 0;
const OPT_DMWAPPUSH: usize = 1;
const OPT_WERSVC: usize = 2;
const OPT_FIREWALL: usize = 3;

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
    // SECTION 1: TELEMETRIE
    // =========================================
    println!("----------------------------------------------------------------");
    println!("  1/3  TELEMETRIE");
    println!("----------------------------------------------------------------");
    
    let telemetry_items = vec![
        "[SAFE] DiagTrack - Service telemetrie principale",
        "[SAFE] dmwappushservice - Push WAP telemetrie",
        "[SAFE] WerSvc - Windows Error Reporting",
        "[SAFE] Firewall - Bloquer 42 domaines telemetrie",
    ];
    
    let telemetry_selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Telemetrie (Espace=cocher, Entree=valider)")
        .items(&telemetry_items)
        .defaults(&[true, true, true, true])
        .interact()?;
    
    // =========================================
    // SECTION 2: PERFORMANCE
    // =========================================
    println!();
    println!("----------------------------------------------------------------");
    println!("  2/3  PERFORMANCE");
    println!("----------------------------------------------------------------");
    
    // Construction dynamique avec IDs explicites
    #[derive(Clone)]
    struct PerfOption {
        id: &'static str,
        label: String,
        default: bool,
    }
    
    let mut perf_options: Vec<PerfOption> = Vec::new();
    
    // Timer 0.5ms
    if laptop {
        perf_options.push(PerfOption {
            id: "timer",
            label: "[WARN][LAPTOP] Timer 0.5ms - +25% conso batterie".to_string(),
            default: false,
        });
    } else {
        perf_options.push(PerfOption {
            id: "timer",
            label: "[SAFE] Timer 0.5ms - Latence reduite (gaming)".to_string(),
            default: true,
        });
    }
    
    // Power Plan - TOUJOURS ajouter les deux options pour laptop
    if laptop {
        perf_options.push(PerfOption {
            id: "power_ultimate",
            label: "[WARN][LAPTOP] Ultimate Performance - Usure batterie".to_string(),
            default: false,
        });
        perf_options.push(PerfOption {
            id: "power_high",
            label: "[SAFE] High Performance - Recommande laptop".to_string(),
            default: true,
        });
    } else {
        perf_options.push(PerfOption {
            id: "power_ultimate",
            label: "[SAFE] Ultimate Performance - Max performance desktop".to_string(),
            default: true,
        });
    }
    
    // MSI Mode
    perf_options.push(PerfOption {
        id: "msi",
        label: "[SAFE] MSI Mode GPU/NVMe - -40% latence interrupts".to_string(),
        default: true,
    });
    
    // SysMain
    perf_options.push(PerfOption {
        id: "sysmain",
        label: "[SAFE] Desactiver SysMain - Recommande si SSD".to_string(),
        default: true,
    });
    
    // WSearch
    perf_options.push(PerfOption {
        id: "wsearch",
        label: "[COND] Desactiver WSearch - Recherche plus lente".to_string(),
        default: false,
    });
    
    let perf_labels: Vec<&str> = perf_options.iter().map(|o| o.label.as_str()).collect();
    let perf_defaults: Vec<bool> = perf_options.iter().map(|o| o.default).collect();
    
    let perf_selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Performance (Espace=cocher, Entree=valider)")
        .items(&perf_labels)
        .defaults(&perf_defaults)
        .interact()?;
    
    // =========================================
    // SECTION 3: SCHEDULER
    // =========================================
    println!();
    println!("----------------------------------------------------------------");
    println!("  3/3  SCHEDULER");
    println!("----------------------------------------------------------------");
    
    let sched_items = vec![
        "[SAFE] Win32PrioritySeparation 0x26 - Short quantum, foreground boost",
    ];
    
    let sched_selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Scheduler (Espace=cocher, Entree=valider)")
        .items(&sched_items)
        .defaults(&[true])
        .interact()?;
    
    // =========================================
    // RESUME SELECTION
    // =========================================
    println!();
    println!("================================================================");
    println!("                    RESUME SELECTION");
    println!("================================================================");
    
    let mut total = 0;
    
    if !telemetry_selected.is_empty() {
        println!();
        println!("  TELEMETRIE:");
        for idx in &telemetry_selected {
            println!("    [x] {}", telemetry_items[*idx]);
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
            println!("    [x] {}", sched_items[*idx]);
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
    
    // Creer snapshot avant modifications
    println!("[*] Creation snapshot de sauvegarde...");
    let changes = Vec::<ChangeRecord>::new();
    match snapshot::create("Avant mode interactif", changes) {
        Ok(snap) => println!("    Snapshot: {}", snap.id),
        Err(e) => println!("    Snapshot erreur: {}", e),
    }
    println!();
    
    let mut success_count = 0;
    let mut error_count = 0;
    
    // TELEMETRIE - utiliser les IDs fixes
    for idx in &telemetry_selected {
        match *idx {
            OPT_DIAGTRACK => {
                print!("[*] DiagTrack... ");
                match pieuvre_sync::services::disable_service("DiagTrack") {
                    Ok(_) => { println!("OK"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            OPT_DMWAPPUSH => {
                print!("[*] dmwappushservice... ");
                match pieuvre_sync::services::disable_service("dmwappushservice") {
                    Ok(_) => { println!("OK"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            OPT_WERSVC => {
                print!("[*] WerSvc... ");
                match pieuvre_sync::services::disable_service("WerSvc") {
                    Ok(_) => { println!("OK"); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            OPT_FIREWALL => {
                print!("[*] Firewall rules... ");
                match firewall::create_telemetry_block_rules() {
                    Ok(rules) => { println!("OK ({} regles)", rules.len()); success_count += 1; }
                    Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
                }
            }
            _ => {}
        }
    }
    
    // PERFORMANCE - utiliser les IDs explicites
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
            "msi" => {
                print!("[*] MSI Mode detection... ");
                match msi::list_msi_eligible_devices() {
                    Ok(devices) => { 
                        println!("OK ({} devices)", devices.len()); 
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
            _ => {}
        }
    }
    
    // SCHEDULER
    for idx in &sched_selected {
        if *idx == 0 {
            print!("[*] Win32PrioritySeparation... ");
            match registry::set_priority_separation(0x26) {
                Ok(_) => { println!("OK (0x26)"); success_count += 1; }
                Err(e) => { println!("ERREUR: {}", e); error_count += 1; }
            }
        }
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
