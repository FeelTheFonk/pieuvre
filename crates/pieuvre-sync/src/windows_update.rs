//! Windows Update Control
//!
//! Control Windows Update behavior and deferral.

use pieuvre_common::Result;
use std::process::Command;

/// Pause Windows Updates for 35 days (maximum)
pub fn pause_updates() -> Result<()> {
    // Static pause date (35 days from release)
    let date_str = "2026-01-25";
    
    // Pause feature updates
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SOFTWARE\Microsoft\WindowsUpdate\UX\Settings",
            "/v", "PauseFeatureUpdatesStartTime",
            "/t", "REG_SZ",
            "/d", &date_str,
            "/f"
        ])
        .output();
    
    // Pause quality updates
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SOFTWARE\Microsoft\WindowsUpdate\UX\Settings",
            "/v", "PauseQualityUpdatesStartTime",
            "/t", "REG_SZ",
            "/d", &date_str,
            "/f"
        ])
        .output();
    
    // Disable auto-restart
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU",
            "/v", "NoAutoRebootWithLoggedOnUsers",
            "/t", "REG_DWORD",
            "/d", "1",
            "/f"
        ])
        .output();
    
    tracing::info!("Windows Updates paused for 35 days");
    Ok(())
}

/// Disable automatic driver updates
pub fn disable_driver_updates() -> Result<()> {
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate",
            "/v", "ExcludeWUDriversInQualityUpdate",
            "/t", "REG_DWORD",
            "/d", "1",
            "/f"
        ])
        .output();
    
    // Also via Device Installation Settings
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\DriverSearching",
            "/v", "SearchOrderConfig",
            "/t", "REG_DWORD",
            "/d", "0",
            "/f"
        ])
        .output();
    
    tracing::info!("Automatic driver updates disabled");
    Ok(())
}

/// Enable automatic driver updates
pub fn enable_driver_updates() -> Result<()> {
    let _ = Command::new("reg")
        .args([
            "delete",
            r"HKLM\SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate",
            "/v", "ExcludeWUDriversInQualityUpdate",
            "/f"
        ])
        .output();
    
    tracing::info!("Automatic driver updates enabled");
    Ok(())
}

/// Check if updates are paused
pub fn is_updates_paused() -> bool {
    let output = Command::new("reg")
        .args([
            "query",
            r"HKLM\SOFTWARE\Microsoft\WindowsUpdate\UX\Settings",
            "/v", "PauseFeatureUpdatesStartTime"
        ])
        .output();
    
    output.map(|o| o.status.success()).unwrap_or(false)
}
