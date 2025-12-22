//! Security Tweaks SOTA 2026
//!
//! VBS, HVCI, Memory Integrity, and Spectre/Meltdown mitigations.
//! WARNING: These settings reduce system security for performance gains.
//!
//! Utilise les APIs Windows natives via registry.rs au lieu de Command::new("reg").

use crate::registry::set_dword_value;
use pieuvre_common::Result;

// ============================================
// CONSTANTES CHEMINS REGISTRE
// ============================================

/// Clé registre pour HVCI (Memory Integrity)
const HVCI_KEY: &str =
    r"SYSTEM\CurrentControlSet\Control\DeviceGuard\Scenarios\HypervisorEnforcedCodeIntegrity";

/// Clé registre pour Device Guard / VBS
const DEVICE_GUARD_KEY: &str = r"SYSTEM\CurrentControlSet\Control\DeviceGuard";

/// Clé registre pour Memory Management (Spectre/Meltdown)
const MEMORY_MANAGEMENT_KEY: &str =
    r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management";

// ============================================
// MEMORY INTEGRITY (HVCI)
// ============================================

/// Disable Memory Integrity (HVCI) - 5-10% gaming performance gain
/// WARNING: Reduces protection against kernel-level malware
pub fn disable_memory_integrity() -> Result<()> {
    set_dword_value(HVCI_KEY, "Enabled", 0)?;
    tracing::info!(
        key = HVCI_KEY,
        "Memory Integrity (HVCI) disabled - reboot required"
    );
    Ok(())
}

/// Enable Memory Integrity (restore security)
pub fn enable_memory_integrity() -> Result<()> {
    set_dword_value(HVCI_KEY, "Enabled", 1)?;
    tracing::info!(key = HVCI_KEY, "Memory Integrity (HVCI) enabled");
    Ok(())
}

// ============================================
// VIRTUALIZATION-BASED SECURITY (VBS)
// ============================================

/// Disable Virtualization-Based Security completely
/// WARNING: Major security reduction
pub fn disable_vbs() -> Result<()> {
    // Disable VBS
    set_dword_value(DEVICE_GUARD_KEY, "EnableVirtualizationBasedSecurity", 0)?;

    // Disable Credential Guard
    set_dword_value(DEVICE_GUARD_KEY, "LsaCfgFlags", 0)?;

    // Disable UEFI lock
    set_dword_value(DEVICE_GUARD_KEY, "RequirePlatformSecurityFeatures", 0)?;

    tracing::info!("VBS completely disabled - reboot required");
    Ok(())
}

/// Enable VBS (restore security)
pub fn enable_vbs() -> Result<()> {
    set_dword_value(DEVICE_GUARD_KEY, "EnableVirtualizationBasedSecurity", 1)?;
    tracing::info!("VBS enabled");
    Ok(())
}

// ============================================
// SPECTRE / MELTDOWN MITIGATIONS
// ============================================

/// Disable Spectre/Meltdown mitigations
/// WARNING: Critical security risk - only for isolated gaming systems
pub fn disable_spectre_meltdown() -> Result<()> {
    // FeatureSettingsOverride = 3 disables all mitigations
    set_dword_value(MEMORY_MANAGEMENT_KEY, "FeatureSettingsOverride", 3)?;
    set_dword_value(MEMORY_MANAGEMENT_KEY, "FeatureSettingsOverrideMask", 3)?;

    tracing::warn!("Spectre/Meltdown mitigations disabled - CRITICAL SECURITY RISK");
    Ok(())
}

/// Enable Spectre/Meltdown mitigations (restore security)
/// Note: Suppression des valeurs = Windows utilise les defaults (mitigations ON)
pub fn enable_spectre_meltdown() -> Result<()> {
    crate::registry::delete_value(MEMORY_MANAGEMENT_KEY, "FeatureSettingsOverride")?;
    crate::registry::delete_value(MEMORY_MANAGEMENT_KEY, "FeatureSettingsOverrideMask")?;

    tracing::info!("Spectre/Meltdown mitigations restored");
    Ok(())
}

// ============================================
// STATUS CHECKS (READ-ONLY)
// ============================================

/// Check if Memory Integrity is enabled via registry read
pub fn is_memory_integrity_enabled() -> bool {
    use windows::core::PCWSTR;
    use windows::Win32::System::Registry::{
        RegCloseKey, RegOpenKeyExW, RegQueryValueExW, HKEY_LOCAL_MACHINE, KEY_READ, REG_DWORD,
    };

    unsafe {
        let subkey_wide: Vec<u16> = HVCI_KEY.encode_utf16().chain(std::iter::once(0)).collect();
        let mut hkey = Default::default();

        if RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(subkey_wide.as_ptr()),
            Some(0),
            KEY_READ,
            &mut hkey,
        )
        .is_err()
        {
            return false;
        }

        let value_name: Vec<u16> = "Enabled".encode_utf16().chain(std::iter::once(0)).collect();
        let mut data: u32 = 0;
        let mut data_size = std::mem::size_of::<u32>() as u32;
        let mut value_type = REG_DWORD;

        let result = RegQueryValueExW(
            hkey,
            PCWSTR(value_name.as_ptr()),
            None,
            Some(&mut value_type),
            Some(&mut data as *mut u32 as *mut u8),
            Some(&mut data_size),
        );

        let _ = RegCloseKey(hkey);

        result.is_ok() && data == 1
    }
}

/// Check if VBS is enabled via registry read
pub fn is_vbs_enabled() -> bool {
    use windows::core::PCWSTR;
    use windows::Win32::System::Registry::{
        RegCloseKey, RegOpenKeyExW, RegQueryValueExW, HKEY_LOCAL_MACHINE, KEY_READ, REG_DWORD,
    };

    unsafe {
        let subkey_wide: Vec<u16> = DEVICE_GUARD_KEY
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        let mut hkey = Default::default();

        if RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(subkey_wide.as_ptr()),
            Some(0),
            KEY_READ,
            &mut hkey,
        )
        .is_err()
        {
            return false;
        }

        let value_name: Vec<u16> = "EnableVirtualizationBasedSecurity"
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        let mut data: u32 = 0;
        let mut data_size = std::mem::size_of::<u32>() as u32;
        let mut value_type = REG_DWORD;

        let result = RegQueryValueExW(
            hkey,
            PCWSTR(value_name.as_ptr()),
            None,
            Some(&mut value_type),
            Some(&mut data as *mut u32 as *mut u8),
            Some(&mut data_size),
        );

        let _ = RegCloseKey(hkey);

        result.is_ok() && data != 0
    }
}
