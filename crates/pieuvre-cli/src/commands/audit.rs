//! Commande audit

use anyhow::Result;

pub fn run(full: bool, output: Option<String>) -> Result<()> {
    tracing::info!("Démarrage audit (full: {})", full);
    
    let report = pieuvre_audit::full_audit()?;
    
    let json = serde_json::to_string_pretty(&report)?;
    
    if let Some(path) = output {
        std::fs::write(&path, &json)?;
        println!("Rapport sauvegardé: {}", path);
    } else {
        println!("{}", json);
    }
    
    Ok(())
}
