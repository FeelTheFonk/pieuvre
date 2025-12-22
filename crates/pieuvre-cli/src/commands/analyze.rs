//! Commande analyze
//!
//! Génère des recommandations basées sur l'audit système.

use anyhow::Result;
use pieuvre_audit::full_audit;
use chrono::Local;
use std::fs;
use std::path::PathBuf;

const OUTPUT_DIR: &str = r"C:\ProgramData\Pieuvre\reports";

pub fn run(profile: &str) -> Result<()> {
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║              PIEUVRE - Analyse Systeme                           ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");
    
    println!("  [*] Profil cible: {}\n", profile.to_uppercase());
    println!("  [*] Collecte des donnees systeme...\n");
    
    // Exécuter l'audit
    let report = full_audit()?;
    
    // Afficher les résultats
    println!("═══════════════════════════════════════════════════════════════════");
    println!("                         SYSTÈME");
    println!("═══════════════════════════════════════════════════════════════════");
    println!("  OS:         {} (Build {})", report.system.os_version, report.system.build_number);
    println!("  Hostname:   {}", report.system.hostname);
    
    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("                        HARDWARE");
    println!("═══════════════════════════════════════════════════════════════════");
    println!("  CPU:        {}", report.hardware.cpu.model_name);
    println!("  Vendor:     {}", report.hardware.cpu.vendor);
    println!("  Cores:      {} logical / {} physical", 
        report.hardware.cpu.logical_cores, 
        report.hardware.cpu.physical_cores);
    if report.hardware.cpu.is_hybrid {
        println!("  Hybrid:     [OK] {} P-Cores + {} E-Cores", 
            report.hardware.cpu.p_cores.len(),
            report.hardware.cpu.e_cores.len());
    }
    println!("  RAM:        {:.1} GB total / {:.1} GB disponible",
        report.hardware.memory.total_bytes as f64 / 1024.0 / 1024.0 / 1024.0,
        report.hardware.memory.available_bytes as f64 / 1024.0 / 1024.0 / 1024.0);
    
    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("                       TÉLÉMÉTRIE");
    println!("═══════════════════════════════════════════════════════════════════");
    let diag_status = if report.telemetry.diagtrack_enabled { "[ACTIF]" } else { "[OFF]" };
    println!("  DiagTrack:  {}", diag_status);
    println!("  Niveau:     {} ({})", 
        report.telemetry.data_collection_level,
        match report.telemetry.data_collection_level {
            0 => "Security only",
            1 => "Basic",
            2 => "Enhanced",
            _ => "Full",
        });
    let adv_status = if report.telemetry.advertising_id_enabled { "[ACTIF]" } else { "[OFF]" };
    println!("  Advertising ID: {}", adv_status);
    
    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("                    RECOMMANDATIONS");
    println!("═══════════════════════════════════════════════════════════════════");
    
    // Detection laptop pour warnings
    let is_laptop = pieuvre_audit::hardware::is_laptop();
    let mut recommendations = Vec::new();
    
    // Detection etat actuel
    let timer_info = pieuvre_sync::timer::get_timer_resolution();
    let timer_optimized = match &timer_info {
        Ok(info) => info.current_ms() <= 0.55,
        Err(_) => false,
    };
    
    let power_plan = pieuvre_sync::power::get_active_power_plan().unwrap_or_default();
    let power_optimized = power_plan.contains("High") || power_plan.contains("Ultimate") || power_plan.contains("Bitsum");
    
    // Recommandations basées sur le profil
    match profile {
        "gaming" => {
            if report.telemetry.diagtrack_enabled {
                recommendations.push(("PERF", "Desactiver DiagTrack (service telemetrie)"));
            }
            // CPU Hybrid: recommander seulement si scheduler pas encore optimise
            // Note: verification via registry Win32PrioritySeparation
            if report.hardware.cpu.is_hybrid {
                let priority_sep = pieuvre_audit::registry::read_dword_value(
                    r"SYSTEM\CurrentControlSet\Control\PriorityControl",
                    "Win32PrioritySeparation"
                ).unwrap_or(0);
                if priority_sep != 0x26 {
                    recommendations.push(("PERF", "Optimiser scheduler pour CPU hybrid (P-Core priority)"));
                }
            }
            // Timer: recommander seulement si pas deja optimise
            if !timer_optimized {
                if is_laptop {
                    recommendations.push(("WARN", "Timer 0.5ms - Attention: +25% conso batterie sur laptop"));
                } else {
                    recommendations.push(("PERF", "Timer Resolution -> 0.5ms"));
                }
            }
            // Power: recommander seulement si pas deja optimise
            if !power_optimized {
                if is_laptop {
                    recommendations.push(("WARN", "Power Plan - High Performance recommande (pas Ultimate)"));
                } else {
                    recommendations.push(("PERF", "Power Plan -> Ultimate Performance"));
                }
            }
            // MSI: recommander seulement si pas deja actif
            if !pieuvre_sync::msi::is_msi_enabled_on_gpu() {
                recommendations.push(("PERF", "Activer MSI-Mode sur GPU"));
            }
        }
        "privacy" => {
            if report.telemetry.diagtrack_enabled {
                recommendations.push(("PRIVACY", "Désactiver DiagTrack"));
            }
            if report.telemetry.data_collection_level > 0 {
                recommendations.push(("PRIVACY", "Réduire niveau télémétrie → Security only"));
            }
            if report.telemetry.advertising_id_enabled {
                recommendations.push(("PRIVACY", "Désactiver Advertising ID"));
            }
            recommendations.push(("PRIVACY", "Bloquer domaines télémétrie (firewall)"));
            recommendations.push(("PRIVACY", "Désactiver services: WerSvc, lfsvc, MapsBroker"));
        }
        "workstation" => {
            if report.telemetry.diagtrack_enabled {
                recommendations.push(("BALANCED", "Désactiver DiagTrack"));
            }
            recommendations.push(("BALANCED", "Power Plan → High Performance"));
            recommendations.push(("BALANCED", "Conserver WSearch pour productivité"));
        }
        _ => {
            recommendations.push(("INFO", "Profil non reconnu, utilisez: gaming, privacy, workstation"));
        }
    }
    
    if recommendations.is_empty() {
        println!("  [OK] Systeme deja optimise pour le profil {}", profile);
    } else {
        for (cat, rec) in &recommendations {
            println!("  [{:^8}] {}", cat, rec);
        }
    }
    
    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("                      SERVICES ({} total)", report.services.len());
    println!("═══════════════════════════════════════════════════════════════════");
    
    let telemetry_services: Vec<_> = report.services.iter()
        .filter(|s| matches!(s.category, pieuvre_common::ServiceCategory::Telemetry))
        .collect();
    let running_telemetry = telemetry_services.iter().filter(|s| matches!(s.status, pieuvre_common::ServiceStatus::Running)).count();
    
    println!("  Télémétrie: {} services ({} running)", telemetry_services.len(), running_telemetry);
    
    let perf_services: Vec<_> = report.services.iter()
        .filter(|s| matches!(s.category, pieuvre_common::ServiceCategory::Performance))
        .collect();
    println!("  Performance: {} services", perf_services.len());
    
    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("                      PACKAGES APPX");
    println!("═══════════════════════════════════════════════════════════════════");
    
    let safe_to_remove: Vec<_> = report.appx.iter()
        .filter(|p| matches!(p.removal_risk, pieuvre_common::RemovalRisk::Safe))
        .take(5)
        .collect();
    
    if !safe_to_remove.is_empty() {
        println!("  Bloatware détecté (safe à supprimer):");
        for pkg in safe_to_remove {
            println!("    • {}", pkg.name);
        }
    }
    
    // Sauvegarder le rapport
    fs::create_dir_all(OUTPUT_DIR)?;
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let report_path = PathBuf::from(OUTPUT_DIR).join(format!("analyze_{}_{}.json", profile, timestamp));
    let json = serde_json::to_string_pretty(&report)?;
    fs::write(&report_path, &json)?;
    
    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("  [*] Rapport sauvegarde: {}", report_path.display());
    println!("═══════════════════════════════════════════════════════════════════");
    
    println!("\n  Prochaines etapes:");
    println!("    pieuvre sync --profile {} --dry-run", profile);
    println!("    pieuvre sync --profile {}", profile);
    
    Ok(())
}
