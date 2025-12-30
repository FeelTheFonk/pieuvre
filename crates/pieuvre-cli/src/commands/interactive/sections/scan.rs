use crate::commands::interactive::types::{OptItem, RiskLevel};

pub fn get_options() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "scan_yara",
            label: "YARA-X Signature Scan",
            description: "Deep scan for malware signatures using YARA-X.",
            default: true,
            risk: RiskLevel::Low,
        },
        OptItem {
            id: "scan_browser",
            label: "Browser Forensics",
            description: "Analyze browser history and extensions for threats.",
            default: true,
            risk: RiskLevel::Low,
        },
        OptItem {
            id: "scan_registry",
            label: "Registry Persistence Scan",
            description: "Check for malicious registry keys and startup items.",
            default: true,
            risk: RiskLevel::Medium,
        },
    ]
}
