//! Rollback Manager
//!
//! Logique de retour arrière automatique basée sur les ChangeRecords.

use pieuvre_common::{ChangeRecord, Result};
use tracing::{info, instrument};

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
                hive,
                original_value,
            } => {
                rollback_registry_full(hive, &key, &value_name, original_value).await?;
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
    hive: pieuvre_common::RegistryHive,
    key: &str,
    value: &str,
    original_value: Option<pieuvre_common::RegistryValue>,
) -> Result<()> {
    info!(key, value, "Restauration registre...");

    // Déverrouillage
    let _ = tokio::task::spawn_blocking({
        let key = key.to_string();
        move || crate::hardening::unlock_registry_key(&key)
    })
    .await;

    let key_clone = key.to_string();
    let value_clone = value.to_string();

    tokio::task::spawn_blocking(move || {
        let hive_handle = match hive {
            pieuvre_common::RegistryHive::Hklm => {
                windows::Win32::System::Registry::HKEY_LOCAL_MACHINE
            }
            pieuvre_common::RegistryHive::Hku => windows::Win32::System::Registry::HKEY_USERS,
            pieuvre_common::RegistryHive::Hkcu => {
                windows::Win32::System::Registry::HKEY_CURRENT_USER
            }
        };

        if let Some(val) = original_value {
            match val {
                pieuvre_common::RegistryValue::Dword(d) => {
                    crate::registry::set_dword_value_in_hive(
                        hive_handle,
                        &key_clone,
                        &value_clone,
                        d,
                    )
                }
                pieuvre_common::RegistryValue::String(s) => {
                    crate::registry::set_string_value_in_hive(
                        hive_handle,
                        &key_clone,
                        &value_clone,
                        &s,
                    )
                }
                _ => Ok(()), // Binary non supporté pour l'instant
            }
        } else {
            // Si pas de valeur originale, on supprime la valeur (état initial = inexistant)
            crate::registry::delete_value(&key_clone, &value_clone)
        }
    })
    .await
    .map_err(|e| pieuvre_common::PieuvreError::Internal(e.to_string()))??;

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
