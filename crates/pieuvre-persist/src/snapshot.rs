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

/// Restaure un snapshot
pub fn restore(id: &str) -> Result<()> {
    let snapshots = list_all()?;
    let snapshot = snapshots
        .into_iter()
        .find(|s| s.id.to_string().starts_with(id))
        .ok_or_else(|| PieuvreError::SnapshotNotFound(id.to_string()))?;
    
    tracing::info!("Restauration snapshot: {}", snapshot.id);
    
    for change in &snapshot.changes {
        match change {
            ChangeRecord::Registry { key, value_name, value_type: _, original_data: _ } => {
                tracing::debug!("Restauration registre: {}\\{}", key, value_name);
                // TODO: Implémenter restauration registre
            }
            ChangeRecord::Service { name, original_start_type } => {
                tracing::debug!("Restauration service: {} -> {}", name, original_start_type);
                // TODO: Implémenter restauration service
            }
            ChangeRecord::FirewallRule { name } => {
                tracing::debug!("Suppression règle firewall: {}", name);
                // TODO: Implémenter suppression règle
            }
        }
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
