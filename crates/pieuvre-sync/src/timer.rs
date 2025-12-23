//! Timer Resolution Optimization
//!
//! Configuration of system timer resolution via NtSetTimerResolution.

use pieuvre_common::{PieuvreError, Result};

#[link(name = "ntdll")]
extern "system" {
    fn NtSetTimerResolution(
        DesiredResolution: u32,
        SetResolution: u8,
        CurrentResolution: *mut u32,
    ) -> i32;

    fn NtQueryTimerResolution(
        MinimumResolution: *mut u32,
        MaximumResolution: *mut u32,
        CurrentResolution: *mut u32,
    ) -> i32;
}

/// Retrieves the current timer resolution (in 100ns)
pub fn get_timer_resolution() -> Result<TimerResolutionInfo> {
    let mut min = 0u32;
    let mut max = 0u32;
    let mut current = 0u32;

    let status = unsafe { NtQueryTimerResolution(&mut min, &mut max, &mut current) };

    if status >= 0 {
        Ok(TimerResolutionInfo {
            minimum_100ns: min,
            maximum_100ns: max,
            current_100ns: current,
        })
    } else {
        Err(PieuvreError::Unsupported(format!(
            "NtQueryTimerResolution: {}",
            status
        )))
    }
}

/// Configures the timer resolution
///
/// # Arguments
/// * `resolution_100ns` - Resolution in 100ns units (5000 = 0.5ms, 10000 = 1ms)
pub fn set_timer_resolution(resolution_100ns: u32) -> Result<u32> {
    let mut actual = 0u32;

    let status = unsafe { NtSetTimerResolution(resolution_100ns, 1, &mut actual) };

    if status >= 0 {
        tracing::info!(
            "Timer resolution: {}00ns -> {}00ns",
            resolution_100ns / 100,
            actual / 100
        );
        Ok(actual)
    } else {
        Err(PieuvreError::Unsupported(format!(
            "NtSetTimerResolution: {}",
            status
        )))
    }
}

#[derive(Debug, Clone)]
pub struct TimerResolutionInfo {
    /// Minimum supported resolution (in 100ns)
    pub minimum_100ns: u32,
    /// Maximum supported resolution (in 100ns) - the finest
    pub maximum_100ns: u32,
    /// Current resolution (in 100ns)
    pub current_100ns: u32,
}

impl TimerResolutionInfo {
    /// Converts the current resolution to milliseconds
    pub fn current_ms(&self) -> f64 {
        self.current_100ns as f64 / 10000.0
    }

    /// Converts the minimum resolution (coarsest) to milliseconds
    pub fn min_ms(&self) -> f64 {
        self.minimum_100ns as f64 / 10000.0
    }

    /// Converts the maximum resolution (finest) to milliseconds
    pub fn max_ms(&self) -> f64 {
        self.maximum_100ns as f64 / 10000.0
    }

    /// Converts the maximum resolution (finest) to milliseconds
    pub fn best_ms(&self) -> f64 {
        self.maximum_100ns as f64 / 10000.0
    }
}

/// Resets the timer resolution to the default value (15.625ms)
pub fn reset_timer_resolution() -> Result<u32> {
    // 156250 = 15.625ms in 100ns units
    let mut actual = 0u32;

    let status = unsafe { NtSetTimerResolution(156250, 0, &mut actual) };

    if status >= 0 {
        tracing::info!("Timer resolution reset to default (15.625ms)");
        Ok(actual)
    } else {
        Err(PieuvreError::Unsupported(format!(
            "NtSetTimerResolution reset: {}",
            status
        )))
    }
}
