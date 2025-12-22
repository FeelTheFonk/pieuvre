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

/// Set GPU Pre-Rendered Frames to 1 for minimum input lag
/// Works for NVIDIA (via registry fallback)
pub fn set_prerendered_frames(frames: u32) -> Result<()> {
    // Generic DirectX setting
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SOFTWARE\Microsoft\DirectX",
            "/v", "MaxFrameLatency",
            "/t", "REG_DWORD",
            "/d", &frames.to_string(),
            "/f"
        ])
        .output();
    
    // NVIDIA specific (if applicable)
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
            "/v", "FlipQueueSize",
            "/t", "REG_DWORD",
            "/d", &frames.to_string(),
            "/f"
        ])
        .output();
    
    tracing::info!("Pre-rendered frames set to {}", frames);
    Ok(())
}

/// Reset Pre-Rendered Frames to default (3)
pub fn reset_prerendered_frames() -> Result<()> {
    let _ = Command::new("reg")
        .args([
            "delete",
            r"HKLM\SOFTWARE\Microsoft\DirectX",
            "/v", "MaxFrameLatency",
            "/f"
        ])
        .output();
    
    let _ = Command::new("reg")
        .args([
            "delete",
            r"HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
            "/v", "FlipQueueSize",
            "/f"
        ])
        .output();
    
    tracing::info!("Pre-rendered frames reset to default");
    Ok(())
}

/// Set DirectX Shader Cache size (in MB, 0 = disabled)
pub fn set_shader_cache_size(size_mb: u32) -> Result<()> {
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SOFTWARE\Microsoft\DirectX",
            "/v", "ShaderCacheSize",
            "/t", "REG_DWORD",
            "/d", &(size_mb * 1024 * 1024).to_string(), // Convert to bytes
            "/f"
        ])
        .output();
    
    tracing::info!("Shader cache size set to {}MB", size_mb);
    Ok(())
}

/// Disable Variable Refresh Rate scheduling (can cause input lag in some cases)
pub fn disable_vrr_optimizations() -> Result<()> {
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKCU\Software\Microsoft\DirectX\UserGpuPreferences",
            "/v", "VRROptimizeEnable",
            "/t", "REG_DWORD",
            "/d", "0",
            "/f"
        ])
        .output();
    
    tracing::info!("VRR optimizations disabled");
    Ok(())
}

/// Check if HAGS is enabled
pub fn is_hags_enabled() -> bool {
    let output = Command::new("reg")
        .args([
            "query",
            r"HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
            "/v", "HwSchMode"
        ])
        .output();
    
    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            stdout.contains("0x2") // 2 = Enabled
        }
        Err(_) => false,
    }
}

/// Apply all GPU/Gaming optimizations for minimum input lag
pub fn apply_all_gpu_optimizations() -> Result<()> {
    disable_game_bar()?;
    enable_game_mode()?;
    disable_fullscreen_optimizations()?;
    set_prerendered_frames(1)?;
    
    tracing::info!("All GPU optimizations applied");
    Ok(())
}

