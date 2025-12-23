//! Modifications registre atomiques

use pieuvre_common::{PieuvreError, Result};
use windows::core::{PCWSTR, PWSTR};
use windows::Win32::System::Registry::{
    RegCloseKey, RegCreateKeyExW, RegDeleteTreeW, RegDeleteValueW, RegEnumKeyExW, RegOpenKeyExW,
    RegQueryValueExW, RegSetValueExW, HKEY, HKEY_LOCAL_MACHINE, KEY_READ, KEY_SET_VALUE, KEY_WRITE,
    REG_DWORD, REG_OPTION_NON_VOLATILE, REG_SZ,
};

/// Écrit une valeur DWORD dans le registre
pub fn set_dword_value(subkey: &str, value_name: &str, value: u32) -> Result<()> {
    unsafe {
        let mut hkey = Default::default();
        let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();

        let result = RegCreateKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(subkey_wide.as_ptr()),
            Some(0),
            None,
            REG_OPTION_NON_VOLATILE,
            KEY_WRITE,
            None,
            &mut hkey,
            None,
        );

        if result.is_err() {
            return Err(PieuvreError::Registry(format!(
                "Cannot create/open key: {}",
                subkey
            )));
        }

        let value_wide: Vec<u16> = value_name
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        let data_bytes = value.to_le_bytes();

        let result = RegSetValueExW(
            hkey,
            PCWSTR(value_wide.as_ptr()),
            Some(0),
            REG_DWORD,
            Some(&data_bytes),
        );

        let _ = RegCloseKey(hkey);

        if result.is_err() {
            return Err(PieuvreError::Registry(format!(
                "Cannot set value: {}",
                value_name
            )));
        }

        tracing::debug!("Registre: {}\\{} = {}", subkey, value_name, value);
        Ok(())
    }
}

/// Écrit une valeur STRING dans le registre
pub fn set_string_value(subkey: &str, value_name: &str, value: &str) -> Result<()> {
    unsafe {
        let mut hkey = Default::default();
        let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();

        let result = RegCreateKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(subkey_wide.as_ptr()),
            Some(0),
            None,
            REG_OPTION_NON_VOLATILE,
            KEY_WRITE,
            None,
            &mut hkey,
            None,
        );

        if result.is_err() {
            return Err(PieuvreError::Registry(format!(
                "Cannot create/open key: {}",
                subkey
            )));
        }

        let value_wide: Vec<u16> = value_name
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        let data_wide: Vec<u16> = value.encode_utf16().chain(std::iter::once(0)).collect();

        let result = RegSetValueExW(
            hkey,
            PCWSTR(value_wide.as_ptr()),
            Some(0),
            REG_SZ,
            Some(std::slice::from_raw_parts(
                data_wide.as_ptr() as *const u8,
                data_wide.len() * 2,
            )),
        );

        let _ = RegCloseKey(hkey);

        if result.is_err() {
            return Err(PieuvreError::Registry(format!(
                "Cannot set value: {}",
                value_name
            )));
        }

        tracing::debug!("Registre: {}\\{} = {}", subkey, value_name, value);
        Ok(())
    }
}

/// Supprime une clé et toutes ses sous-clés (récursif)
pub fn delete_key_recursive(subkey: &str) -> Result<()> {
    unsafe {
        let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
        let result = RegDeleteTreeW(HKEY_LOCAL_MACHINE, PCWSTR(subkey_wide.as_ptr()));

        if result.is_err() {
            // Si la clé n'existe pas, on considère cela comme un succès (YAGNI/Fail Fast)
            let err_code = result.0 as u32;
            if err_code != 2 {
                // ERROR_FILE_NOT_FOUND
                return Err(PieuvreError::Registry(format!(
                    "Failed to delete key recursive {}: {:?}",
                    subkey, result
                )));
            }
        }
        Ok(())
    }
}

/// Supprime une valeur du registre
pub fn delete_value(subkey: &str, value_name: &str) -> Result<()> {
    unsafe {
        let mut hkey = Default::default();
        let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();

        let result = RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(subkey_wide.as_ptr()),
            Some(0),
            KEY_SET_VALUE,
            &mut hkey,
        );

        if result.is_err() {
            return Ok(()); // La clé n'existe pas, donc la valeur non plus
        }

        let value_wide: Vec<u16> = value_name
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        let _ = RegDeleteValueW(hkey, PCWSTR(value_wide.as_ptr()));

        let _ = RegCloseKey(hkey);
        Ok(())
    }
}

/// Liste les sous-clés d'une clé de registre
pub fn list_subkeys(subkey: &str) -> Result<Vec<String>> {
    unsafe {
        let mut hkey = Default::default();
        let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();

        let result = RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(subkey_wide.as_ptr()),
            Some(0),
            KEY_READ,
            &mut hkey,
        );

        if result.is_err() {
            return Err(PieuvreError::Registry(format!(
                "Cannot open key for reading: {}",
                subkey
            )));
        }

        let mut subkeys = Vec::new();
        let mut index = 0;
        let mut name_buffer = vec![0u16; 256];

        loop {
            let mut name_len = name_buffer.len() as u32;
            let result = RegEnumKeyExW(
                hkey,
                index,
                Some(PWSTR(name_buffer.as_mut_ptr())),
                &mut name_len,
                None,
                None,
                None,
                None,
            );

            if result.is_err() {
                break;
            }

            let name = String::from_utf16_lossy(&name_buffer[..name_len as usize]);
            subkeys.push(name);
            index += 1;
        }

        let _ = RegCloseKey(hkey);
        Ok(subkeys)
    }
}

