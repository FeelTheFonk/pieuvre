//! Game Mode Tweaks
//!
//! Windows Game Mode and Game Bar optimizations.

use pieuvre_common::Result;
use std::process::Command;

/// Disable Game Bar and Game DVR
pub fn disable_game_bar() -> Result<()> {
    // Disable Game Bar
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\GameDVR",
            "/v", "AppCaptureEnabled",
            "/t", "REG_DWORD",
            "/d", "0",
            "/f"
        ])
        .output();
    
    // Disable Game DVR
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKCU\System\GameConfigStore",
            "/v", "GameDVR_Enabled",
            "/t", "REG_DWORD",
            "/d", "0",
            "/f"
        ])
        .output();
    
    // Disable Game Bar tips
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKCU\Software\Microsoft\GameBar",
            "/v", "ShowStartupPanel",
            "/t", "REG_DWORD",
            "/d", "0",
            "/f"
        ])
        .output();
    
    // Disable Game Bar controller hints
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKCU\Software\Microsoft\GameBar",
            "/v", "UseNexusForGameBarEnabled",
            "/t", "REG_DWORD",
            "/d", "0",
            "/f"
        ])
        .output();
    
    tracing::info!("Game Bar disabled");
    Ok(())
}

/// Enable Windows Game Mode (hardware optimization)
pub fn enable_game_mode() -> Result<()> {
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKCU\Software\Microsoft\GameBar",
            "/v", "AutoGameModeEnabled",
            "/t", "REG_DWORD",
            "/d", "1",
            "/f"
        ])
        .output();
    
    tracing::info!("Game Mode enabled");
    Ok(())
}

/// Disable fullscreen optimizations globally
pub fn disable_fullscreen_optimizations() -> Result<()> {
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKCU\System\GameConfigStore",
            "/v", "GameDVR_FSEBehaviorMode",
            "/t", "REG_DWORD",
            "/d", "2",
            "/f"
        ])
        .output();
    
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKCU\System\GameConfigStore",
            "/v", "GameDVR_HonorUserFSEBehaviorMode",
            "/t", "REG_DWORD",
            "/d", "1",
            "/f"
        ])
        .output();
    
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKCU\System\GameConfigStore",
            "/v", "GameDVR_DXGIHonorFSEWindowsCompatible",
            "/t", "REG_DWORD",
            "/d", "1",
            "/f"
        ])
        .output();
    
    tracing::info!("Fullscreen optimizations disabled");
    Ok(())
}

/// Disable hardware-accelerated GPU scheduling (for older games)
pub fn disable_hags() -> Result<()> {
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
            "/v", "HwSchMode",
            "/t", "REG_DWORD",
            "/d", "1",
            "/f"
        ])
        .output();
    
    tracing::info!("HAGS disabled");
    Ok(())
}

/// Enable hardware-accelerated GPU scheduling
pub fn enable_hags() -> Result<()> {
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
            "/v", "HwSchMode",
            "/t", "REG_DWORD",
            "/d", "2",
            "/f"
        ])
        .output();
    
    tracing::info!("HAGS enabled");
    Ok(())
}

/// Check if Game Mode is enabled
pub fn is_game_mode_enabled() -> bool {
    let output = Command::new("reg")
        .args([
            "query",
            r"HKCU\Software\Microsoft\GameBar",
            "/v", "AutoGameModeEnabled"
        ])
        .output();
    
    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            stdout.contains("0x1")
        }
        Err(_) => false,
    }
}
