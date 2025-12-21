//! Gestion des services Windows

use pieuvre_common::{PieuvreError, Result};
use windows::Win32::System::Services::{
    OpenSCManagerW, OpenServiceW, ChangeServiceConfigW, QueryServiceConfigW, CloseServiceHandle,
    SC_MANAGER_ALL_ACCESS, SERVICE_CHANGE_CONFIG, SERVICE_QUERY_CONFIG,
    SERVICE_START_TYPE, ENUM_SERVICE_TYPE, SERVICE_ERROR, QUERY_SERVICE_CONFIGW,
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

/// Récupère le start type actuel d'un service (pour snapshot)
pub fn get_service_start_type(name: &str) -> Result<u32> {
    unsafe {
        let scm = OpenSCManagerW(
            PCWSTR::null(),
            PCWSTR::null(),
            SC_MANAGER_ALL_ACCESS,
        ).map_err(|e| PieuvreError::Permission(e.to_string()))?;
        
        let name_wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
        
        let service = match OpenServiceW(
            scm,
            PCWSTR(name_wide.as_ptr()),
            SERVICE_QUERY_CONFIG,
        ) {
            Ok(s) => s,
            Err(_) => {
                let _ = CloseServiceHandle(scm);
                return Err(PieuvreError::ServiceNotFound(name.to_string()));
            }
        };
        
        // Premier appel pour obtenir la taille
        let mut bytes_needed = 0u32;
        let _ = QueryServiceConfigW(service, None, 0, &mut bytes_needed);
        
        if bytes_needed == 0 {
            let _ = CloseServiceHandle(service);
            let _ = CloseServiceHandle(scm);
            return Err(PieuvreError::ServiceNotFound(name.to_string()));
        }
        
        let mut buffer = vec![0u8; bytes_needed as usize];
        let config_ptr = buffer.as_mut_ptr() as *mut QUERY_SERVICE_CONFIGW;
        
        let result = QueryServiceConfigW(
            service,
            Some(config_ptr),
            bytes_needed,
            &mut bytes_needed,
        );
        
        let start_type = if result.is_ok() {
            (*config_ptr).dwStartType.0
        } else {
            3 // Default to Manual if error
        };
        
        let _ = CloseServiceHandle(service);
        let _ = CloseServiceHandle(scm);
        
        Ok(start_type)
    }
}

fn set_service_start_type(name: &str, start_type: SERVICE_START_TYPE) -> Result<()> {
    unsafe {
        let scm = OpenSCManagerW(
            PCWSTR::null(),
            PCWSTR::null(),
            SC_MANAGER_ALL_ACCESS,
        ).map_err(|e| PieuvreError::Permission(e.to_string()))?;
        
        let name_wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
        
        let service = match OpenServiceW(
            scm,
            PCWSTR(name_wide.as_ptr()),
            SERVICE_CHANGE_CONFIG,
        ) {
            Ok(s) => s,
            Err(_) => {
                let _ = CloseServiceHandle(scm);
                return Err(PieuvreError::ServiceNotFound(name.to_string()));
            }
        };
        
        let result = ChangeServiceConfigW(
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
        );
        
        let _ = CloseServiceHandle(service);
        let _ = CloseServiceHandle(scm);
        
        result.map_err(|e| PieuvreError::Registry(e.to_string()))?;
        
        tracing::info!("Service {} start_type -> {:?}", name, start_type);
        Ok(())
    }
}
