//! Windows services inspection
//!
//! Enumeration and categorization of services.

use pieuvre_common::{Result, ServiceCategory, ServiceInfo, ServiceStartType, ServiceStatus};
use windows::core::PCWSTR;
use windows::Win32::System::Services::{
    CloseServiceHandle, EnumServicesStatusExW, OpenSCManagerW, OpenServiceW, QueryServiceConfigW,
    ENUM_SERVICE_STATUS_PROCESSW, QUERY_SERVICE_CONFIGW, SC_ENUM_PROCESS_INFO,
    SC_MANAGER_ENUMERATE_SERVICE, SERVICE_QUERY_CONFIG, SERVICE_STATE_ALL, SERVICE_WIN32,
};

/// Services known as telemetry
const TELEMETRY_SERVICES: &[&str] = &[
    "DiagTrack",
    "dmwappushservice",
    "WerSvc",
    "wercplsupport",
    "PcaSvc",
    "WdiSystemHost",
    "WdiServiceHost",
    "diagnosticshub.standardcollector.service",
    "DusmSvc",
    "MapsBroker",
    "lfsvc",
    "wisvc",
];

/// Performance related services
const PERFORMANCE_SERVICES: &[&str] = &[
    "SysMain",
    "WSearch",
    "Spooler",
    "Fax",
    "TabletInputService",
    "BITS",
    "wuauserv",
    "UsoSvc",
    "DoSvc",
];

/// Security services
const SECURITY_SERVICES: &[&str] = &[
    "WinDefend",
    "SecurityHealthService",
    "Sense",
    "wscsvc",
    "MpsSvc",
    "BFE",
    "SharedAccess",
    "IKEEXT",
    "PolicyAgent",
    "SamSs",
    "VaultSvc",
    "KeyIso",
];

/// Network services
const NETWORK_SERVICES: &[&str] = &[
    "Dhcp",
    "Dnscache",
    "NlaSvc",
    "netprofm",
    "Netman",
    "LanmanServer",
    "LanmanWorkstation",
    "RpcSs",
    "RpcEptMapper",
    "lmhosts",
    "NetTcpPortSharing",
];

/// Critical system services
const SYSTEM_SERVICES: &[&str] = &[
    "Wdf",
    "Wmi",
    "Power",
    "PlugPlay",
    "EventLog",
    "EventSystem",
    "Schedule",
    "ProfSvc",
    "UserManager",
    "LSM",
    "Winmgmt",
    "CoreMessagingRegistrar",
    "SystemEventsBroker",
    "TimeBrokerSvc",
];

/// Inspects all system services with start_type detection
pub fn inspect_services() -> Result<Vec<ServiceInfo>> {
    let mut services = Vec::new();

    unsafe {
        let scm = match OpenSCManagerW(PCWSTR::null(), PCWSTR::null(), SC_MANAGER_ENUMERATE_SERVICE)
        {
            Ok(handle) => handle,
            Err(_) => return Ok(Vec::new()),
        };

        let mut bytes_needed = 0u32;
        let mut services_returned = 0u32;
        let mut resume_handle = 0u32;

        // First call to get size
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

                    // Service status
                    let status = match entry.ServiceStatusProcess.dwCurrentState.0 {
                        1 => ServiceStatus::Stopped,
                        2 => ServiceStatus::StartPending,
                        3 => ServiceStatus::StopPending,
                        4 => ServiceStatus::Running,
                        5 => ServiceStatus::ContinuePending,
                        6 => ServiceStatus::PausePending,
                        7 => ServiceStatus::Paused,
                        _ => ServiceStatus::Unknown,
                    };

                    // Get real start_type via QueryServiceConfigW
                    let start_type = get_service_start_type(scm, &name);

                    // PID if running
                    let pid = if status == ServiceStatus::Running {
                        Some(entry.ServiceStatusProcess.dwProcessId)
                    } else {
                        None
                    };

                    let category = categorize_service(&name);

                    services.push(ServiceInfo {
                        name,
                        display_name,
                        status,
                        start_type,
                        category,
                        pid,
                    });
                }
            }
        }

        let _ = CloseServiceHandle(scm);
    }

    Ok(services)
}

