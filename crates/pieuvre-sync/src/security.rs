//! Security Tweaks
//!
//! VBS, HVCI, Memory Integrity, and Spectre/Meltdown mitigations.
//! WARNING: These settings reduce system security for performance gains.

use pieuvre_common::Result;
use std::process::Command;

/// Disable Memory Integrity (HVCI) - 5-10% gaming performance gain
/// WARNING: Reduces protection against kernel-level malware
pub fn disable_memory_integrity() -> Result<()> {
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SYSTEM\CurrentControlSet\Control\DeviceGuard\Scenarios\HypervisorEnforcedCodeIntegrity",
            "/v", "Enabled",
            "/t", "REG_DWORD",
            "/d", "0",
            "/f"
        ])
        .output();
    
    tracing::info!("Memory Integrity (HVCI) disabled - reboot required");
    Ok(())
}

/// Enable Memory Integrity (restore security)
pub fn enable_memory_integrity() -> Result<()> {
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SYSTEM\CurrentControlSet\Control\DeviceGuard\Scenarios\HypervisorEnforcedCodeIntegrity",
            "/v", "Enabled",
            "/t", "REG_DWORD",
            "/d", "1",
            "/f"
        ])
        .output();
    
    tracing::info!("Memory Integrity (HVCI) enabled");
    Ok(())
}

/// Disable Virtualization-Based Security completely
/// WARNING: Major security reduction
pub fn disable_vbs() -> Result<()> {
    // Disable VBS
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SYSTEM\CurrentControlSet\Control\DeviceGuard",
            "/v", "EnableVirtualizationBasedSecurity",
            "/t", "REG_DWORD",
            "/d", "0",
            "/f"
        ])
        .output();
    
    // Disable Credential Guard
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SYSTEM\CurrentControlSet\Control\DeviceGuard",
            "/v", "LsaCfgFlags",
            "/t", "REG_DWORD",
            "/d", "0",
            "/f"
        ])
        .output();
    
    // Disable UEFI lock
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SYSTEM\CurrentControlSet\Control\DeviceGuard",
            "/v", "RequirePlatformSecurityFeatures",
            "/t", "REG_DWORD",
            "/d", "0",
            "/f"
        ])
        .output();
    
    tracing::info!("VBS completely disabled - reboot required");
    Ok(())
}

/// Enable VBS (restore security)
pub fn enable_vbs() -> Result<()> {
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SYSTEM\CurrentControlSet\Control\DeviceGuard",
            "/v", "EnableVirtualizationBasedSecurity",
            "/t", "REG_DWORD",
            "/d", "1",
            "/f"
        ])
        .output();
    
    tracing::info!("VBS enabled");
    Ok(())
}

/// Disable Spectre/Meltdown mitigations
/// WARNING: Critical security risk - only for isolated gaming systems
pub fn disable_spectre_meltdown() -> Result<()> {
    // FeatureSettingsOverride = 3 disables all mitigations
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
            "/v", "FeatureSettingsOverride",
            "/t", "REG_DWORD",
            "/d", "3",
            "/f"
        ])
        .output();
    
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
            "/v", "FeatureSettingsOverrideMask",
            "/t", "REG_DWORD",
            "/d", "3",
            "/f"
        ])
        .output();
    
    tracing::info!("Spectre/Meltdown mitigations disabled - CRITICAL SECURITY RISK");
    Ok(())
}

/// Enable Spectre/Meltdown mitigations (restore security)
pub fn enable_spectre_meltdown() -> Result<()> {
    let _ = Command::new("reg")
        .args([
            "delete",
            r"HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
            "/v", "FeatureSettingsOverride",
            "/f"
        ])
        .output();
    
    let _ = Command::new("reg")
        .args([
            "delete",
            r"HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
            "/v", "FeatureSettingsOverrideMask",
            "/f"
        ])
        .output();
    
    tracing::info!("Spectre/Meltdown mitigations restored");
    Ok(())
}

/// Check if Memory Integrity is enabled
pub fn is_memory_integrity_enabled() -> bool {
    let output = Command::new("reg")
        .args([
            "query",
            r"HKLM\SYSTEM\CurrentControlSet\Control\DeviceGuard\Scenarios\HypervisorEnforcedCodeIntegrity",
            "/v", "Enabled"
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

/// Check if VBS is enabled
pub fn is_vbs_enabled() -> bool {
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            "(Get-CimInstance -ClassName Win32_DeviceGuard -Namespace root\\Microsoft\\Windows\\DeviceGuard).VirtualizationBasedSecurityStatus"
        ])
        .output();
    
    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            let status = stdout.trim();
            status == "2" // 2 = Running
        }
        Err(_) => false,
    }
}
