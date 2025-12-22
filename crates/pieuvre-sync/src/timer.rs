//! Optimisation Timer Resolution
//!
//! Configuration de la résolution du timer système via NtSetTimerResolution.

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

/// Récupère la résolution timer actuelle (en 100ns)
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

/// Configure la résolution timer
///
/// # Arguments
/// * `resolution_100ns` - Résolution en unités de 100ns (5000 = 0.5ms, 10000 = 1ms)
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
    /// Résolution minimale supportée (en 100ns)
    pub minimum_100ns: u32,
    /// Résolution maximale supportée (en 100ns) - la plus fine
    pub maximum_100ns: u32,
    /// Résolution actuelle (en 100ns)
    pub current_100ns: u32,
}

impl TimerResolutionInfo {
    /// Convertit la résolution actuelle en millisecondes
    pub fn current_ms(&self) -> f64 {
        self.current_100ns as f64 / 10000.0
    }

    /// Convertit la résolution minimale (la plus grossière) en millisecondes
    pub fn min_ms(&self) -> f64 {
        self.minimum_100ns as f64 / 10000.0
    }

    /// Convertit la résolution maximale (la plus fine) en millisecondes
    pub fn max_ms(&self) -> f64 {
        self.maximum_100ns as f64 / 10000.0
    }

    /// Convertit la résolution maximale (la plus fine) en millisecondes
    pub fn best_ms(&self) -> f64 {
        self.maximum_100ns as f64 / 10000.0
    }
}

/// Réinitialise la résolution timer à la valeur par défaut (15.625ms)
pub fn reset_timer_resolution() -> Result<u32> {
    // 156250 = 15.625ms en unités de 100ns
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
