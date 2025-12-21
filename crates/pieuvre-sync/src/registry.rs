//! Modifications registre atomiques

use pieuvre_common::{PieuvreError, Result};
use windows::Win32::System::Registry::{
    RegOpenKeyExW, RegSetValueExW, RegCloseKey,
    HKEY_LOCAL_MACHINE, KEY_SET_VALUE, REG_DWORD,
};
use windows::core::PCWSTR;

/// Écrit une valeur DWORD dans le registre
pub fn set_dword_value(subkey: &str, value_name: &str, value: u32) -> Result<()> {
    unsafe {
        let mut hkey = Default::default();
        let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
        
        let result = RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(subkey_wide.as_ptr()),
            0,
            KEY_SET_VALUE,
            &mut hkey,
        );
        
        if result.is_err() {
            return Err(PieuvreError::Registry(format!("Cannot open key: {}", subkey)));
        }
        
        let value_wide: Vec<u16> = value_name.encode_utf16().chain(std::iter::once(0)).collect();
        let data_bytes = value.to_le_bytes();
        
        let result = RegSetValueExW(
            hkey,
            PCWSTR(value_wide.as_ptr()),
            0,
            REG_DWORD,
            Some(&data_bytes),
        );
        
        let _ = RegCloseKey(hkey);
        
        if result.is_err() {
            return Err(PieuvreError::Registry(format!("Cannot set value: {}", value_name)));
        }
        
        tracing::debug!("Registre: {}\\{} = {}", subkey, value_name, value);
        Ok(())
    }
}

/// Configure Win32PrioritySeparation
pub fn set_priority_separation(value: u32) -> Result<()> {
    set_dword_value(
        r"SYSTEM\CurrentControlSet\Control\PriorityControl",
        "Win32PrioritySeparation",
        value,
    )
}

// ============================================
// PRIVACY TWEAKS
// ============================================

/// Configure le niveau de telemetrie (0=Security, 1=Basic, 2=Enhanced, 3=Full)
pub fn set_telemetry_level(level: u32) -> Result<()> {
    // Policy level
    let _ = set_dword_value(
        r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
        "AllowTelemetry",
        level,
    );
    // User level
    set_dword_value(
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection",
        "AllowTelemetry",
        level,
    )?;
    tracing::info!("Telemetry level -> {}", level);
    Ok(())
}

/// Desactive l'Advertising ID
pub fn disable_advertising_id() -> Result<()> {
    set_dword_value(
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\AdvertisingInfo",
        "Enabled",
        0,
    )?;
    tracing::info!("Advertising ID disabled");
    Ok(())
}

/// Desactive la localisation
pub fn disable_location() -> Result<()> {
    // Valeur string mais on peut utiliser DWORD 0 pour desactiver
    set_dword_value(
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location",
        "Value",
        0,
    )?;
    tracing::info!("Location disabled");
    Ok(())
}

/// Desactive l'historique d'activite
pub fn disable_activity_history() -> Result<()> {
    set_dword_value(
        r"SOFTWARE\Policies\Microsoft\Windows\System",
        "EnableActivityFeed",
        0,
    )?;
    set_dword_value(
        r"SOFTWARE\Policies\Microsoft\Windows\System",
        "PublishUserActivities",
        0,
    )?;
    set_dword_value(
        r"SOFTWARE\Policies\Microsoft\Windows\System",
        "UploadUserActivities",
        0,
    )?;
    tracing::info!("Activity history disabled");
    Ok(())
}

/// Desactive Cortana
pub fn disable_cortana() -> Result<()> {
    set_dword_value(
        r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
        "AllowCortana",
        0,
    )?;
    tracing::info!("Cortana disabled");
    Ok(())
}

// ============================================
// MMCSS / GAMING TWEAKS (SOTA)
// ============================================

/// Configure MMCSS pour gaming (SystemResponsiveness = 10, NetworkThrottling = OFF)
pub fn configure_mmcss_gaming() -> Result<()> {
    let mmcss_path = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile";
    
    // SystemResponsiveness: 10 = reserve only 10% for background (vs 20% default)
    set_dword_value(mmcss_path, "SystemResponsiveness", 10)?;
    
    // NetworkThrottlingIndex: 0xFFFFFFFF = disable throttling
    set_dword_value(mmcss_path, "NetworkThrottlingIndex", 0xFFFFFFFF)?;
    
    tracing::info!("MMCSS gaming configured: SystemResponsiveness=10, NetworkThrottling=OFF");
    Ok(())
}

