//! Nettoyage système profond SOTA 2026
//!
//! Gestion des fichiers temporaires, cache WinSxS et caches navigateurs.

use crate::operation::SyncOperation;
use async_trait::async_trait;
use pieuvre_common::{ChangeRecord, Result};
use std::fs;
use std::path::PathBuf;

/// Opération pour nettoyer les fichiers temporaires
pub struct CleanupTempOperation;

#[async_trait]
impl SyncOperation for CleanupTempOperation {
    fn name(&self) -> &str {
        "Cleanup Temporary Files"
    }

    async fn apply(&self) -> Result<Vec<ChangeRecord>> {
        let temp_dirs = vec![std::env::temp_dir(), PathBuf::from(r"C:\Windows\Temp")];

        for dir in temp_dirs {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        let _ = fs::remove_file(path);
                    } else if path.is_dir() {
                        let _ = fs::remove_dir_all(path);
                    }
                }
            }
        }

        Ok(vec![])
    }

    async fn is_applied(&self) -> Result<bool> {
        Ok(false) // Toujours applicable
    }
}

/// Opération pour nettoyer WinSxS (Windows Update)
pub struct CleanupWinSxSOperation;

#[async_trait]
impl SyncOperation for CleanupWinSxSOperation {
    fn name(&self) -> &str {
        "Cleanup WinSxS (Windows Update)"
    }

    async fn apply(&self) -> Result<Vec<ChangeRecord>> {
        // Utilisation de DISM pour le nettoyage sécurisé
        let _ = std::process::Command::new("dism.exe")
            .args([
                "/online",
                "/cleanup-image",
                "/startcomponentcleanup",
                "/quiet",
                "/norestart",
            ])
            .output();

        Ok(vec![])
    }

    async fn is_applied(&self) -> Result<bool> {
        Ok(false)
    }
}

/// Opération pour nettoyer le cache Edge
pub struct CleanupEdgeCacheOperation;

#[async_trait]
impl SyncOperation for CleanupEdgeCacheOperation {
    fn name(&self) -> &str {
        "Cleanup Edge Browser Cache"
    }

    async fn apply(&self) -> Result<Vec<ChangeRecord>> {
        if let Some(local_appdata) = std::env::var_os("LOCALAPPDATA") {
            let edge_cache =
                PathBuf::from(local_appdata).join(r"Microsoft\Edge\User Data\Default\Cache");

            if edge_cache.exists() {
                let _ = fs::remove_dir_all(&edge_cache);
            }
        }
        Ok(vec![])
    }

    async fn is_applied(&self) -> Result<bool> {
        Ok(false)
    }
}
