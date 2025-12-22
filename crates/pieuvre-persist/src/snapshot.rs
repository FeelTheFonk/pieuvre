//! Gestion des snapshots SOTA 2026
//!
//! Sauvegarde et restauration des modifications.
//! Compression zstd et validation checksums SHA256.

use chrono::Utc;
use pieuvre_common::{ChangeRecord, PieuvreError, Result, Snapshot};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

const SNAPSHOT_DIR: &str = r"C:\ProgramData\Pieuvre\snapshots";
const DEFAULT_MAX_SNAPSHOTS: usize = 10;

// ============================================
// SNAPSHOT CREATION
// ============================================

/// Crée un nouveau snapshot avec compression et checksum
pub fn create(description: &str, changes: Vec<ChangeRecord>) -> Result<Snapshot> {
    let snapshot = Snapshot {
        id: Uuid::new_v4(),
        timestamp: Utc::now(),
        description: description.to_string(),
        changes,
    };

    let dir = PathBuf::from(SNAPSHOT_DIR);
    fs::create_dir_all(&dir)?;

    // Sérialiser en JSON
    let json =
        serde_json::to_string_pretty(&snapshot).map_err(|e| PieuvreError::Parse(e.to_string()))?;

    // Sauvegarder avec compression
    save_compressed(&dir, &snapshot.id.to_string(), json.as_bytes())?;

    // Rotation automatique
    rotate_snapshots(&dir)?;

    tracing::info!(id = %snapshot.id, description = %snapshot.description, "Snapshot créé");
    Ok(snapshot)
}

/// Sauvegarde les données avec compression zstd et checksum SHA256
fn save_compressed(dir: &Path, id: &str, data: &[u8]) -> Result<()> {
    // Calculer checksum SHA256
    let mut hasher = Sha256::new();
    hasher.update(data);
    let checksum = hasher.finalize();
    let checksum_hex = hex_encode(&checksum);

    // Compresser avec zstd (niveau 3 = bon ratio taille/vitesse)
    let compressed =
        zstd::encode_all(data, 3).map_err(|e| PieuvreError::Io(std::io::Error::other(e)))?;

    // Sauvegarder fichier compressé
    let path = dir.join(format!("{}.json.zst", id));
    fs::write(&path, &compressed)?;

    // Sauvegarder checksum
    let checksum_path = dir.join(format!("{}.sha256", id));
    fs::write(&checksum_path, &checksum_hex)?;

    tracing::debug!(
        id = id,
        original_size = data.len(),
        compressed_size = compressed.len(),
        ratio = format!("{:.1}x", data.len() as f64 / compressed.len() as f64),
        "Snapshot compressé"
    );

    Ok(())
}

/// Charge un snapshot avec décompression et validation checksum
fn load_compressed(dir: &Path, id: &str) -> Result<Snapshot> {
    // Chercher fichier compressé ou non-compressé (rétro-compatibilité)
    let zst_path = dir.join(format!("{}.json.zst", id));
    let json_path = dir.join(format!("{}.json", id));

    let data = if zst_path.exists() {
        // Fichier compressé
        let compressed = fs::read(&zst_path)?;
        zstd::decode_all(compressed.as_slice())
            .map_err(|e| PieuvreError::Parse(format!("Decompression failed: {}", e)))?
    } else if json_path.exists() {
        // Fichier JSON non-compressé (ancienne version)
        fs::read(&json_path)?
    } else {
        return Err(PieuvreError::SnapshotNotFound(id.to_string()));
    };

    // Valider checksum si présent
    let checksum_path = dir.join(format!("{}.sha256", id));
    if checksum_path.exists() {
        let expected = fs::read_to_string(&checksum_path)?;

        let mut hasher = Sha256::new();
        hasher.update(&data);
        let actual = hex_encode(&hasher.finalize());

        if expected.trim() != actual {
            return Err(PieuvreError::Parse(format!(
                "Checksum mismatch: expected {}, got {}",
                expected.trim(),
                actual
            )));
        }
        tracing::debug!(id = id, "Checksum validé");
    }

    let snapshot: Snapshot =
        serde_json::from_slice(&data).map_err(|e| PieuvreError::Parse(e.to_string()))?;

    Ok(snapshot)
}

