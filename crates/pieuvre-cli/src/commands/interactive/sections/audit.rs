use crate::commands::interactive::types::{OptItem, RiskLevel};

pub fn get_options() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "audit_hardware",
            label: "Inventaire matériel",
            description: "Analyse détaillée du CPU, du GPU et du stockage.",
            default: true,
            risk: RiskLevel::Low,
        },
        OptItem {
            id: "audit_security",
            label: "Audit des politiques de sécurité",
            description: "Vérifie l'état de Windows Defender et du pare-feu.",
            default: true,
            risk: RiskLevel::Low,
        },
        OptItem {
            id: "audit_services",
            label: "Audit des services et pilotes",
            description: "Identifie les services non standard ou suspects.",
            default: true,
            risk: RiskLevel::Medium,
        },
    ]
}
