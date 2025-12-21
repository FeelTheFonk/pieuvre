//! Gestion des snapshots
//!
//! Sauvegarde et restauration des modifications.

use pieuvre_common::{ChangeRecord, PieuvreError, Result, Snapshot};
use chrono::Utc;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

const SNAPSHOT_DIR: &str = r"C:\ProgramData\Pieuvre\snapshots";

/// Crée un nouveau snapshot
pub fn create(description: &str, changes: Vec<ChangeRecord>) -> Result<Snapshot> {
    let snapshot = Snapshot {
        id: Uuid::new_v4(),
        timestamp: Utc::now(),
        description: description.to_string(),
        changes,
    };
    
    let dir = PathBuf::from(SNAPSHOT_DIR);
    fs::create_dir_all(&dir)?;
    
    let path = dir.join(format!("{}.json", snapshot.id));
    let json = serde_json::to_string_pretty(&snapshot)
        .map_err(|e| PieuvreError::Parse(e.to_string()))?;
    fs::write(&path, json)?;
    
    tracing::info!("Snapshot créé: {}", snapshot.id);
    Ok(snapshot)
}

/// Liste tous les snapshots
pub fn list_all() -> Result<Vec<Snapshot>> {
    let dir = PathBuf::from(SNAPSHOT_DIR);
    
    if !dir.exists() {
        return Ok(Vec::new());
    }
    
    let mut snapshots = Vec::new();
    
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().map_or(false, |e| e == "json") {
            let content = fs::read_to_string(&path)?;
            if let Ok(snapshot) = serde_json::from_str::<Snapshot>(&content) {
                snapshots.push(snapshot);
            }
        }
    }
    
    // Trier par date décroissante
    snapshots.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    
    Ok(snapshots)
}

/// Restaure un snapshot (applique les valeurs originales)
pub fn restore(id: &str) -> Result<()> {
    let snapshots = list_all()?;
    let snapshot = snapshots
        .into_iter()
        .find(|s| s.id.to_string().starts_with(id))
        .ok_or_else(|| PieuvreError::SnapshotNotFound(id.to_string()))?;
    
    tracing::info!("Restauration snapshot: {}", snapshot.id);
    
    let mut restored = 0;
    let mut errors = 0;
    
    for change in &snapshot.changes {
        match change {
            ChangeRecord::Registry { key, value_name, value_type: _, original_data } => {
                tracing::debug!("Restauration registre: {}\\{}", key, value_name);
                
                // Restaurer la valeur DWORD originale
                if original_data.len() == 4 {
                    let value = u32::from_le_bytes([
                        original_data[0], original_data[1], 
                        original_data[2], original_data[3]
                    ]);
                    
                    match pieuvre_sync::registry::set_dword_value(key, value_name, value) {
                        Ok(_) => {
                            tracing::info!("Registry restauré: {}\\{} = {}", key, value_name, value);
                            restored += 1;
                        }
                        Err(e) => {
                            tracing::warn!("Échec restauration registry: {}", e);
                            errors += 1;
                        }
                    }
                }
            }
            ChangeRecord::Service { name, original_start_type } => {
                tracing::debug!("Restauration service: {} -> {}", name, original_start_type);
                
                // Restaurer le start type original
                let result = match *original_start_type {
                    2 => pieuvre_sync::services::set_service_automatic(name),
                    3 => pieuvre_sync::services::set_service_manual(name),
                    4 => pieuvre_sync::services::disable_service(name),
                    _ => {
                        tracing::warn!("Start type {} non supporté pour {}", original_start_type, name);
                        Ok(())
                    }
                };
                
                match result {
                    Ok(_) => {
                        tracing::info!("Service restauré: {} -> start_type {}", name, original_start_type);
                        restored += 1;
                    }
                    Err(e) => {
                        tracing::warn!("Échec restauration service {}: {}", name, e);
                        errors += 1;
                    }
                }
            }
            ChangeRecord::FirewallRule { name } => {
                tracing::debug!("Suppression règle firewall: {}", name);
                
                // Supprimer la règle firewall ajoutée
                if let Err(e) = pieuvre_sync::firewall::remove_pieuvre_rules() {
                    tracing::warn!("Échec suppression règle {}: {}", name, e);
                    errors += 1;
                } else {
                    restored += 1;
                }
            }
        }
    }
    
    tracing::info!("Restauration terminée: {} restaurés, {} erreurs", restored, errors);
    
    if errors > 0 {
        tracing::warn!("Certaines restaurations ont échoué");
    }
    
    Ok(())
}

/// Supprime un snapshot
pub fn delete(id: &str) -> Result<()> {
    let path = PathBuf::from(SNAPSHOT_DIR).join(format!("{}.json", id));
    
    if path.exists() {
        fs::remove_file(&path)?;
        tracing::info!("Snapshot supprimé: {}", id);
        Ok(())
    } else {
        Err(PieuvreError::SnapshotNotFound(id.to_string()))
    }
}
