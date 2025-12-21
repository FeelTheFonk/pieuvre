//! Initialisation de l'état UI
//!
//! Setup initial au démarrage de l'application.

use crate::models::SystemInfoUI;

/// Configure l'état initial de l'application
pub fn setup_initial_state<T>(_app: &T) {
    tracing::info!("Initialisation état UI");
    
    // TODO: Implémenter quand MainWindow est générée
    // - Détecter laptop/desktop
    // - Charger config default.toml
    // - Charger profils disponibles
    // - Audit rapide services critiques
}

/// Récupère les informations système
pub fn get_system_info() -> SystemInfoUI {
    // TODO: Implémenter avec pieuvre_audit
    SystemInfoUI {
        os_version: "Windows 11".into(),
        build_number: "22631".into(),
        hostname: "DESKTOP".into(),
        cpu_name: "Unknown CPU".into(),
        cpu_cores: 8,
        ram_gb: 16,
        gpu_name: "Unknown GPU".into(),
        is_laptop: false,
    }
}

/// Vérifie si le système est un laptop
pub fn detect_laptop() -> bool {
    // Utilise pieuvre_audit::hardware::is_laptop
    pieuvre_audit::hardware::is_laptop()
}
