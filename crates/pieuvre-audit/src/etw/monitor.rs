//! Latency Monitor
//!
//! Centralise les statistiques de latence DPC/ISR pour l'affichage et l'analyse.

use crate::etw::parser::LatencyStats;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

static MONITOR: OnceLock<Arc<LatencyMonitor>> = OnceLock::new();

/// Moniteur global de latence
pub struct LatencyMonitor {
    stats: Arc<Mutex<HashMap<String, LatencyStats>>>,
}

impl LatencyMonitor {
    /// Récupère l'instance unique du moniteur
    pub fn global() -> Arc<Self> {
        MONITOR
            .get_or_init(|| {
                Arc::new(Self {
                    stats: Arc::new(Mutex::new(HashMap::new())),
                })
            })
            .clone()
    }

    /// Met à jour les statistiques pour un driver/routine
    pub fn update_dpc(&self, driver_name: String, latency_us: u64) {
        let mut stats = self
            .stats
            .lock()
            .expect("LatencyMonitor stats mutex poisoned");
        let entry = stats.entry(driver_name).or_default();
        entry.dpc_count += 1;
        entry.dpc_total_us += latency_us;
        if latency_us > entry.dpc_max_us {
            entry.dpc_max_us = latency_us;
        }
    }

    /// Met à jour les statistiques ISR
    pub fn update_isr(&self, driver_name: String, latency_us: u64) {
        let mut stats = self
            .stats
            .lock()
            .expect("LatencyMonitor stats mutex poisoned");
        let entry = stats.entry(driver_name).or_default();
        entry.isr_count += 1;
        entry.isr_total_us += latency_us;
        if latency_us > entry.isr_max_us {
            entry.isr_max_us = latency_us;
        }
    }

    /// Récupère une copie des statistiques actuelles
    pub fn get_all_stats(&self) -> HashMap<String, LatencyStats> {
        self.stats
            .lock()
            .expect("LatencyMonitor stats mutex poisoned")
            .clone()
    }

    /// Récupère la latence maximale observée
    pub fn get_max_latency(&self) -> u64 {
        self.stats
            .lock()
            .expect("LatencyMonitor stats mutex poisoned")
            .values()
            .map(|s| s.dpc_max_us.max(s.isr_max_us))
            .max()
            .unwrap_or(0)
    }
}