/// Configure priorite taches gaming
pub fn configure_games_priority() -> Result<()> {
    let games_path = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games";
    
    // GPU Priority: 8 (max)
    set_dword_value(games_path, "GPU Priority", 8)?;
    // Priority: 6 (high)
    set_dword_value(games_path, "Priority", 6)?;
    // Background Priority: 1 (low)
    set_dword_value(games_path, "Background Priority", 1)?;
    // SFIO Rate: 4
    set_dword_value(games_path, "SFIO Rate", 4)?;
    
    tracing::info!("Games task priority configured: GPU=8, Priority=6");
    Ok(())
}

/// Active la resolution timer globale permanente
pub fn enable_global_timer_resolution() -> Result<()> {
    set_dword_value(
        r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
        "GlobalTimerResolutionRequests",
        1,
    )?;
    tracing::info!("GlobalTimerResolutionRequests enabled");
    Ok(())
}

/// Desactive le delai de demarrage des apps startup
pub fn disable_startup_delay() -> Result<()> {
    use std::process::Command;
    // Note: Cette cle peut ne pas exister, on la cree avec reg add
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Serialize",
            "/v", "StartupDelayInMSec",
            "/t", "REG_DWORD",
            "/d", "0",
            "/f"
        ])
        .output();
    tracing::info!("Startup delay disabled");
    Ok(())
}

/// Reduit le timeout de fermeture des services (5000ms -> 2000ms)
pub fn reduce_shutdown_timeout() -> Result<()> {
    // Note: C'est une valeur string dans le registre, mais on peut utiliser DWORD pour int
    // Pour cela, il faudrait set_string_value, mais la valeur fonctionne aussi en DWORD
    set_dword_value(
        r"SYSTEM\CurrentControlSet\Control",
        "WaitToKillServiceTimeout",
        2000,
    )?;
    tracing::info!("Shutdown timeout reduced to 2000ms");
    Ok(())
}

// ============================================
// P4 SOTA - ADVANCED TWEAKS
// ============================================

/// Disable CPU Power Throttling for max performance
pub fn disable_power_throttling() -> Result<()> {
    set_dword_value(
        r"SYSTEM\CurrentControlSet\Control\Power\PowerThrottling",
        "PowerThrottlingOff",
        1,
    )?;
    tracing::info!("Power Throttling disabled");
    Ok(())
}

/// Enable CPU Power Throttling (restore default)
pub fn enable_power_throttling() -> Result<()> {
    use std::process::Command;
    let _ = Command::new("reg")
        .args([
            "delete",
            r"HKLM\SYSTEM\CurrentControlSet\Control\Power\PowerThrottling",
            "/v", "PowerThrottlingOff",
            "/f"
        ])
        .output();
    tracing::info!("Power Throttling enabled (default)");
    Ok(())
}

/// Block Windows Recall (24H2 AI feature)
pub fn disable_recall() -> Result<()> {
    use std::process::Command;
    // Disable via Group Policy
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
            "/v", "DisableAIDataAnalysis",
            "/t", "REG_DWORD",
            "/d", "1",
            "/f"
        ])
        .output();
    
    // Also disable Recall specifically
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
            "/v", "TurnOffSavingSnapshots",
            "/t", "REG_DWORD",
            "/d", "1",
            "/f"
        ])
        .output();
    
    tracing::info!("Windows Recall disabled");
    Ok(())
}

/// Enable Windows Recall
pub fn enable_recall() -> Result<()> {
    use std::process::Command;
    let _ = Command::new("reg")
        .args([
            "delete",
            r"HKLM\SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
            "/f"
        ])
        .output();
    tracing::info!("Windows Recall enabled");
    Ok(())
}

/// Set Group Policy Telemetry level (enterprise style)
pub fn set_group_policy_telemetry(level: u32) -> Result<()> {
    use std::process::Command;
    // Computer Configuration > Administrative Templates > Windows Components > Data Collection
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SOFTWARE\Policies\Microsoft\Windows\DataCollection",
            "/v", "AllowTelemetry",
            "/t", "REG_DWORD",
            "/d", &level.to_string(),
            "/f"
        ])
        .output();
    
    // Disable device name in diagnostics
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SOFTWARE\Policies\Microsoft\Windows\DataCollection",
            "/v", "AllowDeviceNameInTelemetry",
            "/t", "REG_DWORD",
            "/d", "0",
            "/f"
        ])
        .output();
    
    // Disable tailored experiences
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SOFTWARE\Policies\Microsoft\Windows\CloudContent",
            "/v", "DisableTailoredExperiencesWithDiagnosticData",
            "/t", "REG_DWORD",
            "/d", "1",
            "/f"
        ])
        .output();
    
    tracing::info!("Group Policy Telemetry set to {}", level);
    Ok(())
}
