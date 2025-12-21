//! OneDrive Management
//!
//! Uninstall and disable OneDrive completely.

use pieuvre_common::Result;
use std::process::Command;
use std::env;
use std::path::PathBuf;

/// Uninstall OneDrive completely
pub fn uninstall_onedrive() -> Result<()> {
    tracing::info!("Uninstalling OneDrive...");
    
    // Kill OneDrive process
    let _ = Command::new("taskkill")
        .args(["/F", "/IM", "OneDrive.exe"])
        .output();
    
    // Find OneDrive setup
    let localappdata = env::var("LOCALAPPDATA").unwrap_or_default();
    let systemroot = env::var("SYSTEMROOT").unwrap_or_else(|_| "C:\\Windows".to_string());
    
    let paths = [
        format!("{}\\OneDriveSetup.exe", systemroot),
        format!("{}\\SysWOW64\\OneDriveSetup.exe", systemroot),
        format!("{}\\Microsoft\\OneDrive\\OneDriveSetup.exe", localappdata),
    ];
    
    for path in paths {
        if PathBuf::from(&path).exists() {
            let output = Command::new(&path)
                .args(["/uninstall"])
                .output();
            
            if output.is_ok() {
                tracing::info!("OneDrive uninstalled via {}", path);
                break;
            }
        }
    }
    
    // Remove OneDrive folders
    let folders = [
        format!("{}\\OneDrive", env::var("USERPROFILE").unwrap_or_default()),
        format!("{}\\Microsoft\\OneDrive", localappdata),
        format!("{}\\OneDrive", env::var("PROGRAMDATA").unwrap_or_default()),
    ];
    
    for folder in folders {
        let _ = std::fs::remove_dir_all(&folder);
    }
    
    // Remove from explorer sidebar via registry
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SOFTWARE\Policies\Microsoft\Windows\OneDrive",
            "/v", "DisableFileSyncNGSC",
            "/t", "REG_DWORD",
            "/d", "1",
            "/f"
        ])
        .output();
    
    tracing::info!("OneDrive completely removed");
    Ok(())
}

/// Disable OneDrive sync without uninstalling
pub fn disable_onedrive() -> Result<()> {
    // Disable via Group Policy registry
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SOFTWARE\Policies\Microsoft\Windows\OneDrive",
            "/v", "DisableFileSyncNGSC",
            "/t", "REG_DWORD",
            "/d", "1",
            "/f"
        ])
        .output();
    
    // Disable startup
    let _ = Command::new("reg")
        .args([
            "delete",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run",
            "/v", "OneDrive",
            "/f"
        ])
        .output();
    
    tracing::info!("OneDrive disabled");
    Ok(())
}

/// Check if OneDrive is installed
pub fn is_onedrive_installed() -> bool {
    let localappdata = env::var("LOCALAPPDATA").unwrap_or_default();
    let path = format!("{}\\Microsoft\\OneDrive\\OneDrive.exe", localappdata);
    PathBuf::from(path).exists()
}

/// Re-enable OneDrive (for rollback)
pub fn enable_onedrive() -> Result<()> {
    let _ = Command::new("reg")
        .args([
            "delete",
            r"HKLM\SOFTWARE\Policies\Microsoft\Windows\OneDrive",
            "/v", "DisableFileSyncNGSC",
            "/f"
        ])
        .output();
    
    tracing::info!("OneDrive re-enabled (requires reinstall from Microsoft Store)");
    Ok(())
}
