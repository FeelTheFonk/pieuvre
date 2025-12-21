//! Commande sync

use anyhow::Result;

pub fn run(profile: &str, dry_run: bool) -> Result<()> {
    if dry_run {
        println!("Mode simulation - aucune modification");
    }
    
    println!("Application profil: {}", profile);
    
    pieuvre_sync::apply_profile(profile, dry_run)?;
    
    Ok(())
}
