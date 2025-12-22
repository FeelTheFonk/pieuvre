//! Rollback Manager SOTA 2026
//!
//! Logique de retour arrière automatique basée sur les ChangeRecords.

use crate::operation::{SyncOperation, ServiceOperation, RegistryDwordOperation};
use pieuvre_common::{Result, ChangeRecord};
use tracing::{instrument, info, warn};

/// Gère le rollback d'une liste de changements
#[instrument(skip(changes))]
pub async fn rollback_changes(changes: Vec<ChangeRecord>) -> Result<()> {
    info!("Lancement du rollback automatique ({} changements)...", changes.len());
    
    // Inverser l'ordre des changements pour le rollback (LIFO)
    let mut changes = changes;
    changes.reverse();
    
    for record in changes {
        match record {
            ChangeRecord::Registry { key, value_name, value_type, original_data } => {
                rollback_registry(&key, &value_name, &value_type, original_data).await?;
            }
            ChangeRecord::Service { name, original_start_type } => {
                rollback_service(&name, original_start_type).await?;
            }
            ChangeRecord::FirewallRule { name } => {
                rollback_firewall(&name).await?;
            }
        }
    }
    
    info!("Rollback termine avec succes.");
    Ok(())
}

async fn rollback_registry(key: &str, value: &str, val_type: &str, data: Vec<u8>) -> Result<()> {
    info!(key, value, "Restauration registre...");
    
    if val_type == "REG_DWORD" && data.len() == 4 {
        let dword = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        tokio::task::spawn_blocking({
            let key = key.to_string();
            let value = value.to_string();
            move || crate::registry::set_dword_value(&key, &value, dword)
        }).await.map_err(|e| pieuvre_common::PieuvreError::Internal(e.to_string()))??;
    } else {
        warn!("Type de registre non supporte pour le rollback: {}", val_type);
    }
    
    Ok(())
}

async fn rollback_service(name: &str, start_type: u32) -> Result<()> {
    info!(name, start_type, "Restauration service...");
    
    tokio::task::spawn_blocking({
        let name = name.to_string();
        move || {
            // TODO: implémenter set_service_start_type générique
            match start_type {
                2 => crate::services::set_service_automatic(&name),
                3 => crate::services::set_service_manual(&name),
                4 => crate::services::disable_service(&name),
                _ => Ok(()),
            }
        }
    }).await.map_err(|e| pieuvre_common::PieuvreError::Internal(e.to_string()))??;
    
    Ok(())
}

async fn rollback_firewall(name: &str) -> Result<()> {
    info!(name, "Suppression regle firewall...");
    
    tokio::task::spawn_blocking({
        let name = name.to_string();
        move || {
            use std::process::Command;
            let _ = Command::new("netsh")
                .args(["advfirewall", "firewall", "delete", "rule", &format!("name={}", name)])
                .output();
            Ok::<(), pieuvre_common::PieuvreError>(())
        }
    }).await.map_err(|e| pieuvre_common::PieuvreError::Internal(e.to_string()))??;
    
    Ok(())
}
