use crate::commands::interactive::types::{OptItem, RiskLevel};

pub fn get_options() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "audit_hardware",
            label: "Hardware Inventory",
            description: "Detailed analysis of CPU, GPU, and Storage.",
            default: true,
            risk: RiskLevel::Low,
        },
        OptItem {
            id: "audit_security",
            label: "Security Policy Audit",
            description: "Verify Windows Defender and Firewall status.",
            default: true,
            risk: RiskLevel::Low,
        },
        OptItem {
            id: "audit_services",
            label: "Services & Drivers Audit",
            description: "Identify non-standard or suspicious services.",
            default: true,
            risk: RiskLevel::Medium,
        },
    ]
}
