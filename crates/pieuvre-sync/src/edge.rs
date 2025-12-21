//! Microsoft Edge Management
//!
//! Disable and remove Microsoft Edge browser.

use pieuvre_common::Result;
use std::process::Command;

/// Disable Microsoft Edge features without full removal
pub fn disable_edge() -> Result<()> {
    // Disable Edge auto-start
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SOFTWARE\Policies\Microsoft\Edge",
            "/v", "HideFirstRunExperience",
            "/t", "REG_DWORD",
            "/d", "1",
            "/f"
        ])
        .output();
    
    // Disable Edge sidebar
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SOFTWARE\Policies\Microsoft\Edge",
            "/v", "HubsSidebarEnabled",
            "/t", "REG_DWORD",
            "/d", "0",
            "/f"
        ])
        .output();
    
    // Disable Edge desktop shortcut creation
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SOFTWARE\Policies\Microsoft\EdgeUpdate",
            "/v", "CreateDesktopShortcutDefault",
            "/t", "REG_DWORD",
            "/d", "0",
            "/f"
        ])
        .output();
    
    // Disable Edge as default PDF handler
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SOFTWARE\Policies\Microsoft\Edge",
            "/v", "DefaultBrowserSettingEnabled",
            "/t", "REG_DWORD",
            "/d", "0",
            "/f"
        ])
        .output();
    
    // Disable Edge collections
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SOFTWARE\Policies\Microsoft\Edge",
            "/v", "EdgeCollectionsEnabled",
            "/t", "REG_DWORD",
            "/d", "0",
            "/f"
        ])
        .output();
    
    // Disable Edge shopping
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SOFTWARE\Policies\Microsoft\Edge",
            "/v", "EdgeShoppingAssistantEnabled",
            "/t", "REG_DWORD",
            "/d", "0",
            "/f"
        ])
        .output();
    
    tracing::info!("Microsoft Edge features disabled");
    Ok(())
}

/// Remove Edge scheduled tasks
pub fn remove_edge_tasks() -> Result<u32> {
    let tasks = [
        r"\Microsoft\Edge\MicrosoftEdgeUpdateTaskMachineCore",
        r"\Microsoft\Edge\MicrosoftEdgeUpdateTaskMachineUA",
        r"\Microsoft\EdgeUpdate\MicrosoftEdgeUpdateBrowserReplacementTask",
    ];
    
    let mut removed = 0u32;
    for task in tasks {
        let output = Command::new("schtasks")
            .args(["/Change", "/TN", task, "/Disable"])
            .output();
        
        if output.is_ok() {
            removed += 1;
        }
    }
    
    tracing::info!("Disabled {} Edge tasks", removed);
    Ok(removed)
}

/// Check if Edge is the default browser
pub fn is_edge_default_browser() -> bool {
    let output = Command::new("reg")
        .args([
            "query",
            r"HKCU\Software\Microsoft\Windows\Shell\Associations\UrlAssociations\http\UserChoice",
            "/v", "ProgId"
        ])
        .output();
    
    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            stdout.contains("Edge")
        }
        Err(_) => false,
    }
}

/// Re-enable Edge features
pub fn enable_edge() -> Result<()> {
    let _ = Command::new("reg")
        .args([
            "delete",
            r"HKLM\SOFTWARE\Policies\Microsoft\Edge",
            "/f"
        ])
        .output();
    
    tracing::info!("Microsoft Edge re-enabled");
    Ok(())
}
