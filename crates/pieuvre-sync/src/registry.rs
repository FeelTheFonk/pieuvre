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
