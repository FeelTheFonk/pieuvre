//! Commande audit
//!
//! Audit complet du système avec sauvegarde automatique.

use anyhow::Result;
use chrono::Local;
use std::fs;
use std::path::PathBuf;

const OUTPUT_DIR: &str = r"C:\ProgramData\Pieuvre\reports";

pub fn run(full: bool, output: Option<String>) -> Result<()> {
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║              PIEUVRE - Audit Systeme                             ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    println!(
        "  [*] Mode: {}\n",
        if full { "Complet" } else { "Standard" }
    );

    tracing::info!("Démarrage audit (full: {})", full);

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

    // Afficher un résumé
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

    println!("\n  Prochaines etapes:");
    println!("    pieuvre analyze --profile gaming");
    println!("    pieuvre sync --profile gaming --dry-run");

    Ok(())
}
