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
    
    // Use reg command to enumerate and modify
    let output = Command::new("reg")
        .args(["query", &format!("HKLM\\{}", interfaces_key)])
        .output()?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    for line in stdout.lines() {
        if line.contains("HKEY_LOCAL_MACHINE") && line.contains("Interfaces\\{") {
            // Extract interface GUID
            if let Some(_guid_start) = line.find('{') {
                let key_path = &line[line.find("SYSTEM").unwrap_or(0)..];
                
                // Set TcpNoDelay
                let _ = Command::new("reg")
                    .args([
                        "add",
                        &format!("HKLM\\{}", key_path),
                        "/v", "TcpNoDelay",
                        "/t", "REG_DWORD",
                        "/d", "1",
                        "/f"
                    ])
                    .output();
                
                // Set TcpAckFrequency
                let _ = Command::new("reg")
                    .args([
                        "add",
                        &format!("HKLM\\{}", key_path),
                        "/v", "TcpAckFrequency",
                        "/t", "REG_DWORD",
                        "/d", "1",
                        "/f"
                    ])
                    .output();
                
                modified += 1;
            }
        }
    }
    
    tracing::info!("Nagle disabled on {} interfaces", modified);
    Ok(modified)
}

/// Enable Nagle's Algorithm (restore default)
pub fn enable_nagle_algorithm() -> Result<()> {
    let interfaces_key = r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces";
    
    let output = Command::new("reg")
        .args(["query", &format!("HKLM\\{}", interfaces_key)])
        .output()?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    for line in stdout.lines() {
        if line.contains("HKEY_LOCAL_MACHINE") && line.contains("Interfaces\\{") {
            let key_path = &line[line.find("SYSTEM").unwrap_or(0)..];
            
            let _ = Command::new("reg")
                .args([
                    "delete",
                    &format!("HKLM\\{}", key_path),
                    "/v", "TcpNoDelay",
                    "/f"
                ])
                .output();
            
            let _ = Command::new("reg")
                .args([
                    "delete",
                    &format!("HKLM\\{}", key_path),
                    "/v", "TcpAckFrequency",
                    "/f"
                ])
                .output();
        }
    }
    
    tracing::info!("Nagle re-enabled (default)");
    Ok(())
}

/// Check if Nagle is disabled on primary adapter
pub fn is_nagle_disabled() -> bool {
    let output = Command::new("reg")
        .args([
            "query",
            r"HKLM\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces",
            "/s",
            "/v", "TcpNoDelay"
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
