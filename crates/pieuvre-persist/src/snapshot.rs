//! Snapshot Management
//!
//! Backup and restoration of system modifications.
//! zstd compression and SHA256 checksum validation.

use chrono::Utc;
use pieuvre_common::{ChangeRecord, PieuvreError, Result, Snapshot};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

const SNAPSHOT_DIR: &str = r"C:\ProgramData\pieuvre\snapshots";
const DEFAULT_MAX_SNAPSHOTS: usize = 10;

// ============================================
// SNAPSHOT CREATION
// ============================================

/// Creates a new snapshot with compression and checksum
pub fn create(description: &str, changes: Vec<ChangeRecord>) -> Result<Snapshot> {
    let snapshot = Snapshot {
        id: Uuid::new_v4(),
        timestamp: Utc::now(),
        description: description.to_string(),
        changes,
    };

    let dir = PathBuf::from(SNAPSHOT_DIR);
    fs::create_dir_all(&dir)?;

    // Serialize to JSON
    let json =
        serde_json::to_string_pretty(&snapshot).map_err(|e| PieuvreError::Parse(e.to_string()))?;

    // Save with compression
    save_compressed(&dir, &snapshot.id.to_string(), json.as_bytes())?;

    // Automatic rotation
    rotate_snapshots(&dir)?;

    tracing::info!(id = %snapshot.id, description = %snapshot.description, "Snapshot created");
    Ok(snapshot)
}

/// Saves data with zstd compression and SHA256 checksum
fn save_compressed(dir: &Path, id: &str, data: &[u8]) -> Result<()> {
    // Calculate SHA256 checksum
    let mut hasher = Sha256::new();
    hasher.update(data);
    let checksum = hasher.finalize();
    let checksum_hex = hex_encode(&checksum);

    // Compress with zstd (level 3 = good size/speed ratio)
    let compressed =
        zstd::encode_all(data, 3).map_err(|e| PieuvreError::Io(std::io::Error::other(e)))?;

    // Save compressed file
    let path = dir.join(format!("{}.json.zst", id));
    fs::write(&path, &compressed)?;

    // Save checksum
    let checksum_path = dir.join(format!("{}.sha256", id));
    fs::write(&checksum_path, &checksum_hex)?;

    tracing::debug!(
        id = id,
        original_size = data.len(),
        compressed_size = compressed.len(),
        ratio = format!("{:.1}x", data.len() as f64 / compressed.len() as f64),
        "Snapshot compressed"
    );

    Ok(())
}

/// Loads a snapshot with decompression and checksum validation
fn load_compressed(dir: &Path, id: &str) -> Result<Snapshot> {
    // Look for compressed or uncompressed file (backward compatibility)
    let zst_path = dir.join(format!("{}.json.zst", id));
    let json_path = dir.join(format!("{}.json", id));

    let data = if zst_path.exists() {
        // Compressed file
        let compressed = fs::read(&zst_path)?;
        zstd::decode_all(compressed.as_slice())
            .map_err(|e| PieuvreError::Parse(format!("Decompression failed: {}", e)))?
    } else if json_path.exists() {
        // Uncompressed JSON file (legacy)
        fs::read(&json_path)?
    } else {
        return Err(PieuvreError::SnapshotNotFound(id.to_string()));
    };

    // Validate checksum if present
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
        tracing::debug!(id = id, "Checksum validated");
    }

    let snapshot: Snapshot =
        serde_json::from_slice(&data).map_err(|e| PieuvreError::Parse(e.to_string()))?;

    Ok(snapshot)
}

/// Encodes bytes to hex string
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

// ============================================
// ROTATION AUTOMATIQUE
// ============================================

/// Automatic snapshot rotation (keeps N most recent)
fn rotate_snapshots(dir: &Path) -> Result<()> {
    let mut snapshots = list_all_internal(dir)?;

    if snapshots.len() <= DEFAULT_MAX_SNAPSHOTS {
        return Ok(());
    }

    // Sort by date (most recent first)
    snapshots.sort_by(|a, b| b.1.cmp(&a.1));

    // Remove oldest
    let to_remove = snapshots.len() - DEFAULT_MAX_SNAPSHOTS;
    for (id, _) in snapshots.iter().skip(DEFAULT_MAX_SNAPSHOTS) {
        let _ = delete_by_id(dir, id);
        tracing::debug!(id = id, "Snapshot deleted (rotation)");
    }

    tracing::info!(
        removed = to_remove,
        max = DEFAULT_MAX_SNAPSHOTS,
        "Snapshot rotation"
    );

    Ok(())
}

/// Internal list returning (id, timestamp)
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

        // Try to load to get timestamp
        if let Ok(snapshot) = load_compressed(dir, id) {
            snapshots.push((id.to_string(), snapshot.timestamp));
        }
    }

    Ok(snapshots)
}

/// Deletes a snapshot by ID
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

/// Lists all snapshots
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

    // Sort by date descending
    snapshots.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    Ok(snapshots)
}

/// Loads a snapshot by ID
pub fn load(id: &str) -> Result<Snapshot> {
    let dir = PathBuf::from(SNAPSHOT_DIR);
    load_compressed(&dir, id)
}

/// Restores a snapshot (applies original values)
pub fn restore(id: &str) -> Result<()> {
    let dir = PathBuf::from(SNAPSHOT_DIR);

    // Search for snapshot starting with provided ID
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

    tracing::info!(id = %snapshot.id, "Restoring snapshot");

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
                tracing::debug!(key = key, value_name = value_name, "Restoring registry");

                // Restore original DWORD value if possible
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
                                "Registry restored"
                            );
                            restored += 1;
                        }
                        Err(e) => {
                            tracing::warn!(key = key, error = %e, "Registry restoration failed");
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
                    "Restoring service"
                );

                let result =
                    pieuvre_sync::services::set_service_start_type(name, *original_start_type);

                match result {
                    Ok(_) => {
                        tracing::info!(
                            service = name,
                            start_type = original_start_type,
                            "Service restored"
                        );
                        restored += 1;
                    }
                    Err(e) => {
                        tracing::warn!(service = name, error = %e, "Service restoration failed");
                        errors += 1;
                    }
                }
            }
            ChangeRecord::FirewallRule { name } => {
                tracing::debug!(rule = name, "Removing firewall rule");

                if let Err(e) = pieuvre_sync::firewall::remove_pieuvre_rules() {
                    tracing::warn!(rule = name, error = %e, "Rule removal failed");
                    errors += 1;
                } else {
                    restored += 1;
                }
            }
            ChangeRecord::AppX { package_full_name } => {
                tracing::debug!(package = package_full_name, "Restoring AppX (no-op)");
                restored += 1;
            }
        }
    }

    tracing::info!(
        restored = restored,
        errors = errors,
        "Restoration completed"
    );

    if errors > 0 {
        tracing::warn!(errors = errors, "Some restorations failed");
    }

    Ok(())
}

/// Deletes a snapshot
pub fn delete(id: &str) -> Result<()> {
    let dir = PathBuf::from(SNAPSHOT_DIR);

    // Search for corresponding file
    let zst_path = dir.join(format!("{}.json.zst", id));
    let json_path = dir.join(format!("{}.json", id));
    let exists = zst_path.exists() || json_path.exists();

    if !exists {
        return Err(PieuvreError::SnapshotNotFound(id.to_string()));
    }

    delete_by_id(&dir, id)?;

    tracing::info!(id = id, "Snapshot deleted");
    Ok(())
}

/// Returns the snapshot directory path
pub fn get_snapshot_dir() -> PathBuf {
    PathBuf::from(SNAPSHOT_DIR)
}
