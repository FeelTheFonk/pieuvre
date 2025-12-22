//! DPC Latency Optimizations
//!
//! Reduce Deferred Procedure Call latency for smoother gaming.
//! Addresses micro-stutters, input lag, and audio crackling.

use pieuvre_common::Result;
use std::process::Command;

/// Keep kernel code in RAM (prevent paging)
/// Reduces latency spikes from disk access
pub fn disable_paging_executive() -> Result<()> {
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
            "/v", "DisablePagingExecutive",
            "/t", "REG_DWORD",
            "/d", "1",
            "/f"
        ])
        .output();
    
    tracing::info!("DisablePagingExecutive enabled - kernel stays in RAM");
    Ok(())
}

/// Enable kernel paging (restore default)
pub fn enable_paging_executive() -> Result<()> {
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
            "/v", "DisablePagingExecutive",
            "/t", "REG_DWORD",
            "/d", "0",
            "/f"
        ])
        .output();
    
    tracing::info!("DisablePagingExecutive disabled (default)");
    Ok(())
}

/// Disable Dynamic Tick for consistent timer behavior
/// Requires reboot
pub fn disable_dynamic_tick() -> Result<()> {
    let _ = Command::new("bcdedit")
        .args(["/set", "disabledynamictick", "yes"])
        .output();
    
    tracing::info!("Dynamic tick disabled - reboot required");
    Ok(())
}

/// Enable Dynamic Tick (restore default)
pub fn enable_dynamic_tick() -> Result<()> {
    let _ = Command::new("bcdedit")
        .args(["/set", "disabledynamictick", "no"])
        .output();
    
    tracing::info!("Dynamic tick enabled (default)");
    Ok(())
}

/// Set TSC sync policy to enhanced for better timer precision
pub fn set_tsc_sync_enhanced() -> Result<()> {
    let _ = Command::new("bcdedit")
        .args(["/set", "tscsyncpolicy", "enhanced"])
        .output();
    
    tracing::info!("TSC sync policy set to enhanced");
    Ok(())
}

/// Reset TSC sync policy to default
pub fn reset_tsc_sync() -> Result<()> {
    let _ = Command::new("bcdedit")
        .args(["/deletevalue", "tscsyncpolicy"])
        .output();
    
    tracing::info!("TSC sync policy reset to default");
    Ok(())
}

/// Disable HPET (High Precision Event Timer)
/// Impact varies by hardware - test with LatencyMon
pub fn disable_hpet() -> Result<()> {
    let _ = Command::new("bcdedit")
        .args(["/set", "useplatformclock", "false"])
        .output();
    
    tracing::info!("HPET disabled via bcdedit");
    Ok(())
}

/// Enable HPET
pub fn enable_hpet() -> Result<()> {
    let _ = Command::new("bcdedit")
        .args(["/set", "useplatformclock", "true"])
        .output();
    
    tracing::info!("HPET enabled");
    Ok(())
}

/// Set interrupt affinity policy to spread across cores
pub fn set_interrupt_affinity_spread() -> Result<()> {
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Kernel",
            "/v", "InterruptAffinityPolicy",
            "/t", "REG_DWORD",
            "/d", "2",
            "/f"
        ])
        .output();
    
    tracing::info!("Interrupt affinity set to spread across cores");
    Ok(())
}

/// Reset interrupt affinity policy
pub fn reset_interrupt_affinity() -> Result<()> {
    let _ = Command::new("reg")
        .args([
            "delete",
            r"HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Kernel",
            "/v", "InterruptAffinityPolicy",
            "/f"
        ])
        .output();
    
    tracing::info!("Interrupt affinity policy reset");
    Ok(())
}

/// Apply all DPC latency optimizations
pub fn apply_all_dpc_optimizations() -> Result<()> {
    disable_paging_executive()?;
    disable_dynamic_tick()?;
    set_tsc_sync_enhanced()?;
    set_interrupt_affinity_spread()?;
    
    tracing::info!("All DPC latency optimizations applied - reboot required");
    Ok(())
}

/// Reset all DPC optimizations to defaults
pub fn reset_all_dpc_optimizations() -> Result<()> {
    enable_paging_executive()?;
    enable_dynamic_tick()?;
    reset_tsc_sync()?;
    reset_interrupt_affinity()?;
    
    tracing::info!("DPC optimizations reset to defaults");
    Ok(())
}

/// Check if DisablePagingExecutive is enabled
pub fn is_paging_executive_disabled() -> bool {
    let output = Command::new("reg")
        .args([
            "query",
            r"HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
            "/v", "DisablePagingExecutive"
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
