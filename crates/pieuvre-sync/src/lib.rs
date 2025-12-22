//! Pieuvre Sync Engine
//!
//! Module de synchronisation: application des optimisations.

pub mod appx;
pub mod context_menu;
pub mod cpu;
pub mod dpc;
pub mod edge;
pub mod explorer;
pub mod firewall;
pub mod game_mode;
pub mod hosts;
pub mod msi;
pub mod network;
pub mod operation;
pub mod onedrive;
pub mod power;
pub mod registry;
pub mod rollback;
pub mod scheduled_tasks;
pub mod security;
pub mod services;
pub mod timer;
pub mod widgets;
pub mod windows_update;

#[cfg(test)]
mod tests;


use pieuvre_common::Result;
use tracing::instrument;
use std::path::Path;

/// Applique un profil d'optimisation
pub async fn apply_profile(profile_name: &str, dry_run: bool) -> Result<()> {
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
        "gaming" => apply_gaming_profile().await?,
        "privacy" => apply_privacy_profile().await?,
        "workstation" => apply_workstation_profile().await?,
        _ => {
            tracing::warn!("Profil inconnu: {}", profile_name);
        }
    }
    
    Ok(())
}

async fn apply_gaming_profile() -> Result<()> {
    tracing::info!("Application profil Gaming (Polymorphe)...");
    
    use crate::operation::{SyncOperation, ServiceOperation, RegistryDwordOperation};
    use tokio::task::JoinSet;

    let operations: Vec<Box<dyn SyncOperation>> = vec![
        // 1. Timer & Priority (Registre)
        Box::new(RegistryDwordOperation {
            key: r"SYSTEM\CurrentControlSet\Control\PriorityControl".to_string(),
            value: "Win32PrioritySeparation".to_string(),
            target_data: 0x26,
        }),
        // 2. Services Télémétrie
        Box::new(ServiceOperation { name: "DiagTrack".to_string(), target_start_type: 4 }),
        Box::new(ServiceOperation { name: "dmwappushservice".to_string(), target_start_type: 4 }),
        Box::new(ServiceOperation { name: "WerSvc".to_string(), target_start_type: 4 }),
        // 3. Services Performance
        Box::new(ServiceOperation { name: "SysMain".to_string(), target_start_type: 4 }),
        Box::new(ServiceOperation { name: "WSearch".to_string(), target_start_type: 4 }),
    ];

    let mut set = JoinSet::new();
    for op in operations {
        set.spawn(async move {
            op.apply().await
        });
    }

    let mut all_changes = Vec::new();
    while let Some(res) = set.join_next().await {
        if let Ok(Ok(changes)) = res {
            all_changes.extend(changes);
        }
    }

    // 4. Power plan (Encore spécifique car complexe)
    let _ = tokio::task::spawn_blocking(|| power::apply_gaming_power_config()).await;
    
    tracing::info!("Profil Gaming applique ({} changements)", all_changes.len());
    Ok(())
}

#[instrument]
async fn apply_privacy_profile() -> Result<()> {
    tracing::info!("Application profil Privacy (Polymorphe)...");
    
    use crate::operation::{SyncOperation, ServiceOperation, RegistryDwordOperation};
    use tokio::task::JoinSet;

    let operations: Vec<Box<dyn SyncOperation>> = vec![
        // 1. Services Télémétrie (Complet)
        Box::new(ServiceOperation { name: "DiagTrack".to_string(), target_start_type: 4 }),
        Box::new(ServiceOperation { name: "dmwappushservice".to_string(), target_start_type: 4 }),
        Box::new(ServiceOperation { name: "WerSvc".to_string(), target_start_type: 4 }),
        Box::new(ServiceOperation { name: "OneSyncSvc".to_string(), target_start_type: 4 }),
        Box::new(ServiceOperation { name: "MessagingService".to_string(), target_start_type: 4 }),
        Box::new(ServiceOperation { name: "PhoneSvc".to_string(), target_start_type: 4 }),
        // 2. Registre Privacy
        Box::new(RegistryDwordOperation {
            key: r"SOFTWARE\Policies\Microsoft\Windows\DataCollection".to_string(),
            value: "AllowTelemetry".to_string(),
            target_data: 0,
        }),
        Box::new(RegistryDwordOperation {
            key: r"SOFTWARE\Microsoft\Windows\CurrentVersion\AdvertisingInfo".to_string(),
            value: "Enabled".to_string(),
            target_data: 0,
        }),
    ];

    let mut set = JoinSet::new();
    for op in operations {
        set.spawn(async move { op.apply().await });
    }

    let mut all_changes = Vec::new();
    while let Some(res) = set.join_next().await {
        if let Ok(Ok(changes)) = res {
            all_changes.extend(changes);
        }
    }

    // 3. Hosts & Firewall (Encore spécifique)
    let _ = tokio::task::spawn_blocking(|| hosts::add_telemetry_blocks()).await;
    let _ = tokio::task::spawn_blocking(|| firewall::create_telemetry_block_rules()).await;
    
    tracing::info!("Profil Privacy applique ({} changements)", all_changes.len());
    Ok(())
}

