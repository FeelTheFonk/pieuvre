//! Audit command
//!
//! Complete system audit with automatic report generation.

use anyhow::Result;
use chrono::Local;
use pieuvre_common::AuditReport;
use std::fs;
use std::path::PathBuf;

const OUTPUT_DIR: &str = r"C:\ProgramData\pieuvre\reports";

/// Callback type for audit logging
pub type AuditLogCallback<'a> = &'a mut dyn FnMut(&str, &str);

pub fn run(
    full: bool,
    output: Option<String>,
    mut log_cb: Option<AuditLogCallback>,
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
                "Starting audit (mode: {})",
                if full { "Full" } else { "Standard" }
            ),
        );
    }

    let report = pieuvre_audit::full_audit()?;

    let json = serde_json::to_string_pretty(&report)?;

    // Determine output path
    let output_path = if let Some(path) = output {
        PathBuf::from(path)
    } else {
        // Automatic backup
        fs::create_dir_all(OUTPUT_DIR)?;
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        PathBuf::from(OUTPUT_DIR).join(format!("audit_{}.json", timestamp))
    };

    fs::write(&output_path, &json)?;

    if let Some(ref mut cb) = log_cb {
        cb(
            "SUCCESS",
            &format!("Audit completed. Report: {}", output_path.display()),
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
        // Standard console summary
        println!("═══════════════════════════════════════════════════════════════════");
        println!("                         AUDIT SUMMARY");
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
        println!("  Services:   {} analyzed", report.services.len());
        println!("  Packages:   {} Appx", report.appx.len());
        println!("═══════════════════════════════════════════════════════════════════");
        println!("\n  [*] Report saved: {}", output_path.display());
    }

    Ok(report)
}
