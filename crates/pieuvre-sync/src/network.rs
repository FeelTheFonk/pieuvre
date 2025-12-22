//! Network Optimizations
//!
//! Nagle algorithm disable and TCP optimizations for gaming.

use pieuvre_common::Result;
use std::process::Command;

/// Disable Nagle's Algorithm for all network adapters
/// Reduces latency for online gaming by sending TCP packets immediately
pub fn disable_nagle_algorithm() -> Result<u32> {
    let mut modified = 0u32;
    
    // Get all network interfaces via registry
    let interfaces_key = r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces";
    let subkeys = crate::registry::list_subkeys(interfaces_key)?;
    
    for guid in subkeys {
        let key_path = format!(r"{}\{}", interfaces_key, guid);
        
        // Set TcpNoDelay
        if crate::registry::set_dword_value(&key_path, "TcpNoDelay", 1).is_ok() {
            // Set TcpAckFrequency
            let _ = crate::registry::set_dword_value(&key_path, "TcpAckFrequency", 1);
            modified += 1;
        }
    }
    
    tracing::info!("Nagle disabled on {} interfaces", modified);
    Ok(modified)
}

/// Enable Nagle's Algorithm (restore default)
pub fn enable_nagle_algorithm() -> Result<()> {
    let interfaces_key = r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces";
    let subkeys = crate::registry::list_subkeys(interfaces_key)?;
    
    for guid in subkeys {
        let key_path = format!(r"{}\{}", interfaces_key, guid);
        let _ = crate::registry::delete_value(&key_path, "TcpNoDelay");
        let _ = crate::registry::delete_value(&key_path, "TcpAckFrequency");
    }
    
    tracing::info!("Nagle re-enabled (default)");
    Ok(())
}

/// Check if Nagle is disabled on primary adapter
pub fn is_nagle_disabled() -> bool {
    let interfaces_key = r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces";
    if let Ok(subkeys) = crate::registry::list_subkeys(interfaces_key) {
        for guid in subkeys {
            let key_path = format!(r"{}\{}", interfaces_key, guid);
            if crate::registry::read_dword_value(&key_path, "TcpNoDelay").unwrap_or(0) == 1 {
                return true;
            }
        }
    }
    false
}

/// Disable Interrupt Moderation on all network adapters
/// Reduces network latency at cost of higher CPU usage
pub fn disable_interrupt_moderation() -> Result<u32> {
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            r#"Get-NetAdapterAdvancedProperty -DisplayName "*Interrupt Moderation*" | ForEach-Object { Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName "Interrupt Moderation" -RegistryValue 0 -ErrorAction SilentlyContinue }"#
        ])
        .output()?;
    
    if output.status.success() {
        tracing::info!("Interrupt Moderation disabled on all adapters");
        Ok(1)
    } else {
        tracing::warn!("Could not disable Interrupt Moderation");
        Ok(0)
    }
}

/// Enable Interrupt Moderation (restore default)
pub fn enable_interrupt_moderation() -> Result<()> {
    let _ = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            r#"Get-NetAdapterAdvancedProperty -DisplayName "*Interrupt Moderation*" | ForEach-Object { Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName "Interrupt Moderation" -RegistryValue 1 -ErrorAction SilentlyContinue }"#
        ])
        .output();
    
    tracing::info!("Interrupt Moderation enabled");
    Ok(())
}

/// Disable Large Send Offload (LSO) for reduced latency
pub fn disable_lso() -> Result<()> {
    // IPv4
    let _ = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            r#"Get-NetAdapterAdvancedProperty -DisplayName "*Large Send Offload*" | ForEach-Object { Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName $_.DisplayName -RegistryValue 0 -ErrorAction SilentlyContinue }"#
        ])
        .output();
    
    tracing::info!("Large Send Offload disabled");
    Ok(())
}

/// Enable LSO (restore default)
pub fn enable_lso() -> Result<()> {
    let _ = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            r#"Get-NetAdapterAdvancedProperty -DisplayName "*Large Send Offload*" | ForEach-Object { Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName $_.DisplayName -RegistryValue 1 -ErrorAction SilentlyContinue }"#
        ])
        .output();
    
    tracing::info!("Large Send Offload enabled");
    Ok(())
}

/// Disable Energy Efficient Ethernet for consistent performance
pub fn disable_eee() -> Result<()> {
    let _ = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            r#"Get-NetAdapterAdvancedProperty -DisplayName "*Energy Efficient Ethernet*" | ForEach-Object { Set-NetAdapterAdvancedProperty -Name $_.Name -DisplayName "Energy Efficient Ethernet" -RegistryValue 0 -ErrorAction SilentlyContinue }"#
        ])
        .output();
    
    tracing::info!("Energy Efficient Ethernet disabled");
    Ok(())
}

/// Enable Receive Side Scaling (RSS) across all cores
pub fn enable_rss() -> Result<()> {
    let _ = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            "Enable-NetAdapterRss -Name * -ErrorAction SilentlyContinue"
        ])
        .output();
    
    tracing::info!("Receive Side Scaling enabled");
    Ok(())
}

/// Disable Receive Segment Coalescing for lower latency
pub fn disable_rsc() -> Result<()> {
    let _ = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            "Disable-NetAdapterRsc -Name * -ErrorAction SilentlyContinue"
        ])
        .output();
    
    tracing::info!("Receive Segment Coalescing disabled");
    Ok(())
}

/// Apply all network latency optimizations
pub fn apply_all_network_optimizations() -> Result<u32> {
    let mut count = 0u32;
    
    count += disable_nagle_algorithm()?;
    let _ = disable_interrupt_moderation();
    let _ = disable_lso();
    let _ = disable_eee();
    let _ = enable_rss();
    let _ = disable_rsc();
    
    tracing::info!("All network latency optimizations applied");
    Ok(count)
}