#[instrument]
async fn apply_workstation_profile() -> Result<()> {
    tracing::info!("Application profil Workstation (Polymorphe)...");
    
    use crate::operation::{SyncOperation, ServiceOperation, RegistryDwordOperation};
    use tokio::task::JoinSet;

    let operations: Vec<Box<dyn SyncOperation>> = vec![
        // 1. Timer (1ms pour workstation)
        Box::new(RegistryDwordOperation {
            key: r"SYSTEM\CurrentControlSet\Control\PriorityControl".to_string(),
            value: "Win32PrioritySeparation".to_string(),
            target_data: 0x18,
        }),
        // 2. Services Télémétrie (Minimal)
        Box::new(ServiceOperation { name: "DiagTrack".to_string(), target_start_type: 4 }),
        Box::new(ServiceOperation { name: "dmwappushservice".to_string(), target_start_type: 4 }),
    ];

    let mut set = JoinSet::new();
    for op in operations {
        set.spawn(async move { op.apply().await });
    }

    let mut all_changes = Vec::new();
    while let Some(res) = set.join_next().await {
        if let Ok(Ok(changes)) = res {
            all_changes.extend(changes);
        }
    }

    // 3. Power plan High Performance
    let _ = tokio::task::spawn_blocking(|| power::set_power_plan(power::PowerPlan::HighPerformance)).await;
    
    tracing::info!("Profil Workstation applique ({} changements)", all_changes.len());
    Ok(())
}

#[instrument]
pub async fn reset_to_defaults() -> Result<()> {
    tracing::info!("Reinitialisation aux valeurs par defaut (Polymorphe)...");
    
    use crate::operation::{SyncOperation, ServiceOperation, RegistryDwordOperation};
    use tokio::task::JoinSet;

    let operations: Vec<Box<dyn SyncOperation>> = vec![
        // 1. Services en mode automatique (ou manuel selon le service)
        Box::new(ServiceOperation { name: "DiagTrack".to_string(), target_start_type: 2 }),
        Box::new(ServiceOperation { name: "dmwappushservice".to_string(), target_start_type: 3 }),
        Box::new(ServiceOperation { name: "WerSvc".to_string(), target_start_type: 3 }),
        Box::new(ServiceOperation { name: "SysMain".to_string(), target_start_type: 2 }),
        Box::new(ServiceOperation { name: "WSearch".to_string(), target_start_type: 2 }),
        // 2. Registre par défaut
        Box::new(RegistryDwordOperation {
            key: r"SYSTEM\CurrentControlSet\Control\PriorityControl".to_string(),
            value: "Win32PrioritySeparation".to_string(),
            target_data: 0x2,
        }),
    ];

    let mut set = JoinSet::new();
    for op in operations {
        set.spawn(async move { op.apply().await });
    }

    let mut all_changes = Vec::new();
    while let Some(res) = set.join_next().await {
        if let Ok(Ok(changes)) = res {
            all_changes.extend(changes);
        }
    }

    // 3. Power plan Balanced
    let _ = tokio::task::spawn_blocking(|| power::set_power_plan(power::PowerPlan::Balanced)).await;
    
    tracing::info!("Reinitialisation terminee ({} changements)", all_changes.len());
    Ok(())
}
