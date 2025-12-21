//! Modèles de données UI
//!
//! Structures pour le binding Rust ↔ Slint.

use serde::{Deserialize, Serialize};

/// Niveau de risque d'une optimisation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum RiskLevel {
    #[default]
    None,
    Low,
    Medium,
    High,
}

/// Item d'optimisation pour l'UI
#[derive(Debug, Clone, Default)]
pub struct OptimizationItem {
    pub id: String,
    pub label: String,
    pub description: String,
    pub category: String,
    pub risk_level: RiskLevel,
    pub is_selected: bool,
    pub is_applicable: bool,
}

/// Item de snapshot pour l'UI
#[derive(Debug, Clone, Default)]
pub struct SnapshotItem {
    pub id: String,
    pub timestamp: String,
    pub description: String,
    pub changes_count: i32,
}

/// État d'un service pour l'UI
#[derive(Debug, Clone, Default)]
pub struct ServiceItem {
    pub name: String,
    pub display_name: String,
    pub status: String,
    pub start_type: String,
    pub category: String,
}

/// Informations système pour l'UI
#[derive(Debug, Clone, Default)]
pub struct SystemInfoUI {
    pub os_version: String,
    pub build_number: String,
    pub hostname: String,
    pub cpu_name: String,
    pub cpu_cores: i32,
    pub ram_gb: i32,
    pub gpu_name: String,
    pub is_laptop: bool,
}

/// État global de l'application
#[derive(Debug, Clone, Default)]
pub struct AppStateModel {
    pub current_view: String,
    pub active_profile: String,
    pub is_loading: bool,
    pub last_action: String,
    pub selected_count: i32,
}
