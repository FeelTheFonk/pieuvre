//! Modifications registre atomiques SOTA
//! Support natif 64-bit et multi-ruches (HKLM + HKU)

use pieuvre_common::{PieuvreError, Result};
use windows::core::{PCWSTR, PWSTR};
use windows::Win32::System::Registry::{
    RegCloseKey, RegCreateKeyExW, RegDeleteTreeW, RegDeleteValueW, RegEnumKeyExW, RegOpenKeyExW,
    RegQueryValueExW, RegSetValueExW, HKEY, HKEY_LOCAL_MACHINE, HKEY_USERS, KEY_READ,
    KEY_SET_VALUE, KEY_WOW64_64KEY, KEY_WRITE, REG_DWORD, REG_OPTION_NON_VOLATILE, REG_SZ,
};

/// Écrit une valeur DWORD dans une ruche spécifique avec support 64-bit
pub fn set_dword_value_in_hive(
    hive: HKEY,
    subkey: &str,
    value_name: &str,
    value: u32,
) -> Result<()> {
    unsafe {
        let mut hkey = Default::default();
        let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();

        RegCreateKeyExW(
            hive,
            PCWSTR(subkey_wide.as_ptr()),
            Some(0),
            None,
            REG_OPTION_NON_VOLATILE,
            KEY_WRITE | KEY_WOW64_64KEY,
            None,
            &mut hkey,
            None,
        )
        .ok()
        .map_err(|e| {
            PieuvreError::Registry(format!(
                "Cannot create/open key: {} in hive {:?}: {}",
                subkey, hive, e
            ))
        })?;

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
        )
        .ok();

        let _ = RegCloseKey(hkey);

        result.map_err(|e| {
            PieuvreError::Registry(format!(
                "Cannot set value: {} in {}: {}",
                value_name, subkey, e
            ))
        })?;

        Ok(())
    }
}

/// Écrit une valeur STRING dans une ruche spécifique avec support 64-bit
pub fn set_string_value_in_hive(
    hive: HKEY,
    subkey: &str,
    value_name: &str,
    value: &str,
) -> Result<()> {
    unsafe {
        let mut hkey = Default::default();
        let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();

        RegCreateKeyExW(
            hive,
            PCWSTR(subkey_wide.as_ptr()),
            Some(0),
            None,
            REG_OPTION_NON_VOLATILE,
            KEY_WRITE | KEY_WOW64_64KEY,
            None,
            &mut hkey,
            None,
        )
        .ok()
        .map_err(|e| {
            PieuvreError::Registry(format!(
                "Cannot create/open key: {} in hive {:?}: {}",
                subkey, hive, e
            ))
        })?;

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
        )
        .ok();

        let _ = RegCloseKey(hkey);

        result.map_err(|e| {
            PieuvreError::Registry(format!(
                "Cannot set value: {} in {}: {}",
                value_name, subkey, e
            ))
        })?;

        Ok(())
    }
}

/// Applique une valeur DWORD à HKLM et à toutes les ruches utilisateurs chargées (HKU)
pub fn set_value_multi_hive_dword(subkey: &str, value_name: &str, value: u32) -> Result<()> {
    set_dword_value_in_hive(HKEY_LOCAL_MACHINE, subkey, value_name, value)?;
    let users = list_subkeys_in_hive(HKEY_USERS)?;
    for user_sid in users {
        if user_sid.starts_with("S-1-5-21") || user_sid == ".DEFAULT" {
            let _ = set_dword_value_in_hive(
                HKEY_USERS,
                &format!("{}\\{}", user_sid, subkey),
                value_name,
                value,
            );
        }
    }
    tracing::info!("Multi-hive DWORD: {}\\{} = {}", subkey, value_name, value);
    Ok(())
}

/// Liste les sous-clés d'une ruche spécifique
pub fn list_subkeys_in_hive(hive: HKEY) -> Result<Vec<String>> {
    unsafe {
        let mut hkey = Default::default();
        RegOpenKeyExW(hive, None, Some(0), KEY_READ | KEY_WOW64_64KEY, &mut hkey)
            .ok()
            .map_err(|e| {
                PieuvreError::Registry(format!("Cannot open hive for enumeration: {}", e))
            })?;

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

// --- Fonctions de compatibilité existantes ---

pub fn set_dword_value(subkey: &str, value_name: &str, value: u32) -> Result<()> {
    set_dword_value_in_hive(HKEY_LOCAL_MACHINE, subkey, value_name, value)
}

pub fn set_string_value(subkey: &str, value_name: &str, value: &str) -> Result<()> {
    set_string_value_in_hive(HKEY_LOCAL_MACHINE, subkey, value_name, value)
}

pub fn delete_key_recursive(subkey: &str) -> Result<()> {
    unsafe {
        let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
        let result = RegDeleteTreeW(HKEY_LOCAL_MACHINE, PCWSTR(subkey_wide.as_ptr()));
        if result.is_err() && result.0 as u32 != 2 {
            return Err(PieuvreError::Registry(format!(
                "Failed to delete key: {}",
                subkey
            )));
        }
        Ok(())
    }
}

pub fn delete_value(subkey: &str, value_name: &str) -> Result<()> {
    unsafe {
        let mut hkey = Default::default();
        let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
        if RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(subkey_wide.as_ptr()),
            Some(0),
            KEY_SET_VALUE | KEY_WOW64_64KEY,
            &mut hkey,
        )
        .is_ok()
        {
            let value_wide: Vec<u16> = value_name
                .encode_utf16()
                .chain(std::iter::once(0))
                .collect();
            let _ = RegDeleteValueW(hkey, PCWSTR(value_wide.as_ptr()));
            let _ = RegCloseKey(hkey);
        }
        Ok(())
    }
}

pub fn read_dword_value(subkey: &str, value_name: &str) -> Result<u32> {
    unsafe {
        let mut hkey = Default::default();
        let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
        RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(subkey_wide.as_ptr()),
            Some(0),
            KEY_READ | KEY_WOW64_64KEY,
            &mut hkey,
        )
        .ok()
        .map_err(|e| PieuvreError::Registry(format!("Cannot open key {}: {}", subkey, e)))?;

        let value_wide: Vec<u16> = value_name
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        let mut data = 0u32;
        let mut data_size = 4u32;
        let mut val_type = REG_DWORD;

        let res = RegQueryValueExW(
            hkey,
            PCWSTR(value_wide.as_ptr()),
            None,
            Some(&mut val_type),
            Some(&mut data as *mut u32 as *mut u8),
            Some(&mut data_size),
        );
        let _ = RegCloseKey(hkey);

        res.ok().map_err(|e| {
            PieuvreError::Registry(format!("Cannot read value {}: {}", value_name, e))
        })?;
        Ok(data)
    }
}

