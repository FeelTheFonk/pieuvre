//! CPU Optimizations
//!
//! Core parking, processor power management, and memory compression.

use pieuvre_common::Result;
use std::process::Command;
use windows::Win32::System::Threading::{
    GetCurrentProcess, SetProcessInformation, PROCESS_INFORMATION_CLASS,
};


/// Disable CPU Core Parking - keeps all cores active
/// Prevents latency from core wake-up
pub fn disable_core_parking() -> Result<()> {
    // Core Parking - Min Cores (AC)
    let _ = Command::new("powercfg")
        .args([
            "/setacvalueindex",
            "scheme_current",
            "54533251-82be-4824-96c1-47b60b740d00",
            "0cc5b647-c1df-4637-891a-edc3318ea174",
            "100",
        ])
        .output();

    // Core Parking - Min Cores (DC)
    let _ = Command::new("powercfg")
        .args([
            "/setdcvalueindex",
            "scheme_current",
            "54533251-82be-4824-96c1-47b60b740d00",
            "0cc5b647-c1df-4637-891a-edc3318ea174",
            "100",
        ])
        .output();

    // Apply changes
    let _ = Command::new("powercfg")
        .args(["/setactive", "scheme_current"])
        .output();

    tracing::info!("CPU Core Parking disabled - all cores active");
    Ok(())
}

/// Enable CPU Core Parking (restore default)
pub fn enable_core_parking() -> Result<()> {
    let _ = Command::new("powercfg")
        .args([
            "/setacvalueindex",
            "scheme_current",
            "54533251-82be-4824-96c1-47b60b740d00",
            "0cc5b647-c1df-4637-891a-edc3318ea174",
            "0",
        ])
        .output();

    let _ = Command::new("powercfg")
        .args(["/setactive", "scheme_current"])
        .output();

    tracing::info!("CPU Core Parking enabled (default)");
    Ok(())
}

/// Disable Memory Compression
/// Reduces CPU overhead for systems with 16GB+ RAM
pub fn disable_memory_compression() -> Result<()> {
    let _ = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            "Disable-MMAgent -MemoryCompression -ErrorAction SilentlyContinue",
        ])
        .output();

    tracing::info!("Memory Compression disabled");
    Ok(())
}

/// Enable Memory Compression (restore default)
pub fn enable_memory_compression() -> Result<()> {
    let _ = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            "Enable-MMAgent -MemoryCompression -ErrorAction SilentlyContinue",
        ])
        .output();

    tracing::info!("Memory Compression enabled");
    Ok(())
}

/// Disable Superfetch/SysMain memory prefetch
/// Already in services.rs but with registry backup
pub fn disable_superfetch_registry() -> Result<()> {
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\PrefetchParameters",
            "/v", "EnableSuperfetch",
            "/t", "REG_DWORD",
            "/d", "0",
            "/f"
        ])
        .output();

    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\PrefetchParameters",
            "/v", "EnablePrefetcher",
            "/t", "REG_DWORD",
            "/d", "0",
            "/f"
        ])
        .output();

    tracing::info!("Superfetch/Prefetch disabled via registry");
    Ok(())
}

/// Set Page File to static size (reduces fragmentation)
/// Recommended: 1.5x RAM for gaming
pub fn set_static_page_file(size_mb: u32) -> Result<()> {
    let size_str = size_mb.to_string();

    // Disable automatic management
    let _ = Command::new("wmic")
        .args([
            "computersystem",
            "where",
            "name=\"%computername%\"",
            "set",
            "AutomaticManagedPagefile=False",
        ])
        .output();

    // Set static size on C:
    let _ = Command::new("wmic")
        .args([
            "pagefileset",
            "where",
            "name=\"C:\\\\pagefile.sys\"",
            "set",
            &format!("InitialSize={},MaximumSize={}", size_str, size_str),
        ])
        .output();

    tracing::info!("Page file set to static {}MB", size_mb);
    Ok(())
}

/// Reset Page File to automatic management
pub fn reset_page_file() -> Result<()> {
    let _ = Command::new("wmic")
        .args([
            "computersystem",
            "where",
            "name=\"%computername%\"",
            "set",
            "AutomaticManagedPagefile=True",
        ])
        .output();

    tracing::info!("Page file reset to automatic");
    Ok(())
}

/// Set Win32PrioritySeparation (CPU Quantum)
/// 0x26 (38) is often recommended for gaming (Short, Variable, High boost)
pub fn set_cpu_quantum(value: u32) -> Result<()> {
    crate::registry::set_priority_separation(value)?;
    tracing::info!("Win32PrioritySeparation set to 0x{:X}", value);
    Ok(())
}

/// Set I/O Priority for the current process to High
/// This ensures the tool itself has priority during sync/audit
pub fn set_current_process_io_priority_high() -> Result<()> {
    unsafe {
        let mut io_priority = 3i32; // IoPriorityHigh

        let result = SetProcessInformation(
            GetCurrentProcess(),
            PROCESS_INFORMATION_CLASS(9), // ProcessIoPriority
            &mut io_priority as *mut _ as *mut _,
            std::mem::size_of::<i32>() as u32,
        );

        if result.is_err() {
            tracing::warn!("Failed to set I/O priority: {:?}", result);
        } else {
            tracing::debug!("Current process I/O priority set to High");
        }
    }
    Ok(())
}

/// Apply all CPU optimizations for gaming
pub fn apply_gaming_cpu_optimizations() -> Result<()> {
    disable_core_parking()?;
    disable_memory_compression()?;
    disable_superfetch_registry()?;

    tracing::info!("All CPU gaming optimizations applied");
    Ok(())
}

/// Check if Memory Compression is enabled
pub fn is_memory_compression_enabled() -> bool {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", "(Get-MMAgent).MemoryCompression"])
        .output();

    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            stdout.trim().eq_ignore_ascii_case("true")
        }
        Err(_) => true, // Assume enabled by default
    }
}

/// Check if Core Parking is disabled (all cores at 100%)
pub fn is_core_parking_disabled() -> bool {
    let output = Command::new("powercfg")
        .args([
            "/query",
            "scheme_current",
            "54533251-82be-4824-96c1-47b60b740d00",
            "0cc5b647-c1df-4637-891a-edc3318ea174",
        ])
        .output();

    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            stdout.contains("0x00000064") // 100 in hex
        }
        Err(_) => false,
    }
}
