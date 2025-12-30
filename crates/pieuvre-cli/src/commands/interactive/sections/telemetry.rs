use crate::commands::interactive::types::{OptItem, RiskLevel};

pub fn get_options() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "diagtrack",
            label: "Désactiver DiagTrack (SOTA)",
            description: "Désactive le service 'Expériences des utilisateurs connectés et télémétrie', pivot central de la collecte de données Windows.",
            default: true,
            risk: RiskLevel::Low,
        },
        OptItem {
            id: "dmwappush",
            label: "Désactiver WAP Push",
            description: "Désactive 'dmwappushservice' pour neutraliser le routage furtif des données de télémétrie.",
            default: true,
            risk: RiskLevel::Low,
        },
        OptItem {
            id: "wersvc",
            label: "Désactiver Rapport d'erreurs (WER)",
            description: "Neutralise Windows Error Reporting pour empêcher l'exfiltration de rapports de plantage vers Microsoft.",
            default: true,
            risk: RiskLevel::Low,
        },
        OptItem {
            id: "firewall",
            label: "Hardening Pare-feu",
            description: "Déploie des règles de blocage sortant strictes pour les endpoints de télémétrie Microsoft connus.",
            default: true,
            risk: RiskLevel::Low,
        },
        OptItem {
            id: "sched_tasks",
            label: "Tâches Planifiées (Hardening)",
            description: "Désactive exhaustivement les tâches planifiées liées à la collecte de données (CEIP, SQM, etc.).",
            default: true,
            risk: RiskLevel::Low,
        },
        OptItem {
            id: "hosts",
            label: "Filtrage DNS (Hosts)",
            description: "Injecte plus de 500 domaines de télémétrie dans le fichier hosts pour un blocage au niveau système.",
            default: true,
            risk: RiskLevel::Low,
        },
        OptItem {
            id: "onedrive",
            label: "Débloat OneDrive",
            description: "Suppression complète de OneDrive, incluant les résidus de registre et les points de montage shell.",
            default: false,
            risk: RiskLevel::Low,
        },
    ]
}