/// Encode bytes en hex string
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

// ============================================
// ROTATION AUTOMATIQUE
// ============================================

/// Rotation automatique des snapshots (garde les N plus récents)
fn rotate_snapshots(dir: &Path) -> Result<()> {
    let mut snapshots = list_all_internal(dir)?;

    if snapshots.len() <= DEFAULT_MAX_SNAPSHOTS {
        return Ok(());
    }

    // Trier par date (plus récent en premier)
    snapshots.sort_by(|a, b| b.1.cmp(&a.1));

    // Supprimer les plus anciens
    let to_remove = snapshots.len() - DEFAULT_MAX_SNAPSHOTS;
    for (id, _) in snapshots.iter().skip(DEFAULT_MAX_SNAPSHOTS) {
        let _ = delete_by_id(dir, id);
        tracing::debug!(id = id, "Snapshot supprimé (rotation)");
    }

    tracing::info!(
        removed = to_remove,
        max = DEFAULT_MAX_SNAPSHOTS,
        "Rotation snapshots"
    );

    Ok(())
}

/// Liste interne retournant (id, timestamp)
fn list_all_internal(dir: &Path) -> Result<Vec<(String, chrono::DateTime<Utc>)>> {
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut snapshots = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        // Supporter .json et .json.zst
        let id = if filename.ends_with(".json.zst") {
            filename.trim_end_matches(".json.zst")
        } else if filename.ends_with(".json") && !filename.ends_with(".sha256") {
            filename.trim_end_matches(".json")
        } else {
            continue;
        };

        // Essayer de charger pour obtenir le timestamp
        if let Ok(snapshot) = load_compressed(dir, id) {
            snapshots.push((id.to_string(), snapshot.timestamp));
        }
    }

    Ok(snapshots)
}

/// Supprime un snapshot par ID
fn delete_by_id(dir: &Path, id: &str) -> Result<()> {
    let zst_path = dir.join(format!("{}.json.zst", id));
    let json_path = dir.join(format!("{}.json", id));
    let checksum_path = dir.join(format!("{}.sha256", id));

    if zst_path.exists() {
        fs::remove_file(&zst_path)?;
    }
    if json_path.exists() {
        fs::remove_file(&json_path)?;
    }
    if checksum_path.exists() {
        fs::remove_file(&checksum_path)?;
    }

    Ok(())
}

// ============================================
// API PUBLIQUE
// ============================================

/// Liste tous les snapshots
pub fn list_all() -> Result<Vec<Snapshot>> {
    let dir = PathBuf::from(SNAPSHOT_DIR);

    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut snapshots = Vec::new();

    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        // Supporter .json et .json.zst
        let id = if filename.ends_with(".json.zst") {
            filename.trim_end_matches(".json.zst")
        } else if filename.ends_with(".json") && !filename.contains(".sha256") {
            filename.trim_end_matches(".json")
        } else {
            continue;
        };

        if let Ok(snapshot) = load_compressed(&dir, id) {
            snapshots.push(snapshot);
        }
    }

    // Trier par date décroissante
    snapshots.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    Ok(snapshots)
}

