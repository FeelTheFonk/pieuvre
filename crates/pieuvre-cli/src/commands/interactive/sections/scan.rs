use crate::commands::interactive::types::{OptItem, RiskLevel};

pub fn get_options() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "scan_yara",
            label: "Analyse de signatures YARA-X",
            description: "Analyse approfondie des menaces via signatures YARA-X (Tech Preview).",
            default: false,
            risk: RiskLevel::Low,
        },
        OptItem {
            id: "scan_browser",
            label: "Analyse forensique des navigateurs",
            description: "Analyse l'historique et les extensions des navigateurs pour détecter des menaces.",
            default: false,
            risk: RiskLevel::Low,
        },
        OptItem {
            id: "scan_registry",
            label: "Analyse de persistance du registre",
            description: "Vérifie les clés de registre malveillantes et les éléments de démarrage (ASEP).",
            default: false,
            risk: RiskLevel::Medium,
        },
    ]
}
