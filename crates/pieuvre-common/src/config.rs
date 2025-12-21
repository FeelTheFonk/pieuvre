//! Configuration Pieuvre
//!
//! Gestion des fichiers de configuration TOML et profils.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration globale Pieuvre
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PieuvreConfig {
    /// Profil actif
    pub profile: String,
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
            profile: "balanced".into(),
            snapshot_dir: PathBuf::from(r"C:\ProgramData\Pieuvre\snapshots"),
            log_level: "info".into(),
            dry_run: false,
        }
    }
}

/// Profil d'optimisation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub description: String,
    pub risk_level: RiskLevel,
    pub timer: Option<TimerConfig>,
    pub scheduler: Option<SchedulerConfig>,
    pub power: Option<PowerConfig>,
    pub services: Option<ServicesConfig>,
    pub network: Option<NetworkConfig>,
    pub telemetry: Option<TelemetryConfig>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerConfig {
    /// Résolution en unités de 100ns (5000 = 0.5ms)
    pub resolution_100ns: u32,
    pub force_high_precision: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerConfig {
    /// Valeur Win32PrioritySeparation
    pub win32_priority_separation: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerConfig {
    pub plan: String,
    pub usb_selective_suspend: bool,
    pub pci_express_aspm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicesConfig {
    pub disable: Vec<String>,
    pub manual: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub nagle_algorithm: bool,
    pub network_throttling_index: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryConfig {
    pub level: u32,
    pub block_domains: bool,
}
