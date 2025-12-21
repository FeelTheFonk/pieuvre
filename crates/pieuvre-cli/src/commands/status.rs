//! Commande status

use anyhow::Result;
use pieuvre_sync::timer;

pub fn run() -> Result<()> {
    println!("=== Pieuvre Status ===\n");
    
    // Timer resolution
    match timer::get_timer_resolution() {
        Ok(info) => {
            println!("Timer Resolution:");
            println!("  Actuelle: {:.2}ms", info.current_ms());
            println!("  Meilleure possible: {:.2}ms", info.best_ms());
        }
        Err(e) => {
            println!("Timer Resolution: Erreur ({})", e);
        }
    }
    
    // Snapshots
    match pieuvre_persist::list_snapshots() {
        Ok(snapshots) => {
            println!("\nSnapshots: {} disponibles", snapshots.len());
        }
        Err(_) => {
            println!("\nSnapshots: 0 disponibles");
        }
    }
    
    Ok(())
}
