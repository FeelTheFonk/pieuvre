//! Widgets Management
//!
//! Disable Windows 11 widgets board and service.

use pieuvre_common::Result;
use std::process::Command;

/// Disable Windows 11 Widgets completely
pub fn disable_widgets() -> Result<()> {
    // Disable via Group Policy registry
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SOFTWARE\Policies\Microsoft\Dsh",
            "/v", "AllowNewsAndInterests",
            "/t", "REG_DWORD",
            "/d", "0",
            "/f"
        ])
        .output();
    
    // Disable taskbar widgets button
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "/v", "TaskbarDa",
            "/t", "REG_DWORD",
            "/d", "0",
            "/f"
        ])
        .output();
    
    // Disable Widgets service startup
    let _ = Command::new("sc")
        .args(["config", "Widgets", "start=", "disabled"])
        .output();
    
    // Kill widget process
    let _ = Command::new("taskkill")
        .args(["/F", "/IM", "Widgets.exe"])
        .output();
    
    tracing::info!("Widgets disabled");
    Ok(())
}

/// Enable Widgets
pub fn enable_widgets() -> Result<()> {
    let _ = Command::new("reg")
        .args([
            "delete",
            r"HKLM\SOFTWARE\Policies\Microsoft\Dsh",
            "/v", "AllowNewsAndInterests",
            "/f"
        ])
        .output();
    
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "/v", "TaskbarDa",
            "/t", "REG_DWORD",
            "/d", "1",
            "/f"
        ])
        .output();
    
    tracing::info!("Widgets enabled");
    Ok(())
}

/// Check if widgets are disabled
pub fn is_widgets_disabled() -> bool {
    let output = Command::new("reg")
        .args([
            "query",
            r"HKLM\SOFTWARE\Policies\Microsoft\Dsh",
            "/v", "AllowNewsAndInterests"
        ])
        .output();
    
    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            stdout.contains("0x0")
        }
        Err(_) => false,
    }
}
