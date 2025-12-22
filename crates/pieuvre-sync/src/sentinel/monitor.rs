//! Sentinel Engine
//!
//! Active monitoring of critical registry keys and services.
//! Instant restoration upon drift detection.

use crate::hardening::CRITICAL_KEYS;
use pieuvre_common::Result;
use windows::core::PCWSTR;
use windows::Win32::Foundation::WAIT_OBJECT_0;
use windows::Win32::System::Registry::{
    RegCloseKey, RegNotifyChangeKeyValue, RegOpenKeyExW, HKEY_LOCAL_MACHINE, KEY_NOTIFY,
    REG_NOTIFY_CHANGE_LAST_SET,
};
use windows::Win32::System::Threading::{CreateEventW, WaitForSingleObject, INFINITE};

pub struct Sentinel;

impl Sentinel {
    /// Starts background monitoring (Event-Driven)
    pub fn start_monitoring() -> Result<()> {
        tracing::info!("Sentinel Engine started - Event-Driven mode");

        // Perform initial restoration to start from a clean state
        let _ = Self::check_and_restore();

        // Monitor registry keys via native notifications
        for key_path in CRITICAL_KEYS {
            let key_path = key_path.to_string();
            std::thread::spawn(move || {
                if let Err(e) = Self::monitor_registry_key(&key_path) {
                    tracing::error!("Sentinel Registry Monitor error for {}: {:?}", key_path, e);
                }
            });
        }

        // Monitor services via dedicated threads
        for service_name in crate::hardening::CRITICAL_SERVICES {
            let service_name = service_name.to_string();
            std::thread::spawn(move || {
                if let Err(e) = Self::monitor_service(&service_name) {
                    tracing::error!(
                        "Sentinel Service Monitor error for {}: {:?}",
                        service_name,
                        e
                    );
                }
            });
        }

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
            )
            .is_err()
            {
                return Err(pieuvre_common::PieuvreError::Registry(format!(
                    "Sentinel cannot open key for notify: {}",
                    key_path
                )));
            }

            let event = CreateEventW(None, false, false, None).map_err(|e| {
                pieuvre_common::PieuvreError::Internal(format!(
                    "Failed to create sentinel event: {}",
                    e
                ))
            })?;

            loop {
                // Register for notification
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
                    return Err(pieuvre_common::PieuvreError::Registry(format!(
                        "RegNotifyChangeKeyValue failed for {}",
                        key_path
                    )));
                }

                // Wait for event (blocking for this dedicated thread)
                let wait_res = WaitForSingleObject(event, INFINITE);

                if wait_res == WAIT_OBJECT_0 {
                    tracing::warn!(
                        "Sentinel: Modification detected on {}, immediate restoration...",
                        key_path
                    );
                    // Instant restoration (Self-Healing)
                    if let Err(e) = crate::hardening::lock_registry_key(key_path) {
                        tracing::error!("Sentinel failed to restore {}: {:?}", key_path, e);
                    }
                }
            }
        }
    }

    fn monitor_service(service_name: &str) -> Result<()> {
        // Note: NotifyServiceStatusChange is complex to use in a simple thread loop
        // We use a smart polling approach with exponential backoff or event-driven if possible
        // For v0.4.0, we implement a robust polling with 5s interval for services
        loop {
            if let Err(e) = crate::hardening::lock_service(service_name) {
                tracing::error!(
                    "Sentinel failed to restore service {}: {:?}",
                    service_name,
                    e
                );
            }
            std::thread::sleep(std::time::Duration::from_secs(5));
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
