//! Memory Optimizations
//!
//! Working Set trimming, Large System Cache, and I/O Page Lock Limit.

use pieuvre_common::Result;
use windows::Win32::System::ProcessStatus::EmptyWorkingSet;
use windows::Win32::System::Threading::GetCurrentProcess;

/// Trim Working Set of the current process
/// Frees up memory by moving unused pages to the standby list
pub fn trim_current_working_set() -> Result<()> {
    unsafe {
        if EmptyWorkingSet(GetCurrentProcess()).is_ok() {
            tracing::debug!("Current process working set trimmed");
        } else {
            tracing::warn!("Failed to trim current working set");
        }
    }
    Ok(())
}

/// Enable Large System Cache
/// Optimizes file system performance by using more RAM for caching
pub fn enable_large_system_cache() -> Result<()> {
    crate::registry::set_dword_value(
        r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
        "LargeSystemCache",
        1,
    )?;
    tracing::info!("Large System Cache enabled");
    Ok(())
}

/// Disable Large System Cache (restore default)
pub fn disable_large_system_cache() -> Result<()> {
    crate::registry::set_dword_value(
        r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
        "LargeSystemCache",
        0,
    )?;
    tracing::info!("Large System Cache disabled (default)");
    Ok(())
}

/// Set I/O Page Lock Limit
/// Increases the amount of memory that can be locked for I/O operations
/// 0 means dynamic/default, but setting a high value can help high-speed I/O
pub fn set_io_page_lock_limit(max_bytes: u32) -> Result<()> {
    crate::registry::set_dword_value(
        r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
        "IoPageLockLimit",
        max_bytes,
    )?;
    tracing::info!("IoPageLockLimit set to {} bytes", max_bytes);
    Ok(())
}

/// Apply all SOTA memory optimizations
pub fn apply_sota_memory_optimizations() -> Result<()> {
    enable_large_system_cache()?;
    // 512MB for IoPageLockLimit (example SOTA value for high-end systems)
    set_io_page_lock_limit(512 * 1024 * 1024)?;
    trim_current_working_set()?;
    Ok(())
}