/// Retrieves the real start type of a service via QueryServiceConfigW
fn get_service_start_type(
    scm: windows::Win32::System::Services::SC_HANDLE,
    name: &str,
) -> ServiceStartType {
    unsafe {
        let name_wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();

        let service = match OpenServiceW(scm, PCWSTR(name_wide.as_ptr()), SERVICE_QUERY_CONFIG) {
            Ok(handle) => handle,
            Err(_) => return ServiceStartType::Unknown,
        };

        // First call to get necessary size
        let mut bytes_needed = 0u32;
        let _ = QueryServiceConfigW(service, None, 0, &mut bytes_needed);

        if bytes_needed == 0 {
            let _ = CloseServiceHandle(service);
            return ServiceStartType::Unknown;
        }

        let mut buffer = vec![0u8; bytes_needed as usize];
        let config_ptr = buffer.as_mut_ptr() as *mut QUERY_SERVICE_CONFIGW;

        let result =
            QueryServiceConfigW(service, Some(config_ptr), bytes_needed, &mut bytes_needed);

        let _ = CloseServiceHandle(service);

        if result.is_err() {
            return ServiceStartType::Unknown;
        }

        let config = &*config_ptr;

        // Map dwStartType to our enum
        match config.dwStartType.0 {
            0 => ServiceStartType::Boot,      // SERVICE_BOOT_START
            1 => ServiceStartType::System,    // SERVICE_SYSTEM_START
            2 => ServiceStartType::Automatic, // SERVICE_AUTO_START
            3 => ServiceStartType::Manual,    // SERVICE_DEMAND_START
            4 => ServiceStartType::Disabled,  // SERVICE_DISABLED
            _ => ServiceStartType::Unknown,
        }
    }
}

fn categorize_service(name: &str) -> ServiceCategory {
    let lower = name.to_lowercase();

    // Telemetry
    if TELEMETRY_SERVICES
        .iter()
        .any(|s| s.eq_ignore_ascii_case(name))
    {
        return ServiceCategory::Telemetry;
    }

    // Performance
    if PERFORMANCE_SERVICES
        .iter()
        .any(|s| s.eq_ignore_ascii_case(name))
    {
        return ServiceCategory::Performance;
    }

    // Security
    if SECURITY_SERVICES
        .iter()
        .any(|s| s.eq_ignore_ascii_case(name))
    {
        return ServiceCategory::Security;
    }

    // Network
    if NETWORK_SERVICES
        .iter()
        .any(|s| s.eq_ignore_ascii_case(name))
    {
        return ServiceCategory::Network;
    }

    // System - by prefix or list
    if SYSTEM_SERVICES
        .iter()
        .any(|s| lower.starts_with(&s.to_lowercase()))
    {
        return ServiceCategory::System;
    }

    // Additional heuristics
    if lower.contains("xbox") || lower.contains("game") {
        return ServiceCategory::Gaming;
    }

    if lower.contains("audio") || lower.contains("sound") {
        return ServiceCategory::Media;
    }

    if lower.contains("bluetooth") || lower.contains("wifi") || lower.contains("wlan") {
        return ServiceCategory::Network;
    }

    if lower.contains("print") || lower.contains("scan") {
        return ServiceCategory::Peripheral;
    }

    if lower.starts_with("microsoft") || lower.starts_with("windows") {
        return ServiceCategory::System;
    }

    ServiceCategory::Unknown
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

/// Returns active telemetry services
pub fn get_active_telemetry_services(services: &[ServiceInfo]) -> Vec<&ServiceInfo> {
    services
        .iter()
        .filter(|s| s.category == ServiceCategory::Telemetry && s.status == ServiceStatus::Running)
        .collect()
}

/// Returns services safe to disable without risk
pub fn get_safe_to_disable(services: &[ServiceInfo]) -> Vec<&ServiceInfo> {
    services
        .iter()
        .filter(|s| {
            matches!(
                s.category,
                ServiceCategory::Telemetry | ServiceCategory::Performance
            ) && s.start_type != ServiceStartType::Disabled
        })
        .collect()
}