/// Restaure un snapshot (applique les valeurs originales)
pub fn restore(id: &str) -> Result<()> {
    let dir = PathBuf::from(SNAPSHOT_DIR);

    // Chercher snapshot qui commence par l'ID fourni
    let mut found_id: Option<String> = None;

    if dir.exists() {
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

            if filename.starts_with(id) {
                let snapshot_id = if filename.ends_with(".json.zst") {
                    filename.trim_end_matches(".json.zst")
                } else if filename.ends_with(".json") {
                    filename.trim_end_matches(".json")
                } else {
                    continue;
                };
                found_id = Some(snapshot_id.to_string());
                break;
            }
        }
    }

    let snapshot_id = found_id.ok_or_else(|| PieuvreError::SnapshotNotFound(id.to_string()))?;
    let snapshot = load_compressed(&dir, &snapshot_id)?;

    tracing::info!(id = %snapshot.id, "Restauration snapshot");

    let mut restored = 0;
    let mut errors = 0;

    for change in &snapshot.changes {
        match change {
            ChangeRecord::Registry {
                key,
                value_name,
                value_type: _,
                original_data,
            } => {
                tracing::debug!(key = key, value_name = value_name, "Restauration registre");

                // Restaurer la valeur DWORD originale si possible
                if original_data.len() == 4 {
                    let value = u32::from_le_bytes([
                        original_data[0],
                        original_data[1],
                        original_data[2],
                        original_data[3],
                    ]);

                    match pieuvre_sync::registry::set_dword_value(key, value_name, value) {
                        Ok(_) => {
                            tracing::info!(
                                key = key,
                                value_name = value_name,
                                value = value,
                                "Registry restauré"
                            );
                            restored += 1;
                        }
                        Err(e) => {
                            tracing::warn!(key = key, error = %e, "Échec restauration registry");
                            errors += 1;
                        }
                    }
                }
            }
            ChangeRecord::Service {
                name,
                original_start_type,
            } => {
                tracing::debug!(
                    service = name,
                    start_type = original_start_type,
                    "Restauration service"
                );

                let result = match *original_start_type {
                    2 => pieuvre_sync::services::set_service_automatic(name),
                    3 => pieuvre_sync::services::set_service_manual(name),
                    4 => pieuvre_sync::services::disable_service(name),
                    _ => {
                        tracing::warn!(
                            service = name,
                            start_type = original_start_type,
                            "Start type non supporté"
                        );
                        Ok(())
                    }
                };

                match result {
                    Ok(_) => {
                        tracing::info!(
                            service = name,
                            start_type = original_start_type,
                            "Service restauré"
                        );
                        restored += 1;
                    }
                    Err(e) => {
                        tracing::warn!(service = name, error = %e, "Échec restauration service");
                        errors += 1;
                    }
                }
            }
            ChangeRecord::FirewallRule { name } => {
                tracing::debug!(rule = name, "Suppression règle firewall");

                if let Err(e) = pieuvre_sync::firewall::remove_pieuvre_rules() {
                    tracing::warn!(rule = name, error = %e, "Échec suppression règle");
                    errors += 1;
                } else {
                    restored += 1;
                }
            }
            ChangeRecord::AppX { package_full_name } => {
                tracing::debug!(package = package_full_name, "Restauration AppX (no-op)");
                restored += 1;
            }
        }
    }

    tracing::info!(
        restored = restored,
        errors = errors,
        "Restauration terminée"
    );

    if errors > 0 {
        tracing::warn!(errors = errors, "Certaines restaurations ont échoué");
    }

    Ok(())
}

/// Supprime un snapshot
pub fn delete(id: &str) -> Result<()> {
    let dir = PathBuf::from(SNAPSHOT_DIR);

    // Chercher fichier correspondant
    let zst_path = dir.join(format!("{}.json.zst", id));
    let json_path = dir.join(format!("{}.json", id));
    let exists = zst_path.exists() || json_path.exists();

    if !exists {
        return Err(PieuvreError::SnapshotNotFound(id.to_string()));
    }

    delete_by_id(&dir, id)?;

    tracing::info!(id = id, "Snapshot supprimé");
    Ok(())
}

/// Retourne le chemin du répertoire snapshots
pub fn get_snapshot_dir() -> PathBuf {
    PathBuf::from(SNAPSHOT_DIR)
}
