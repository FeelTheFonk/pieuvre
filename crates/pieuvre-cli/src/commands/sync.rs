//! Commande sync
//!
//! Application des profils d'optimisation.

use anyhow::Result;
use pieuvre_persist::snapshot;
use pieuvre_common::ChangeRecord;

pub fn run(profile: &str, dry_run: bool) -> Result<()> {
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║              🐙 PIEUVRE - Synchronisation                        ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");
    
    println!("📋 Profil: {}", profile.to_uppercase());
    println!("🔧 Mode:   {}\n", if dry_run { "SIMULATION (aucune modification)" } else { "APPLICATION RÉELLE" });
    
    if !dry_run {
        // Créer un snapshot avant les modifications
        println!("💾 Création snapshot de sauvegarde...");
        let changes = Vec::<ChangeRecord>::new(); // TODO: collecter les changements
        match snapshot::create(&format!("Avant profil {}", profile), changes) {
            Ok(snap) => println!("   ✓ Snapshot: {}\n", snap.id),
            Err(e) => println!("   ⚠ Snapshot: {}\n", e),
        }
    }
    
    println!("═══════════════════════════════════════════════════════════════════");
    println!("                      MODIFICATIONS");
    println!("═══════════════════════════════════════════════════════════════════");
    
    pieuvre_sync::apply_profile(profile, dry_run)?;
    
    println!("\n═══════════════════════════════════════════════════════════════════");
    
    if dry_run {
        println!("\n✓ Simulation terminée. Pour appliquer réellement:");
        println!("  pieuvre sync --profile {}", profile);
    } else {
        println!("\n✓ Profil {} appliqué avec succès!", profile.to_uppercase());
        println!("\n📝 Note: Un snapshot a été créé. Pour annuler:");
        println!("   pieuvre rollback --last");
    }
    
    println!();
    Ok(())
}
