//! Rollback Manager
//!
//! Logique de retour arrière automatique basée sur les ChangeRecords.

use pieuvre_common::{ChangeRecord, Result};
use tracing::{info, instrument, warn};

/// Gère le rollback d'une liste de changements
#[instrument(skip(changes))]
pub async fn rollback_changes(changes: Vec<ChangeRecord>) -> Result<()> {
    info!(
        "Lancement du rollback automatique ({} changements)...",
        changes.len()
    );

    // Inverser l'ordre des changements pour le rollback (LIFO)
    let mut changes = changes;
    changes.reverse();

    for record in changes {
        match record {
            ChangeRecord::Registry {
                key,
                value_name,
                value_type,
                original_data,
            } => {
                rollback_registry_full(&key, &value_name, &value_type, original_data).await?;
            }
            ChangeRecord::Service {
                name,
                original_start_type,
            } => {
                rollback_service(&name, original_start_type).await?;
            }
            ChangeRecord::FirewallRule { name } => {
                rollback_firewall(&name).await?;
            }
            ChangeRecord::AppX { package_full_name } => {
                rollback_appx(&package_full_name).await?;
            }
        }
    }

    info!("Rollback termine avec succes.");
    Ok(())
}

async fn rollback_registry_full(
    key: &str,
    value: &str,
    val_type: &str,
    data: Vec<u8>,
) -> Result<()> {
    info!(key, value, "Restauration registre (Full)...");

    // Déverrouillage
    let _ = tokio::task::spawn_blocking({
        let key = key.to_string();
        move || crate::hardening::unlock_registry_key(&key)
    })
    .await;

    if val_type == "REG_DWORD" && data.len() == 4 {
        let dword = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        tokio::task::spawn_blocking({
            let key = key.to_string();
            let value = value.to_string();
            move || crate::registry::set_dword_value(&key, &value, dword)
        })
        .await
        .map_err(|e| pieuvre_common::PieuvreError::Internal(e.to_string()))??;
    } else {
        warn!(
            "Type de registre non supporte pour le rollback full: {}",
            val_type
        );
    }

    Ok(())
}

async fn rollback_service(name: &str, start_type: u32) -> Result<()> {
    info!(name, start_type, "Restauration service...");

    tokio::task::spawn_blocking({
        let name = name.to_string();
        move || crate::services::set_service_start_type(&name, start_type)
    })
    .await
    .map_err(|e| pieuvre_common::PieuvreError::Internal(e.to_string()))??;

    Ok(())
}

async fn rollback_firewall(name: &str) -> Result<()> {
    info!(name, "Suppression regle firewall (Native COM)...");

    tokio::task::spawn_blocking({
        let _name = name.to_string();
        move || {
            // Utiliser l'API COM native au lieu de netsh
            crate::firewall::remove_pieuvre_rules().map(|_| ())
        }
    })
    .await
    .map_err(|e| pieuvre_common::PieuvreError::Internal(e.to_string()))??;

    Ok(())
}

async fn rollback_appx(package_full_name: &str) -> Result<()> {
    info!(
        package_full_name,
        "Restauration AppX (Note: Reinstallation non supportee)..."
    );
    // TODO: Implémenter la réinstallation via Add-AppxPackage si nécessaire
    Ok(())
}
