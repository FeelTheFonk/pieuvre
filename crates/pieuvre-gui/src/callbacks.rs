//! Callbacks UI → Rust
//!
//! Handlers pour les actions déclenchées depuis l'UI Slint.

/// Configure tous les callbacks de l'application
pub fn setup_callbacks<T>(_app: &T) {
    tracing::info!("Configuration callbacks UI");
    
    // TODO: Implémenter quand MainWindow est générée
    // Les callbacks seront connectés ici:
    // - on_run_audit
    // - on_apply_optimizations
    // - on_load_profile
    // - on_rollback_snapshot
    // - on_toggle_optimization
    // - on_navigate
}

/// Handler pour lancer un audit
pub fn handle_run_audit() {
    tracing::info!("Démarrage audit système");
    // TODO: Appeler pieuvre_audit
}

/// Handler pour appliquer les optimisations
pub fn handle_apply_optimizations(_dry_run: bool) {
    tracing::info!("Application optimisations");
    // TODO: Appeler pieuvre_sync
}

/// Handler pour charger un profil
pub fn handle_load_profile(_profile: &str) {
    tracing::info!("Chargement profil");
    // TODO: Charger config profil
}

/// Handler pour rollback snapshot
pub fn handle_rollback_snapshot(_snapshot_id: &str) {
    tracing::info!("Rollback snapshot");
    // TODO: Appeler pieuvre_persist
}
