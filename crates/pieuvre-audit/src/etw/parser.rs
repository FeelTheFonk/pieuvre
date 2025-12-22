//! Analyseur d'événements ETW (SOTA 2026)
//!
//! Parse les structures EVENT_RECORD pour extraire les données de latence.

use windows::Win32::System::Diagnostics::Etw::{EVENT_RECORD, EVENT_HEADER_FLAG_64_BIT_HEADER};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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
pub struct EtwParser {
    stats: Arc<Mutex<HashMap<String, LatencyStats>>>,
}

impl EtwParser {
    pub fn new() -> Self {
        Self {
            stats: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Callback appelé par ProcessTrace pour chaque événement
    pub unsafe extern "system" fn event_record_callback(record: *mut EVENT_RECORD) {
        let record = &*record;
        
        // Identification de l'événement via Opcode
        // DPC = 68, ISR = 67 (PerfInfo group)
        let opcode = record.EventHeader.EventDescriptor.Opcode;
        
        match opcode {
//! Analyseur d'événements ETW (SOTA 2026)
//!
//! Parse les structures EVENT_RECORD pour extraire les données de latence.

use windows::Win32::System::Diagnostics::Etw::{EVENT_RECORD, EVENT_HEADER_FLAG_64_BIT_HEADER};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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
pub struct EtwParser {
    stats: Arc<Mutex<HashMap<String, LatencyStats>>>,
}

impl EtwParser {
    pub fn new() -> Self {
        Self {
            stats: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Callback appelé par ProcessTrace pour chaque événement
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
                
                tracing::trace!("DPC detected: routine={:x}, latency={}us", routine, latency_us);
                
                // Note: Dans un callback statique, on a besoin d'un accès global ou d'un singleton
                // Pour SOTA, on utilisera une variable statique ou on passera le contexte via EventContext
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
                tracing::trace!("ISR detected: routine={:x}, latency={}us", routine, latency_us);
            }
        }
    }

    pub fn get_stats(&self) -> HashMap<String, LatencyStats> {
        self.stats.lock().unwrap().clone()
    }
}
