//! Commande status
//!
//! Affiche l'état actuel du système et des optimisations.

use anyhow::Result;
use pieuvre_sync::timer;

pub fn run() -> Result<()> {
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║              🐙 PIEUVRE - Status Système                         ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");
    
    // Timer resolution
    println!("═══════════════════════════════════════════════════════════════════");
    println!("                      TIMER RESOLUTION");
    println!("═══════════════════════════════════════════════════════════════════");
    match timer::get_timer_resolution() {
        Ok(info) => {
            let status = if info.current_ms() <= 1.0 { "✓ Optimisé" } else { "⚠ Standard" };
            println!("  Actuelle:      {:.2}ms {}", info.current_ms(), status);
            println!("  Minimum:       {:.2}ms", info.min_ms());
            println!("  Maximum:       {:.2}ms", info.max_ms());
        }
        Err(e) => {
            println!("  Erreur: {}", e);
        }
    }
    
    // Télémétrie
    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("                       TÉLÉMÉTRIE");
    println!("═══════════════════════════════════════════════════════════════════");
    match pieuvre_audit::registry::get_telemetry_status() {
        Ok(status) => {
            let diag_icon = if status.diagtrack_enabled { "❌" } else { "✓" };
            println!("  DiagTrack:      {} {}", diag_icon, if status.diagtrack_enabled { "Actif" } else { "Désactivé" });
            println!("  Data Level:     {} ({})", status.data_collection_level, 
                match status.data_collection_level {
                    0 => "Security",
                    1 => "Basic",
                    2 => "Enhanced",
                    _ => "Full",
                });
            let adv_icon = if status.advertising_id_enabled { "❌" } else { "✓" };
            println!("  Advertising:    {} {}", adv_icon, if status.advertising_id_enabled { "Actif" } else { "Désactivé" });
        }
        Err(_) => {
            println!("  Erreur lecture registre");
        }
    }
    
    // Power Plan
    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("                       POWER PLAN");
    println!("═══════════════════════════════════════════════════════════════════");
    match pieuvre_sync::power::get_active_power_plan() {
        Ok(plan) => {
            println!("  Plan actif:     {}", plan);
        }
        Err(_) => {
            println!("  Plan actif:     Inconnu");
        }
    }
    
    // Snapshots
    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("                       SNAPSHOTS");
    println!("═══════════════════════════════════════════════════════════════════");
    match pieuvre_persist::list_snapshots() {
        Ok(snapshots) => {
            println!("  Disponibles:    {}", snapshots.len());
            if !snapshots.is_empty() {
                println!("  Dernier:        {} - {}", 
                    snapshots[0].timestamp.format("%Y-%m-%d %H:%M"),
                    snapshots[0].description);
            }
        }
        Err(_) => {
            println!("  Disponibles:    0");
        }
    }
    
    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("💡 Commandes utiles:");
    println!("   pieuvre audit --full           Audit complet");
    println!("   pieuvre analyze --profile gaming");
    println!("   pieuvre sync --profile gaming");
    println!("═══════════════════════════════════════════════════════════════════\n");
    
    Ok(())
}
