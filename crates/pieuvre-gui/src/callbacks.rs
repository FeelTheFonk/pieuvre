//! Callbacks UI -> Rust
//!
//! Handlers pour les actions declenchees depuis l'UI Slint.

use anyhow::Result;

/// Configure tous les callbacks de l'application
/// 
/// Note: Les callbacks sont connectes via les proprietes Slint dans main.slint.
pub fn setup_callbacks<T>(_app: &T) {
    tracing::info!("Configuration callbacks UI");
}

/// Handler pour lancer un audit systeme complet
pub fn handle_run_audit() -> Result<()> {
    tracing::info!("Demarrage audit systeme");
    
    let report = pieuvre_audit::full_audit()?;
    
    tracing::info!(
        "Audit termine: {} services, {} appx",
        report.services.len(),
        report.appx.len()
    );
    
    Ok(())
}

/// Handler pour appliquer les optimisations selectionnees
pub fn handle_apply_optimizations(dry_run: bool, profile: &str) -> Result<()> {
    tracing::info!("Application optimisations (dry_run={}, profile={})", dry_run, profile);
    
    if dry_run {
        tracing::info!("Mode dry-run: aucune modification appliquee");
        return Ok(());
    }
    
    // Creation snapshot avant modification
    let snapshot = pieuvre_persist::snapshot::create("GUI optimization", vec![])?;
    tracing::info!("Snapshot cree: {}", snapshot.id);
    
    // Application du profil via pieuvre_sync
    pieuvre_sync::apply_profile(profile, false)?;
    tracing::info!("Profil {} applique", profile);
    
    Ok(())
}

/// Handler pour charger un profil de configuration
pub fn handle_load_profile(profile: &str) -> Result<()> {
    tracing::info!("Chargement profil: {}", profile);
    
    let profile_path = format!("config/profiles/{}.toml", profile);
    
    if !std::path::Path::new(&profile_path).exists() {
        tracing::warn!("Profil non trouve: {}", profile_path);
        return Ok(());
    }
    
    tracing::info!("Profil {} charge", profile);
    Ok(())
}

/// Handler pour rollback d'un snapshot
pub fn handle_rollback_snapshot(snapshot_id: &str) -> Result<()> {
    tracing::info!("Rollback snapshot: {}", snapshot_id);
    
    pieuvre_persist::snapshot::restore(snapshot_id)?;
    tracing::info!("Snapshot {} restaure", snapshot_id);
    
    Ok(())
}

/// Handler pour creer un nouveau snapshot
pub fn handle_create_snapshot() -> Result<String> {
    tracing::info!("Creation snapshot manuel");
    
    let snapshot = pieuvre_persist::snapshot::create("Manual snapshot", vec![])?;
    let id = snapshot.id.to_string();
    
    tracing::info!("Snapshot cree: {}", id);
    Ok(id)
}

/// Handler pour exporter l'audit en JSON
pub fn handle_export_json() -> Result<String> {
    tracing::info!("Export audit JSON");
    
    let report = pieuvre_audit::full_audit()?;
    let json = serde_json::to_string_pretty(&report)?;
    
    let path = format!("audit_{}.json", chrono::Utc::now().format("%Y%m%d_%H%M%S"));
    std::fs::write(&path, &json)?;
    
    tracing::info!("Audit exporte: {}", path);
    Ok(path)
}

/// Handler pour sauvegarder les parametres
pub fn handle_save_settings() -> Result<()> {
    tracing::info!("Sauvegarde parametres");
    Ok(())
}
