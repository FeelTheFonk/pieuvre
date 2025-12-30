//! Game Mode Tweaks
//!
//! Windows Game Mode and Game Bar optimizations.

use crate::registry::{set_dword_value, set_value_multi_hive_dword};
use pieuvre_common::Result;

/// Disable Game Bar and Game DVR
pub fn disable_game_bar() -> Result<()> {
    // Disable Game Bar
    set_value_multi_hive_dword(
        r"Software\Microsoft\Windows\CurrentVersion\GameDVR",
        "AppCaptureEnabled",
        0,
    )?;

    // Disable Game DVR
    set_value_multi_hive_dword(r"System\GameConfigStore", "GameDVR_Enabled", 0)?;

    // Disable Game Bar tips
    set_value_multi_hive_dword(r"Software\Microsoft\GameBar", "ShowStartupPanel", 0)?;

    // Disable Game Bar controller hints
    set_value_multi_hive_dword(r"Software\Microsoft\GameBar", "UseNexusForGameBarEnabled", 0)?;

    tracing::info!("Game Bar disabled");
    Ok(())
}

/// Enable Windows Game Mode (hardware optimization)
pub fn enable_game_mode() -> Result<()> {
    set_value_multi_hive_dword(r"Software\Microsoft\GameBar", "AutoGameModeEnabled", 1)?;
    tracing::info!("Game Mode enabled");
    Ok(())
}

/// Disable fullscreen optimizations globally
pub fn disable_fullscreen_optimizations() -> Result<()> {
    set_value_multi_hive_dword(r"System\GameConfigStore", "GameDVR_FSEBehaviorMode", 2)?;
    set_value_multi_hive_dword(r"System\GameConfigStore", "GameDVR_HonorUserFSEBehaviorMode", 1)?;
    set_value_multi_hive_dword(
        r"System\GameConfigStore",
        "GameDVR_DXGIHonorFSEWindowsCompatible",
        1,
    )?;

    tracing::info!("Fullscreen optimizations disabled");
    Ok(())
}

/// Disable hardware-accelerated GPU scheduling (for older games)
pub fn disable_hags() -> Result<()> {
    set_dword_value(
        r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
        "HwSchMode",
        1,
    )?;
    tracing::info!("HAGS disabled");
    Ok(())
}

/// Enable hardware-accelerated GPU scheduling
pub fn enable_hags() -> Result<()> {
    set_dword_value(
        r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
        "HwSchMode",
        2,
    )?;
    tracing::info!("HAGS enabled");
    Ok(())
}

/// Check if Game Mode is enabled
pub fn is_game_mode_enabled() -> bool {
    crate::registry::read_dword_value(r"Software\Microsoft\GameBar", "AutoGameModeEnabled")
        .map(|v| v == 1)
        .unwrap_or(false)
}

/// Set GPU Pre-Rendered Frames to 1 for minimum input lag
/// Works for NVIDIA (via registry fallback)
pub fn set_prerendered_frames(frames: u32) -> Result<()> {
    // Generic DirectX setting
    set_dword_value(r"SOFTWARE\Microsoft\DirectX", "MaxFrameLatency", frames)?;

    // NVIDIA specific (if applicable)
    set_dword_value(
        r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
        "FlipQueueSize",
        frames,
    )?;

    tracing::info!("Pre-rendered frames set to {}", frames);
    Ok(())
}

/// Reset Pre-Rendered Frames to default (3)
pub fn reset_prerendered_frames() -> Result<()> {
    crate::registry::delete_value(r"SOFTWARE\Microsoft\DirectX", "MaxFrameLatency")?;
    crate::registry::delete_value(
        r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
        "FlipQueueSize",
    )?;

    tracing::info!("Pre-rendered frames reset to default");
    Ok(())
}

/// Set DirectX Shader Cache size (in MB, 0 = disabled)
pub fn set_shader_cache_size(size_mb: u32) -> Result<()> {
    set_dword_value(
        r"SOFTWARE\Microsoft\DirectX",
        "ShaderCacheSize",
        size_mb * 1024 * 1024,
    )?;

    tracing::info!("Shader cache size set to {}MB", size_mb);
    Ok(())
}

/// Disable Variable Refresh Rate scheduling (can cause input lag in some cases)
pub fn disable_vrr_optimizations() -> Result<()> {
    set_value_multi_hive_dword(
        r"Software\Microsoft\DirectX\UserGpuPreferences",
        "VRROptimizeEnable",
        0,
    )?;
    tracing::info!("VRR optimizations disabled");
    Ok(())
}

/// Check if HAGS is enabled
pub fn is_hags_enabled() -> bool {
    crate::registry::read_dword_value(
        r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
        "HwSchMode",
    )
    .map(|v| v == 2)
    .unwrap_or(false)
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
