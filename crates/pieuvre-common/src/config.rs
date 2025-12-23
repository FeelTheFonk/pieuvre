//! Configuration pieuvre
//!
//! Gestion des fichiers de configuration TOML et profils.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration globale pieuvre
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PieuvreConfig {
    /// Chemin vers le dossier de snapshots
    pub snapshot_dir: PathBuf,
    /// Niveau de log
    pub log_level: String,
    /// Mode dry-run par défaut
    pub dry_run: bool,
}

impl Default for PieuvreConfig {
    fn default() -> Self {
        Self {
            snapshot_dir: PathBuf::from(r"C:\ProgramData\pieuvre\snapshots"),
            log_level: "info".into(),
            dry_run: false,
        }
    }
}
