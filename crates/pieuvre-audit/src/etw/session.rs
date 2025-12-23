//! Gestion des sessions ETW Kernel
//!
//! Permet de démarrer et d'arrêter des traces kernel pour capturer DPC/ISR.

use pieuvre_common::{PieuvreError, Result};
use std::mem::size_of;
use windows::core::{PCWSTR, PWSTR};
use windows::Win32::System::Diagnostics::Etw::{
    ControlTraceW, OpenTraceW, ProcessTrace, StartTraceW, CONTROLTRACE_HANDLE,
    EVENT_TRACE_CONTROL_QUERY, EVENT_TRACE_CONTROL_STOP, EVENT_TRACE_FLAG, EVENT_TRACE_LOGFILEW,
    EVENT_TRACE_PROPERTIES, EVENT_TRACE_REAL_TIME_MODE, EVENT_TRACE_SYSTEM_LOGGER_MODE,
    KERNEL_LOGGER_NAMEW, PROCESS_TRACE_MODE_EVENT_RECORD, PROCESS_TRACE_MODE_REAL_TIME,
    WNODE_FLAG_TRACED_GUID,
};

/// Gère une session de trace ETW Kernel
pub struct EtwSession {
    #[allow(dead_code)]
    handle: CONTROLTRACE_HANDLE,
    name: String,
}

impl EtwSession {
    /// Démarre une nouvelle session "NT Kernel Logger"
    pub fn start_kernel_session() -> Result<Self> {
        unsafe {
            let session_name = KERNEL_LOGGER_NAMEW;

            let name_bytes = (session_name.len() + 1) * 2;
            let total_size = size_of::<EVENT_TRACE_PROPERTIES>() + name_bytes + 2;

            let mut buffer = vec![0u8; total_size];
            let props = &mut *(buffer.as_mut_ptr() as *mut EVENT_TRACE_PROPERTIES);

            props.Wnode.BufferSize = total_size as u32;
            props.Wnode.Flags = WNODE_FLAG_TRACED_GUID;
            props.Wnode.Guid = windows::Win32::System::Diagnostics::Etw::SystemTraceControlGuid;
            props.LogFileMode = EVENT_TRACE_REAL_TIME_MODE | EVENT_TRACE_SYSTEM_LOGGER_MODE;

            // Flags pour DPC (0x20) et Interrupt (0x40)
            props.EnableFlags = EVENT_TRACE_FLAG(0x00000020 | 0x00000040);

            props.LoggerNameOffset = size_of::<EVENT_TRACE_PROPERTIES>() as u32;

            let mut handle = CONTROLTRACE_HANDLE::default();
            let result = StartTraceW(&mut handle, session_name, props);

            if result.is_err() {
                if result.0 as u32 == 183 {
                    // ERROR_ALREADY_EXISTS
                    Self::stop_session(session_name)?;
                    let result_retry = StartTraceW(&mut handle, session_name, props);
                    if result_retry.is_err() {
                        return Err(PieuvreError::Internal(format!(
                            "Failed to start ETW session after retry: {:?}",
                            result_retry
                        )));
                    }
                } else {
                    return Err(PieuvreError::Internal(format!(
                        "Failed to start ETW session: {:?}",
                        result
                    )));
                }
            }

            tracing::info!("Session ETW Kernel démarrée avec succès");
            Ok(Self {
                handle,
                name: "NT Kernel Logger".to_string(),
            })
        }
    }

    /// Consomme les événements de la session (bloquant)
    pub fn process_events(&self) -> Result<()> {
        use crate::etw::parser::EtwParser;

        unsafe {
            let mut log_file = EVENT_TRACE_LOGFILEW::default();
            let name_wide: Vec<u16> = self.name.encode_utf16().chain(std::iter::once(0)).collect();
            log_file.LoggerName = PWSTR(name_wide.as_ptr() as *mut _);
            log_file.Anonymous1.ProcessTraceMode =
                PROCESS_TRACE_MODE_REAL_TIME | PROCESS_TRACE_MODE_EVENT_RECORD;

            log_file.Anonymous2.EventRecordCallback = Some(EtwParser::event_record_callback);

            let trace_handle = OpenTraceW(&mut log_file);
            if trace_handle.Value == 0 || trace_handle.Value == !0 {
                return Err(PieuvreError::Internal(
                    "Failed to open ETW trace".to_string(),
                ));
            }

            let result = ProcessTrace(&[trace_handle], None, None);
            if result.is_err() {
                return Err(PieuvreError::Internal(format!(
                    "ProcessTrace failed: {:?}",
                    result
                )));
            }

            Ok(())
        }
    }

    /// Vérifie si la session "NT Kernel Logger" est déjà active
    pub fn check_active() -> Result<bool> {
        unsafe {
            let session_name = KERNEL_LOGGER_NAMEW;
            let mut buffer = vec![0u8; size_of::<EVENT_TRACE_PROPERTIES>() + 512];
            let props = &mut *(buffer.as_mut_ptr() as *mut EVENT_TRACE_PROPERTIES);
            props.Wnode.BufferSize = buffer.len() as u32;

            let result = ControlTraceW(
                CONTROLTRACE_HANDLE { Value: 0 },
                session_name,
                props,
                EVENT_TRACE_CONTROL_QUERY,
            );
            Ok(result.is_ok())
        }
    }

    fn stop_session(name: PCWSTR) -> Result<()> {
        unsafe {
            let mut buffer = vec![0u8; size_of::<EVENT_TRACE_PROPERTIES>() + 512];
            let props = &mut *(buffer.as_mut_ptr() as *mut EVENT_TRACE_PROPERTIES);
            props.Wnode.BufferSize = buffer.len() as u32;

            let _ = ControlTraceW(
                CONTROLTRACE_HANDLE { Value: 0 },
                name,
                props,
                EVENT_TRACE_CONTROL_STOP,
            );
            Ok(())
        }
    }
}

impl Drop for EtwSession {
    fn drop(&mut self) {
        let name_wide: Vec<u16> = self.name.encode_utf16().chain(std::iter::once(0)).collect();
        let _ = Self::stop_session(PCWSTR(name_wide.as_ptr()));
    }
}
