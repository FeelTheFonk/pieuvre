use crate::commands::interactive::types::{OptItem, RiskLevel};

pub fn get_options() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "timer",
            label: "Résolution Timer 0.5ms",
            description: "Force la résolution du timer système à 0.5ms pour réduire la latence d'entrée et améliorer la fluidité.",
            default: true,
            risk: RiskLevel::Performance,
        },
        OptItem {
            id: "power_ultimate",
            label: "Plan Performances Optimales",
            description: "Déverrouille et active le mode d'alimentation 'Performances Optimales' caché de Windows.",
            default: false,
            risk: RiskLevel::Low,
        },
        OptItem {
            id: "cpu_throttle",
            label: "Désactiver Throttling CPU",
            description: "Empêche Windows de limiter les performances du processeur pour les tâches d'arrière-plan.",
            default: true,
            risk: RiskLevel::Low,
        },
        OptItem {
            id: "msi",
            label: "Mode MSI (Interruption)",
            description: "Migre les périphériques éligibles vers le mode Message Signaled Interrupts (MSI) pour réduire la latence.",
            default: true,
            risk: RiskLevel::Performance,
        },
        OptItem {
            id: "hags",
            label: "Désactiver HAGS",
            description: "Désactive la planification GPU à accélération matérielle pour éviter les micro-saccades.",
            default: false,
            risk: RiskLevel::Low,
        },
        OptItem {
            id: "nagle",
            label: "Désactiver Algorithme de Nagle",
            description: "Désactive TCP NoDelay pour réduire la latence réseau dans les jeux en ligne.",
            default: true,
            risk: RiskLevel::Performance,
        },
        OptItem {
            id: "interrupts",
            label: "Optimiser Interrupt Steering",
            description: "Dirige les pilotes à haute latence vers des cœurs CPU spécifiques pour réduire la latence DPC.",
            default: true,
            risk: RiskLevel::Performance,
        },
        OptItem {
            id: "memory",
            label: "Large System Cache",
            description: "Augmente la taille du cache de travail système pour de meilleures performances d'E/S fichiers.",
            default: true,
            risk: RiskLevel::Performance,
        },
    ]
}
