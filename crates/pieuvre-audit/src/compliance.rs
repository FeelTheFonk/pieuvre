use crate::registry::read_dword_value;
use pieuvre_common::Result;
use windows::Win32::System::Registry::HKEY_LOCAL_MACHINE;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComplianceCheck {
    pub id: String,
    pub name: String,
    pub status: ComplianceStatus,
    pub expected: String,
    pub actual: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum ComplianceStatus {
    Compliant,
    NonCompliant,
    Error(String),
}

pub fn check_compliance() -> Result<Vec<ComplianceCheck>> {
    let mut checks = Vec::new();

    // Exemple : Vérification de la télémétrie
    let telemetry_val = read_dword_value(
        HKEY_LOCAL_MACHINE,
        r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
        "AllowTelemetry",
    )
    .unwrap_or(1);
    checks.push(ComplianceCheck {
        id: "COMP-001".to_string(),
        name: "Telemetry Disabled".to_string(),
        status: if telemetry_val == 0 {
            ComplianceStatus::Compliant
        } else {
            ComplianceStatus::NonCompliant
        },
        expected: "0".to_string(),
        actual: telemetry_val.to_string(),
    });

    Ok(checks)
}
