//! Gestion des services Windows

use pieuvre_common::{PieuvreError, Result};
use windows::Win32::System::Services::{
    OpenSCManagerW, OpenServiceW, ChangeServiceConfigW,
    SC_MANAGER_ALL_ACCESS, SERVICE_CHANGE_CONFIG,
    SERVICE_START_TYPE, ENUM_SERVICE_TYPE, SERVICE_ERROR,
};
use windows::core::PCWSTR;

/// Constantes pour ChangeServiceConfigW
const SERVICE_NO_CHANGE_TYPE: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(0xFFFFFFFF);
const SERVICE_NO_CHANGE_ERROR: SERVICE_ERROR = SERVICE_ERROR(0xFFFFFFFF);

/// Désactive un service
pub fn disable_service(name: &str) -> Result<()> {
    set_service_start_type(name, SERVICE_START_TYPE(4)) // 4 = Disabled
}

/// Met un service en démarrage manuel
pub fn set_service_manual(name: &str) -> Result<()> {
    set_service_start_type(name, SERVICE_START_TYPE(3)) // 3 = Manual
}

/// Met un service en démarrage automatique
pub fn set_service_automatic(name: &str) -> Result<()> {
    set_service_start_type(name, SERVICE_START_TYPE(2)) // 2 = Automatic
}

fn set_service_start_type(name: &str, start_type: SERVICE_START_TYPE) -> Result<()> {
    unsafe {
        let scm = OpenSCManagerW(
            PCWSTR::null(),
            PCWSTR::null(),
            SC_MANAGER_ALL_ACCESS,
        ).map_err(|e| PieuvreError::Permission(e.to_string()))?;
        
        let name_wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
        
        let service = OpenServiceW(
            scm,
            PCWSTR(name_wide.as_ptr()),
            SERVICE_CHANGE_CONFIG,
        ).map_err(|_| PieuvreError::ServiceNotFound(name.to_string()))?;
        
        ChangeServiceConfigW(
            service,
            SERVICE_NO_CHANGE_TYPE,
            start_type,
            SERVICE_NO_CHANGE_ERROR,
            PCWSTR::null(),
            PCWSTR::null(),
            None,
            PCWSTR::null(),
            PCWSTR::null(),
            PCWSTR::null(),
            PCWSTR::null(),
        ).map_err(|e| PieuvreError::Registry(e.to_string()))?;
        
        tracing::info!("Service {} start_type -> {:?}", name, start_type);
        Ok(())
    }
}
