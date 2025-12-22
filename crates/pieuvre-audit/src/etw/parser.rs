//! Analyseur d'événements ETW (SOTA 2026)
//!
//! Parse les structures EVENT_RECORD pour extraire les données de latence.

use windows::Win32::System::Diagnostics::Etw::EVENT_RECORD;
use std::collections::HashMap;

/// Statistiques de latence par driver
#[derive(Debug, Clone, Default)]
pub struct LatencyStats {
    pub dpc_count: u64,
    pub dpc_total_us: u64,
    pub dpc_max_us: u64,
    pub isr_count: u64,
    pub isr_total_us: u64,
    pub isr_max_us: u64,
}

/// Analyseur d'événements temps réel
pub struct EtwParser;

impl EtwParser {
    pub fn new() -> Self {
        Self
    }
}

impl Default for EtwParser {
    fn default() -> Self {
        Self::new()
    }
}

impl EtwParser {


    /// Callback appelé par ProcessTrace pour chaque événement
    /// 
    /// # Safety
    /// 
    /// Cette fonction est appelée par l'API Windows ETW. Le pointeur `record` doit être valide
    /// et pointer vers une structure `EVENT_RECORD` initialisée par le système.
    pub unsafe extern "system" fn event_record_callback(record: *mut EVENT_RECORD) {
        let record = &*record;
        
        // Identification de l'événement via Opcode
        // DPC = 68, ISR = 67 (PerfInfo group)
        let opcode = record.EventHeader.EventDescriptor.Opcode;
        
        match opcode {
            67 => Self::handle_isr(record),
            68 => Self::handle_dpc(record),
            _ => {}
        }
    }

    fn handle_dpc(record: &EVENT_RECORD) {
        // Payload DPC (PerfInfo):
        // [0-7]   InitialTime (LARGE_INTEGER)
        // [8-15]  Routine (Pointer)
        // [16-23] EndTime (LARGE_INTEGER)
        if record.UserDataLength < 24 { return; }
        
        unsafe {
            let data = record.UserData as *const u64;
            let initial_time = *data;
            let routine = *data.add(1);
            let end_time = *data.add(2);
            
            if end_time > initial_time {
                let latency_ticks = end_time - initial_time;
                let latency_us = latency_ticks / 10; 
                
                super::monitor::LatencyMonitor::global().update_dpc(routine, latency_us);
                tracing::trace!("DPC detected: routine={:x}, latency={}us", routine, latency_us);
            }
        }
    }

    fn handle_isr(record: &EVENT_RECORD) {
        // Payload ISR (PerfInfo):
        // [0-7]   InitialTime
        // [8-15]  Routine
        // [16-23] EndTime
        // [24]    Vector
        if record.UserDataLength < 24 { return; }

        unsafe {
            let data = record.UserData as *const u64;
            let initial_time = *data;
            let routine = *data.add(1);
            let end_time = *data.add(2);

            if end_time > initial_time {
                let latency_us = (end_time - initial_time) / 10;
                super::monitor::LatencyMonitor::global().update_isr(routine, latency_us);
                tracing::trace!("ISR detected: routine={:x}, latency={}us", routine, latency_us);
            }
        }
    }

    pub fn get_stats(&self) -> HashMap<String, LatencyStats> {
        super::monitor::LatencyMonitor::global().get_all_stats()
    }
}
