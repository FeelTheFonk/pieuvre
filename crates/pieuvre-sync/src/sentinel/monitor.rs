//! Sentinel Engine (SOTA 2026)
//!
//! Surveillance active des clés de registre et services critiques.
//! Restauration instantanée en cas de détection de dérive (drift).

use pieuvre_common::Result;
use windows::Win32::System::Registry::{
    RegOpenKeyExW, RegNotifyChangeKeyValue, RegCloseKey,
    HKEY_LOCAL_MACHINE, KEY_NOTIFY, REG_NOTIFY_CHANGE_LAST_SET,
};
use windows::Win32::Foundation::WAIT_OBJECT_0;
use windows::Win32::System::Threading::{CreateEventW, WaitForSingleObject, INFINITE};
use windows::core::PCWSTR;
use crate::hardening::CRITICAL_KEYS;

pub struct Sentinel;

impl Sentinel {
    /// Lance la surveillance en arrière-plan (Event-Driven SOTA)
    pub fn start_monitoring() -> Result<()> {
        tracing::info!("Sentinel Engine démarré - Mode Event-Driven (SOTA 2026)");
        
        // On effectue une restauration initiale pour partir d'une base saine
        let _ = Self::check_and_restore();

        // Surveillance des clés de registre via notifications natives
        for key_path in CRITICAL_KEYS {
            let key_path = key_path.to_string();
            std::thread::spawn(move || {
                if let Err(e) = Self::monitor_registry_key(&key_path) {
                    tracing::error!("Sentinel Registry Monitor error for {}: {:?}", key_path, e);
                }
            });
        }

        // TODO: Ajouter la surveillance des services via NotifyServiceStatusChange (Phase 1.2)
        
        Ok(())
    }

    fn monitor_registry_key(key_path: &str) -> Result<()> {
        unsafe {
            let subkey_wide: Vec<u16> = key_path.encode_utf16().chain(std::iter::once(0)).collect();
            let mut hkey = Default::default();
            
            if RegOpenKeyExW(
                HKEY_LOCAL_MACHINE,
                PCWSTR(subkey_wide.as_ptr()),
                Some(0),
                KEY_NOTIFY,
                &mut hkey,
            ).is_err() {
                return Err(pieuvre_common::PieuvreError::Registry(format!("Sentinel cannot open key for notify: {}", key_path)));
            }

            let event = CreateEventW(None, false, false, None)
                .map_err(|e| pieuvre_common::PieuvreError::Internal(format!("Failed to create sentinel event: {}", e)))?;

            loop {
                // S'enregistrer pour la notification
                let res = RegNotifyChangeKeyValue(
                    hkey,
                    true, // Watch subtree
                    REG_NOTIFY_CHANGE_LAST_SET,
                    Some(event),
                    true, // Async
                );

                if res.is_err() {
                    let _ = RegCloseKey(hkey);
                    let _ = windows::Win32::Foundation::CloseHandle(event);
                    return Err(pieuvre_common::PieuvreError::Registry(format!("RegNotifyChangeKeyValue failed for {}", key_path)));
                }

                // Attendre l'événement (bloquant pour ce thread dédié)
                let wait_res = WaitForSingleObject(event, INFINITE);
                
                if wait_res == WAIT_OBJECT_0 {
                    tracing::warn!("Sentinel: Modification détectée sur {}, restauration immédiate...", key_path);
                    // Restauration instantanée (Self-Healing)
                    if let Err(e) = crate::hardening::lock_registry_key(key_path) {
                        tracing::error!("Sentinel failed to restore {}: {:?}", key_path, e);
                    }
                }
            }
        }
    }

    fn check_and_restore() -> Result<()> {
        for key in CRITICAL_KEYS {
            crate::hardening::lock_registry_key(key)?;
        }

        for service in crate::hardening::CRITICAL_SERVICES {
            crate::hardening::lock_service(service)?;
        }
        
        Ok(())
    }
}