pub fn list_subkeys(subkey: &str) -> Result<Vec<String>> {
    unsafe {
        let mut hkey = Default::default();
        let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
        RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(subkey_wide.as_ptr()),
            Some(0),
            KEY_READ | KEY_WOW64_64KEY,
            &mut hkey,
        )
        .ok()
        .map_err(|e| PieuvreError::Registry(format!("Cannot open key {}: {}", subkey, e)))?;

        let mut subkeys = Vec::new();
        let mut index = 0;
        let mut name_buffer = vec![0u16; 256];
        loop {
            let mut name_len = name_buffer.len() as u32;
            if RegEnumKeyExW(
                hkey,
                index,
                Some(PWSTR(name_buffer.as_mut_ptr())),
                &mut name_len,
                None,
                None,
                None,
                None,
            )
            .is_err()
            {
                break;
            }
            subkeys.push(String::from_utf16_lossy(&name_buffer[..name_len as usize]));
            index += 1;
        }
        let _ = RegCloseKey(hkey);
        Ok(subkeys)
    }
}

pub fn read_string_value(subkey: &str, value_name: &str) -> Result<String> {
    unsafe {
        let mut hkey = Default::default();
        let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
        RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(subkey_wide.as_ptr()),
            Some(0),
            KEY_READ | KEY_WOW64_64KEY,
            &mut hkey,
        )
        .ok()
        .map_err(|e| PieuvreError::Registry(format!("Cannot open key {}: {}", subkey, e)))?;

        let value_wide: Vec<u16> = value_name
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        let mut buffer = vec![0u8; 1024];
        let mut data_size = buffer.len() as u32;
        let mut val_type = REG_SZ;

        let res = RegQueryValueExW(
            hkey,
            PCWSTR(value_wide.as_ptr()),
            None,
            Some(&mut val_type),
            Some(buffer.as_mut_ptr()),
            Some(&mut data_size),
        );
        let _ = RegCloseKey(hkey);

        res.ok().map_err(|e| {
            PieuvreError::Registry(format!("Cannot read value {}: {}", value_name, e))
        })?;
        let chars = data_size as usize / 2;
        Ok(String::from_utf16_lossy(std::slice::from_raw_parts(
            buffer.as_ptr() as *const u16,
            chars.saturating_sub(1),
        )))
    }
}

pub fn key_exists(subkey: &str) -> bool {
    unsafe {
        let mut hkey = Default::default();
        let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
        let exists = RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(subkey_wide.as_ptr()),
            Some(0),
            KEY_READ | KEY_WOW64_64KEY,
            &mut hkey,
        )
        .is_ok();
        if exists {
            let _ = RegCloseKey(hkey);
        }
        exists
    }
}

pub fn configure_mmcss_gaming() -> Result<()> {
    let path = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile";
    set_dword_value(path, "SystemResponsiveness", 10)?;
    set_dword_value(path, "NetworkThrottlingIndex", 0xFFFFFFFF)
}

pub fn configure_games_priority() -> Result<()> {
    let path = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games";
    set_dword_value(path, "GPU Priority", 8)?;
    set_dword_value(path, "Priority", 6)?;
    set_dword_value(path, "Background Priority", 1)?;
    set_dword_value(path, "SFIO Rate", 4)
}

pub fn disable_startup_delay() -> Result<()> {
    set_dword_value(
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Serialize",
        "StartupDelayInMSec",
        0,
    )
}

pub fn reduce_shutdown_timeout() -> Result<()> {
    set_dword_value(
        r"SYSTEM\CurrentControlSet\Control",
        "WaitToKillServiceTimeout",
        2000,
    )
}

pub fn disable_power_throttling() -> Result<()> {
    set_dword_value(
        r"SYSTEM\CurrentControlSet\Control\Power\PowerThrottling",
        "PowerThrottlingOff",
        1,
    )
}

pub fn enable_power_throttling() -> Result<()> {
    delete_value(
        r"SYSTEM\CurrentControlSet\Control\Power\PowerThrottling",
        "PowerThrottlingOff",
    )
}
