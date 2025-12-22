//! Audit de Conformité SOTA 2026
//!
//! Détection de dérive (drift) par rapport aux réglages optimisés.

use crate::registry::read_dword_value;
use pieuvre_common::Result;

/// Rapport de conformité
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

/// Vérifie la dérive de configuration par rapport aux standards SOTA
pub fn check_configuration_drift() -> Result<ComplianceReport> {
    let mut drifts = Vec::new();

    // 1. Vérification Timer Resolution (PriorityControl)
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

    // 2. Vérification Télémétrie (DiagTrack)
    // Note: On pourrait vérifier l'état du service ici si on avait accès à services.rs dans audit
    // Mais audit est censé être read-only et indépendant.

    // 3. Vérification MMCSS
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
