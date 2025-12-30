use crate::commands::interactive::types::{OptItem, RiskLevel};

pub fn get_options() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "hvci",
            label: "Désactiver l'intégrité de la mémoire (HVCI)",
            description: "Désactive l'isolation du noyau pour regagner des performances significatives en jeu.",
            default: false,
            risk: RiskLevel::High,
        },
        OptItem {
            id: "vbs",
            label: "Désactiver la sécurité basée sur la virtualisation",
            description: "Désactive VBS pour réduire la charge CPU, particulièrement sur les anciens processeurs.",
            default: false,
            risk: RiskLevel::High,
        },
        OptItem {
            id: "spectre",
            label: "Désactiver les mitigations Spectre/Meltdown",
            description: "Désactive les protections de sécurité CPU pour récupérer les performances perdues. RISQUE ÉLEVÉ.",
            default: false,
            risk: RiskLevel::High,
        },
        OptItem {
            id: "uac_level",
            label: "Désactiver l'UAC (Contrôle de compte d'utilisateur)",
            description: "Règle l'UAC sur 'Ne jamais m'avertir'. Non recommandé pour la sécurité.",
            default: false,
            risk: RiskLevel::High,
        },
        OptItem {
            id: "hardening_lock",
            label: "Verrouiller les clés de registre critiques",
            description: "Applique des ACL en lecture seule sur les clés système et de confidentialité pour empêcher les réinitialisations.",
            default: false,
            risk: RiskLevel::Warning,
        },
        OptItem {
            id: "hardening_ppl",
            label: "Activer la protection PPL",
            description: "Active le 'Protected Process Light' pour le processus Pieuvre.",
            default: false,
            risk: RiskLevel::Safe,
        },
    ]
}
