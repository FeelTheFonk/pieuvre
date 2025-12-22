//! Driver Resolver SOTA 2026
//!
//! Mappe les adresses de routine noyau aux noms de drivers (.sys).

use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};
use windows::Win32::System::ProcessStatus::{EnumDeviceDrivers, GetDeviceDriverBaseNameW};

/// Information sur un driver chargé
#[derive(Debug, Clone)]
pub struct DriverModule {
    pub name: String,
    pub base_address: usize,
}

/// Cache global pour la résolution des drivers
pub struct DriverResolver {
    modules: BTreeMap<usize, String>,
}

static RESOLVER: Lazy<Arc<RwLock<DriverResolver>>> =
    Lazy::new(|| Arc::new(RwLock::new(DriverResolver::new())));

impl DriverResolver {
    pub fn global() -> Arc<RwLock<Self>> {
        RESOLVER.clone()
    }

    fn new() -> Self {
        let mut resolver = Self {
            modules: BTreeMap::new(),
        };
        let _ = resolver.refresh();
        resolver
    }

    /// Rafraîchit la liste des drivers chargés
    pub fn refresh(&mut self) -> anyhow::Result<()> {
        unsafe {
            let mut bytes_needed = 0u32;
            // EnumDeviceDrivers(lpImageBase: *mut *mut c_void, cb: u32, lpcbNeeded: *mut u32)
            if EnumDeviceDrivers(std::ptr::null_mut(), 0, &mut bytes_needed).is_err() {
                return Err(anyhow::anyhow!("Failed to enum device drivers (size)"));
            }

            let count = bytes_needed as usize / std::mem::size_of::<*mut std::ffi::c_void>();
            let mut drivers = vec![std::ptr::null_mut::<std::ffi::c_void>(); count];

            if EnumDeviceDrivers(drivers.as_mut_ptr(), bytes_needed, &mut bytes_needed).is_err() {
                return Err(anyhow::anyhow!("Failed to enum device drivers"));
            }

            self.modules.clear();
            for &base in &drivers {
                if base.is_null() {
                    continue;
                }

                let mut name_buffer = vec![0u16; 256];
                // GetDeviceDriverBaseNameW(ImageBase: *const c_void, lpBaseName: PWSTR, nSize: u32)
                let len = GetDeviceDriverBaseNameW(base, &mut name_buffer);

                if len > 0 {
                    let name = String::from_utf16_lossy(&name_buffer[..len as usize]);
                    self.modules.insert(base as usize, name);
                }
            }

            tracing::debug!("DriverResolver: {} modules chargés", self.modules.len());
            Ok(())
        }
    }

    /// Résout une adresse de routine en nom de driver
    pub fn resolve(&self, address: usize) -> String {
        // Trouve le driver dont la base est immédiatement inférieure ou égale à l'adresse
        if let Some((&base, name)) = self.modules.range(..=address).next_back() {
            // Heuristique : un driver fait rarement plus de 50MB
            if address - base < 50 * 1024 * 1024 {
                return name.clone();
            }
        }
        format!("0x{:x}", address)
    }
}
