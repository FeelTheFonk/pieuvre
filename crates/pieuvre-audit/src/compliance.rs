//! Compliance Audit
//!
//! Drift detection compared to optimized settings.

use crate::registry::read_dword_value;
use pieuvre_common::Result;

/// Compliance report
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComplianceReport {
    pub is_compliant: bool,
    pub drifts: Vec<DriftRecord>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DriftRecord {
    pub component: String,
    pub setting: String,
    pub expected: String,
    pub actual: String,
    pub severity: ComplianceSeverity,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ComplianceSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Checks configuration drift compared to standards
pub fn check_configuration_drift() -> Result<ComplianceReport> {
    let mut drifts = Vec::new();

    // 1. Timer Resolution verification (PriorityControl)
    if let Ok(val) = read_dword_value(
        r"SYSTEM\CurrentControlSet\Control\PriorityControl",
        "Win32PrioritySeparation",
    ) {
        if val != 0x26 && val != 0x18 && val != 0x2 {
            drifts.push(DriftRecord {
                component: "Kernel".to_string(),
                setting: "Win32PrioritySeparation".to_string(),
                expected: "0x26 or 0x18".to_string(),
                actual: format!("{:#x}", val),
                severity: ComplianceSeverity::High,
            });
        }
    }

    // 2. Telemetry verification (DiagTrack)
    // Note: We could check service state here if we had access to services.rs in audit
    // But audit is supposed to be read-only and independent.

    // 3. MMCSS verification
    if let Ok(val) = read_dword_value(
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile",
        "SystemResponsiveness",
    ) {
        if val > 10 {
            drifts.push(DriftRecord {
                component: "MMCSS".to_string(),
                setting: "SystemResponsiveness".to_string(),
                expected: "10".to_string(),
                actual: val.to_string(),
                severity: ComplianceSeverity::Medium,
            });
        }
    }

    Ok(ComplianceReport {
        is_compliant: drifts.is_empty(),
        drifts,
    })
}
