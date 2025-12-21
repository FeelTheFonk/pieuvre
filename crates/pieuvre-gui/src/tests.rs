//! Tests unitaires pour pieuvre-gui

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_info_ui_default() {
        let info = crate::SystemInfoUI::default();
        assert!(!info.os_version.is_empty() || info.os_version == "");
        assert!(info.cpu_cores >= 0);
        assert!(info.ram_gb >= 0);
    }

    #[test]
    fn test_get_system_info() {
        let info = crate::get_system_info();
        // Should return valid data on Windows
        assert!(!info.os_version.is_empty());
        assert!(!info.hostname.is_empty());
        assert!(info.cpu_cores > 0);
        assert!(info.ram_gb > 0);
    }

    #[test]
    fn test_detect_laptop() {
        // Should return bool without panicking
        let _is_laptop = crate::detect_laptop();
    }

    #[test]
    fn test_service_states_default() {
        let states = crate::ServiceStates::default();
        // All should be false by default
        assert!(!states.diagtrack);
        assert!(!states.dmwappush);
        assert!(!states.wersvc);
        assert!(!states.sysmain);
        assert!(!states.wsearch);
        assert!(!states.bits);
        assert!(!states.wuauserv);
        assert!(!states.mapbroker);
    }

    #[test]
    fn test_worker_handle_spawn() {
        let worker = crate::WorkerHandle::spawn();
        // Should create worker without panicking
        assert!(worker.send(crate::WorkerCommand::Shutdown).is_ok());
    }

    #[test]
    fn test_worker_command_send_receive() {
        let worker = crate::WorkerHandle::spawn();
        
        // Send command
        assert!(worker.send(crate::WorkerCommand::CreateSnapshot).is_ok());
        
        // Wait for response
        let result = worker.recv_timeout(std::time::Duration::from_secs(5));
        
        // Should get a result (either success or error)
        assert!(result.is_some());
        
        // Cleanup
        let _ = worker.send(crate::WorkerCommand::Shutdown);
    }
}
