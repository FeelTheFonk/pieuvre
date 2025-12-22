//! Scheduled Tasks Management
//!
//! Disable Windows telemetry and diagnostic scheduled tasks.

use pieuvre_common::Result;
use std::process::Command;

/// Telemetry scheduled tasks to disable (SOTA - Sophia Script reference)
const TELEMETRY_TASKS: &[&str] = &[
    // Microsoft Compatibility Appraiser
    r"\Microsoft\Windows\Application Experience\Microsoft Compatibility Appraiser",
    r"\Microsoft\Windows\Application Experience\ProgramDataUpdater",
    r"\Microsoft\Windows\Application Experience\StartupAppTask",
    // Customer Experience Improvement Program
    r"\Microsoft\Windows\Customer Experience Improvement Program\Consolidator",
    r"\Microsoft\Windows\Customer Experience Improvement Program\UsbCeip",
    r"\Microsoft\Windows\Customer Experience Improvement Program\KernelCeipTask",
    // Disk Diagnostics
    r"\Microsoft\Windows\DiskDiagnostic\Microsoft-Windows-DiskDiagnosticDataCollector",
    // Family Safety
    r"\Microsoft\Windows\Shell\FamilySafetyMonitor",
    r"\Microsoft\Windows\Shell\FamilySafetyRefresh",
    r"\Microsoft\Windows\Shell\FamilySafetyRefreshTask",
    // Feedback
    r"\Microsoft\Windows\Feedback\Siuf\DmClient",
    r"\Microsoft\Windows\Feedback\Siuf\DmClientOnScenarioDownload",
    // Maps
    r"\Microsoft\Windows\Maps\MapsToastTask",
    r"\Microsoft\Windows\Maps\MapsUpdateTask",
    // Office Telemetry
    r"\Microsoft\Office\OfficeTelemetryAgentFallBack2016",
    r"\Microsoft\Office\OfficeTelemetryAgentLogOn2016",
    // Windows Error Reporting
    r"\Microsoft\Windows\Windows Error Reporting\QueueReporting",
    // Diagnostic
    r"\Microsoft\Windows\Power Efficiency Diagnostics\AnalyzeSystem",
    // Retail Demo
    r"\Microsoft\Windows\RetailDemo\CleanupOfflineContent",
    // Cloud Experience Host
    r"\Microsoft\Windows\CloudExperienceHost\CreateObjectTask",
    // License Validation
    r"\Microsoft\Windows\License Manager\TempSignedLicenseExchange",
    // Mobile Broadband
    r"\Microsoft\Windows\WwanSvc\NotificationTask",
    // Windows Media Player
    r"\Microsoft\Windows\WMP\WMPInfoTask",
    // Windows 11 24H2 - AI/Recall/Copilot
    r"\Microsoft\Windows\WindowsAI\AI",
    r"\Microsoft\Windows\Recall\Recall",
    r"\Microsoft\Windows\Copilot\CopilotSetup",
    r"\Microsoft\Windows\DeviceDirectoryClient\IntegratedServicesRegisterDevice",
    r"\Microsoft\Windows\DeviceDirectoryClient\RegisterDeviceAccountChange",
];

/// Disable all telemetry scheduled tasks
pub fn disable_telemetry_tasks() -> Result<Vec<String>> {
    let mut disabled = Vec::new();
    
    for task in TELEMETRY_TASKS {
        if disable_task(task).is_ok() {
            disabled.push(task.to_string());
            tracing::info!("Task disabled: {}", task);
        }
    }
    
    tracing::info!("Disabled {} telemetry tasks", disabled.len());
    Ok(disabled)
}

/// Disable a single scheduled task
pub fn disable_task(task_path: &str) -> Result<()> {
    let _ = Command::new("schtasks")
        .args(["/Change", "/TN", task_path, "/Disable"])
        .output()?;
    
    // Task might not exist, which is fine - toujours Ok
    Ok(())
}

/// Enable a single scheduled task (for rollback)
pub fn enable_task(task_path: &str) -> Result<()> {
    let _ = Command::new("schtasks")
        .args(["/Change", "/TN", task_path, "/Enable"])
        .output()?;
    
    Ok(())
}

/// Check if a task is currently enabled
pub fn is_task_enabled(task_path: &str) -> bool {
    let output = Command::new("schtasks")
        .args(["/Query", "/TN", task_path, "/FO", "CSV", "/NH"])
        .output();
    
    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            !stdout.contains("Disabled")
        }
        Err(_) => false,
    }
}

/// Get list of telemetry tasks (for UI display)
pub fn get_telemetry_tasks() -> &'static [&'static str] {
    TELEMETRY_TASKS
}
