//! Inspection des services Windows
//!
//! Énumération et catégorisation des services.

use pieuvre_common::{Result, ServiceCategory, ServiceInfo, ServiceStartType, ServiceStatus};
use windows::Win32::System::Services::{
    EnumServicesStatusExW, OpenSCManagerW,
    SC_ENUM_PROCESS_INFO, SC_MANAGER_ENUMERATE_SERVICE, SERVICE_STATE_ALL,
    SERVICE_WIN32, ENUM_SERVICE_STATUS_PROCESSW,
};
use windows::core::PCWSTR;

/// Services connus comme télémétrie
const TELEMETRY_SERVICES: &[&str] = &[
    "DiagTrack",
    "dmwappushservice",
    "WerSvc",
    "wercplsupport",
    "PcaSvc",
    "WdiSystemHost",
    "WdiServiceHost",
];

/// Services liés à la performance
const PERFORMANCE_SERVICES: &[&str] = &[
    "SysMain",
    "WSearch",
    "Spooler",
    "Fax",
    "TabletInputService",
];

/// Inspecte tous les services du système
pub fn inspect_services() -> Result<Vec<ServiceInfo>> {
    let mut services = Vec::new();
    
    unsafe {
        let scm = match OpenSCManagerW(
            PCWSTR::null(),
            PCWSTR::null(),
            SC_MANAGER_ENUMERATE_SERVICE,
        ) {
            Ok(handle) => handle,
            Err(_) => return Ok(Vec::new()),
        };
        
        let mut bytes_needed = 0u32;
        let mut services_returned = 0u32;
        let mut resume_handle = 0u32;
        
        // Premier appel pour obtenir la taille
        let _ = EnumServicesStatusExW(
            scm,
            SC_ENUM_PROCESS_INFO,
            SERVICE_WIN32,
            SERVICE_STATE_ALL,
            None,
            &mut bytes_needed,
            &mut services_returned,
            Some(&mut resume_handle),
            None,
        );
        
        if bytes_needed > 0 {
            let mut buffer = vec![0u8; bytes_needed as usize];
            
            let result = EnumServicesStatusExW(
                scm,
                SC_ENUM_PROCESS_INFO,
                SERVICE_WIN32,
                SERVICE_STATE_ALL,
                Some(&mut buffer),
                &mut bytes_needed,
                &mut services_returned,
                Some(&mut resume_handle),
                None,
            );
            
            if result.is_ok() {
                let entries = std::slice::from_raw_parts(
                    buffer.as_ptr() as *const ENUM_SERVICE_STATUS_PROCESSW,
                    services_returned as usize,
                );
                
                for entry in entries {
                    let name = pwstr_to_string(entry.lpServiceName);
                    let display_name = pwstr_to_string(entry.lpDisplayName);
                    
                    // Utiliser .0 pour accéder à la valeur u32 interne
                    let status = match entry.ServiceStatusProcess.dwCurrentState.0 {
                        1 => ServiceStatus::Stopped,
                        4 => ServiceStatus::Running,
                        7 => ServiceStatus::Paused,
                        _ => ServiceStatus::Unknown,
                    };
                    
                    let category = categorize_service(&name);
                    
                    services.push(ServiceInfo {
                        name,
                        display_name,
                        status,
                        start_type: ServiceStartType::Manual,
                        category,
                    });
                }
            }
        }
    }
    
    Ok(services)
}

fn categorize_service(name: &str) -> ServiceCategory {
    if TELEMETRY_SERVICES.iter().any(|s| s.eq_ignore_ascii_case(name)) {
        ServiceCategory::Telemetry
    } else if PERFORMANCE_SERVICES.iter().any(|s| s.eq_ignore_ascii_case(name)) {
        ServiceCategory::Performance
    } else if name.starts_with("Wdf") || name.starts_with("Wmi") {
        ServiceCategory::System
    } else {
        ServiceCategory::Unknown
    }
}

fn pwstr_to_string(ptr: windows::core::PWSTR) -> String {
    if ptr.is_null() {
        return String::new();
    }
    unsafe {
        let len = (0..).take_while(|&i| *ptr.0.add(i) != 0).count();
        String::from_utf16_lossy(std::slice::from_raw_parts(ptr.0, len))
    }
}
