//! Commande sync
//!
//! Application des profils d'optimisation.

use anyhow::Result;
use pieuvre_persist::snapshot;
use pieuvre_common::ChangeRecord;

pub fn run(profile: &str, dry_run: bool) -> Result<()> {
    println!();
    println!("================================================================");
    println!("                 PIEUVRE - Synchronisation");
    println!("================================================================");
    println!();
    
    println!("  Profil: {}", profile.to_uppercase());
    println!("  Mode:   {}", if dry_run { "SIMULATION (aucune modification)" } else { "APPLICATION REELLE" });
    println!();
    
    if !dry_run {
        // Creer un snapshot avant les modifications
        println!("[*] Creation snapshot de sauvegarde...");
        let changes = Vec::<ChangeRecord>::new(); // TODO: collecter les changements
        match snapshot::create(&format!("Avant profil {}", profile), changes) {
            Ok(snap) => println!("    Snapshot: {}", snap.id),
            Err(e) => println!("    Snapshot erreur: {}", e),
        }
        println!();
    }
    
    println!("----------------------------------------------------------------");
    println!("                      MODIFICATIONS");
    println!("----------------------------------------------------------------");
    
    pieuvre_sync::apply_profile(profile, dry_run)?;
    
    println!();
    println!("----------------------------------------------------------------");
    
    if dry_run {
        println!();
        println!("[OK] Simulation terminee. Pour appliquer reellement:");
        println!("     pieuvre sync --profile {}", profile);
    } else {
        println!();
        println!("[OK] Profil {} applique avec succes!", profile.to_uppercase());
        println!();
        println!("Note: Un snapshot a ete cree. Pour annuler:");
        println!("      pieuvre rollback --last");
    }
    
    println!();
    Ok(())
}