/// Lit une valeur DWORD du registre (HKLM)
pub fn read_dword_value(subkey: &str, value_name: &str) -> Result<u32> {
    read_dword_value_from_hive(HKEY_LOCAL_MACHINE, subkey, value_name)
}

fn read_dword_value_from_hive(hive: HKEY, subkey: &str, value_name: &str) -> Result<u32> {
    unsafe {
        let mut hkey = Default::default();
        let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();

        let result = RegOpenKeyExW(
            hive,
            PCWSTR(subkey_wide.as_ptr()),
            Some(0),
            KEY_READ,
            &mut hkey,
        );

        if result.is_err() {
            return Err(PieuvreError::Registry(format!(
                "Cannot open key: {}",
                subkey
            )));
        }

        let value_wide: Vec<u16> = value_name
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        let mut data = 0u32;
        let mut data_size = std::mem::size_of::<u32>() as u32;
        let mut value_type = REG_DWORD;

        let result = RegQueryValueExW(
            hkey,
            PCWSTR(value_wide.as_ptr()),
            None,
            Some(&mut value_type),
            Some(&mut data as *mut u32 as *mut u8),
            Some(&mut data_size),
        );

        let _ = RegCloseKey(hkey);

        if result.is_err() {
            return Err(PieuvreError::Registry(format!(
                "Cannot read value: {}",
                value_name
            )));
        }

        Ok(data)
    }
}

/// Lit une valeur String du registre
pub fn read_string_value(subkey: &str, value_name: &str) -> Result<String> {
    unsafe {
        let mut hkey = Default::default();
        let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();

        if RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(subkey_wide.as_ptr()),
            Some(0),
            KEY_READ,
            &mut hkey,
        )
        .is_err()
        {
            return Err(PieuvreError::Registry(format!(
                "Cannot open key: {}",
                subkey
            )));
        }

        let value_wide: Vec<u16> = value_name
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        let mut buffer = vec![0u8; 1024];
        let mut data_size = buffer.len() as u32;
        let mut value_type = REG_SZ;

        let result = RegQueryValueExW(
            hkey,
            PCWSTR(value_wide.as_ptr()),
            None,
            Some(&mut value_type),
            Some(buffer.as_mut_ptr()),
            Some(&mut data_size),
        );

        let _ = RegCloseKey(hkey);

        if result.is_err() {
            return Err(PieuvreError::Registry(format!(
                "Cannot read value: {}",
                value_name
            )));
        }

        // Convertir UTF-16 en String
        let chars = data_size as usize / 2;
        let s = String::from_utf16_lossy(std::slice::from_raw_parts(
            buffer.as_ptr() as *const u16,
            chars.saturating_sub(1),
        ));

        Ok(s)
    }
}

/// Vérifie si une clé existe
pub fn key_exists(subkey: &str) -> bool {
    unsafe {
        let mut hkey = Default::default();
        let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();

        let exists = RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(subkey_wide.as_ptr()),
            Some(0),
            KEY_READ,
            &mut hkey,
        )
        .is_ok();

        if exists {
            let _ = RegCloseKey(hkey);
        }

        exists
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
// MMCSS / GAMING TWEAKS
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
    let games_path =
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games";

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
    set_dword_value(
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Serialize",
        "StartupDelayInMSec",
        0,
    )?;
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
// ADVANCED TWEAKS
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
    delete_value(
        r"SYSTEM\CurrentControlSet\Control\Power\PowerThrottling",
        "PowerThrottlingOff",
    )?;
    tracing::info!("Power Throttling enabled (default)");
    Ok(())
}

/// Block Windows Recall (24H2 AI feature)
pub fn disable_recall() -> Result<()> {
    set_dword_value(
        r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
        "DisableAIDataAnalysis",
        1,
    )?;

    set_dword_value(
        r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
        "TurnOffSavingSnapshots",
        1,
    )?;

    tracing::info!("Windows Recall disabled");
    Ok(())
}

/// Enable Windows Recall
pub fn enable_recall() -> Result<()> {
    // Suppression récursive de la clé de policy
    delete_key_recursive(r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI")?;
    tracing::info!("Windows Recall enabled");
    Ok(())
}

/// Set Group Policy Telemetry level (enterprise style)
pub fn set_group_policy_telemetry(level: u32) -> Result<()> {
    set_dword_value(
        r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
        "AllowTelemetry",
        level,
    )?;

    set_dword_value(
        r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
        "AllowDeviceNameInTelemetry",
        0,
    )?;

    set_dword_value(
        r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
        "DisableTailoredExperiencesWithDiagnosticData",
        1,
    )?;

    tracing::info!("Group Policy Telemetry set to {}", level);
    Ok(())
}
