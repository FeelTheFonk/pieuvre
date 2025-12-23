//! Commande audit
//!
//! Audit complet du système avec sauvegarde automatique.

use anyhow::Result;
use chrono::Local;
use pieuvre_common::AuditReport;
use std::fs;
use std::path::PathBuf;

const OUTPUT_DIR: &str = r"C:\ProgramData\pieuvre\reports";

pub fn run(
    full: bool,
    output: Option<String>,
    mut log_cb: Option<&mut dyn FnMut(&str, &str)>,
) -> Result<AuditReport> {
    if log_cb.is_none() {
        println!("\n╔══════════════════════════════════════════════════════════════════╗");
        println!("║              PIEUVRE - Audit Systeme                             ║");
        println!("╚══════════════════════════════════════════════════════════════════╝\n");
    }

    if let Some(ref mut cb) = log_cb {
        cb(
            "INFO",
            &format!(
                "Démarrage audit (mode: {})",
                if full { "Complet" } else { "Standard" }
            ),
        );
    }

    let report = pieuvre_audit::full_audit()?;

    let json = serde_json::to_string_pretty(&report)?;

    // Déterminer le chemin de sortie
    let output_path = if let Some(path) = output {
        PathBuf::from(path)
    } else {
        // Sauvegarde automatique
        fs::create_dir_all(OUTPUT_DIR)?;
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        PathBuf::from(OUTPUT_DIR).join(format!("audit_{}.json", timestamp))
    };

    fs::write(&output_path, &json)?;

    if let Some(ref mut cb) = log_cb {
        cb(
            "SUCCESS",
            &format!("Audit terminé. Rapport : {}", output_path.display()),
        );
        cb(
            "INFO",
            &format!(
                "OS: {} (Build {})",
                report.system.os_version, report.system.build_number
            ),
        );
        cb("INFO", &format!("CPU: {}", report.hardware.cpu.model_name));
    } else {
        // Afficher un résumé console classique
        println!("═══════════════════════════════════════════════════════════════════");
        println!("                         RÉSUMÉ AUDIT");
        println!("═══════════════════════════════════════════════════════════════════");
        println!("  ID:         {}", report.id);
        println!("  Timestamp:  {}", report.timestamp);
        println!(
            "  OS:         {} (Build {})",
            report.system.os_version, report.system.build_number
        );
        println!("  CPU:        {}", report.hardware.cpu.model_name);
        println!(
            "  RAM:        {:.1} GB",
            report.hardware.memory.total_bytes as f64 / 1024.0 / 1024.0 / 1024.0
        );
        println!("  Services:   {} analysés", report.services.len());
        println!("  Packages:   {} Appx", report.appx.len());
        println!("═══════════════════════════════════════════════════════════════════");
        println!("\n  [*] Rapport sauvegarde: {}", output_path.display());
    }

    Ok(report)
}
