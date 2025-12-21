//! Windows Explorer Tweaks
//!
//! Optimize Windows Explorer behavior and appearance.

use pieuvre_common::Result;
use std::process::Command;

/// Apply Explorer performance tweaks
pub fn apply_explorer_tweaks() -> Result<()> {
    // Show file extensions
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "/v", "HideFileExt",
            "/t", "REG_DWORD",
            "/d", "0",
            "/f"
        ])
        .output();
    
    // Show hidden files
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "/v", "Hidden",
            "/t", "REG_DWORD",
            "/d", "1",
            "/f"
        ])
        .output();
    
    // Disable recent files in Quick Access
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer",
            "/v", "ShowRecent",
            "/t", "REG_DWORD",
            "/d", "0",
            "/f"
        ])
        .output();
    
    // Disable frequent folders in Quick Access
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer",
            "/v", "ShowFrequent",
            "/t", "REG_DWORD",
            "/d", "0",
            "/f"
        ])
        .output();
    
    // Open Explorer to This PC instead of Quick Access
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "/v", "LaunchTo",
            "/t", "REG_DWORD",
            "/d", "1",
            "/f"
        ])
        .output();
    
    // Disable search highlights
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\SearchSettings",
            "/v", "IsDynamicSearchBoxEnabled",
            "/t", "REG_DWORD",
            "/d", "0",
            "/f"
        ])
        .output();
    
    // Disable chat icon on taskbar
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "/v", "TaskbarMn",
            "/t", "REG_DWORD",
            "/d", "0",
            "/f"
        ])
        .output();
    
    // Disable task view button
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "/v", "ShowTaskViewButton",
            "/t", "REG_DWORD",
            "/d", "0",
            "/f"
        ])
        .output();
    
    tracing::info!("Explorer tweaks applied");
    Ok(())
}

/// Restart Explorer to apply changes
pub fn restart_explorer() -> Result<()> {
    let _ = Command::new("taskkill")
        .args(["/F", "/IM", "explorer.exe"])
        .output();
    
    let _ = Command::new("cmd")
        .args(["/c", "start", "explorer.exe"])
        .output();
    
    tracing::info!("Explorer restarted");
    Ok(())
}

/// Restore default Explorer settings
pub fn restore_explorer_defaults() -> Result<()> {
    let keys = [
        ("HideFileExt", "1"),
        ("Hidden", "2"),
        ("ShowRecent", "1"),
        ("ShowFrequent", "1"),
        ("LaunchTo", "2"),
    ];
    
    for (key, value) in keys {
        let _ = Command::new("reg")
            .args([
                "add",
                r"HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "/v", key,
                "/t", "REG_DWORD",
                "/d", value,
                "/f"
            ])
            .output();
    }
    
    tracing::info!("Explorer defaults restored");
    Ok(())
}
