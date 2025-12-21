//! Pieuvre Sync Engine
//!
//! Module de synchronisation: application des optimisations.

pub mod appx;
pub mod context_menu;
pub mod edge;
pub mod explorer;
pub mod firewall;
pub mod game_mode;
pub mod hosts;
pub mod msi;
pub mod onedrive;
pub mod power;
pub mod registry;
pub mod scheduled_tasks;
pub mod services;
pub mod timer;
pub mod widgets;
pub mod windows_update;

use pieuvre_common::Result;
use std::path::Path;

/// Applique un profil d'optimisation
pub fn apply_profile(profile_name: &str, dry_run: bool) -> Result<()> {
    tracing::info!("Application profil: {} (dry_run: {})", profile_name, dry_run);
    
    // Charger le profil TOML
    let profile_path = format!("config/profiles/{}.toml", profile_name);
    
    if !Path::new(&profile_path).exists() {
        tracing::warn!("Profil {} non trouvé, utilisation des defaults", profile_name);
    }
    
    if dry_run {
        tracing::info!("[DRY-RUN] Aucune modification appliquée");
        return Ok(());
    }
    
    // Appliquer selon le type de profil
    match profile_name {
        "gaming" => apply_gaming_profile()?,
        "privacy" => apply_privacy_profile()?,
        "workstation" => apply_workstation_profile()?,
        _ => {
            tracing::warn!("Profil inconnu: {}", profile_name);
        }
    }
    
    Ok(())
}

fn apply_gaming_profile() -> Result<()> {
    tracing::info!("Application profil Gaming...");
    
    // 1. Timer Resolution 0.5ms
    let actual = timer::set_timer_resolution(5000)?;
    tracing::info!("Timer resolution: {}00ns", actual / 100);
    
    // 2. Power plan Ultimate Performance
    power::apply_gaming_power_config()?;
    
    // 3. Win32PrioritySeparation
    registry::set_priority_separation(0x26)?;
    
    // 4. Services télémétrie
    let telemetry_services = ["DiagTrack", "dmwappushservice", "WerSvc"];
    for svc in telemetry_services {
        if let Err(e) = services::disable_service(svc) {
            tracing::warn!("Service {}: {}", svc, e);
        }
    }
    
    // 5. Services performance
    let perf_services = ["SysMain", "WSearch"];
    for svc in perf_services {
        if let Err(e) = services::disable_service(svc) {
            tracing::warn!("Service {}: {}", svc, e);
        }
    }
    
    tracing::info!("Profil Gaming applique");
    Ok(())
}

fn apply_privacy_profile() -> Result<()> {
    tracing::info!("Application profil Privacy...");
    
    // 1. Services télémétrie complets
    let telemetry_services = [
        "DiagTrack", "dmwappushservice", "WerSvc", "wercplsupport",
        "PcaSvc", "WdiSystemHost", "WdiServiceHost", "lfsvc", "MapsBroker"
    ];
    for svc in telemetry_services {
        if let Err(e) = services::disable_service(svc) {
            tracing::warn!("Service {}: {}", svc, e);
        }
    }
    
    // 2. Firewall rules
    match firewall::create_telemetry_block_rules() {
        Ok(rules) => tracing::info!("Règles firewall: {:?}", rules),
        Err(e) => tracing::warn!("Firewall: {}", e),
    }
    
    tracing::info!("Profil Privacy applique");
    Ok(())
}

fn apply_workstation_profile() -> Result<()> {
    tracing::info!("Application profil Workstation...");
    
    // 1. Timer Resolution 1ms (moins agressif)
    let _ = timer::set_timer_resolution(10000);
    
    // 2. Power plan High Performance (pas Ultimate)
    power::set_power_plan(power::PowerPlan::HighPerformance)?;
    
    // 3. Services télémétrie seulement
    let telemetry_services = ["DiagTrack", "dmwappushservice"];
    for svc in telemetry_services {
        if let Err(e) = services::disable_service(svc) {
            tracing::warn!("Service {}: {}", svc, e);
        }
    }
    
    tracing::info!("Profil Workstation applique");
    Ok(())
}

/// Réinitialise toutes les optimisations
pub fn reset_to_defaults() -> Result<()> {
    tracing::info!("Réinitialisation aux valeurs par défaut...");
    
    // Power plan Balanced
    power::set_power_plan(power::PowerPlan::Balanced)?;
    
    // Services en mode automatique
    let services_to_restore = ["SysMain", "WSearch"];
    for svc in services_to_restore {
        let _ = services::set_service_automatic(svc);
    }
    
    Ok(())
}
