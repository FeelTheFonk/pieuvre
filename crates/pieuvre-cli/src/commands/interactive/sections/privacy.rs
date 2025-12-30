use crate::commands::interactive::types::{OptItem, RiskLevel};

pub fn get_options() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "telemetry_level",
            label: "Niveau de télémétrie : Sécurité (0)",
            description: "Force le niveau de télémétrie sur 'Sécurité' (Entreprise/Education) ou 'Basique' via GPO.",
            default: true,
            risk: RiskLevel::Low,
        },
        OptItem {
            id: "advertising_id",
            label: "Désactiver l'ID de publicité",
            description: "Empêche les applications d'utiliser l'ID de publicité pour des expériences personnalisées.",
            default: true,
            risk: RiskLevel::Low,
        },
        OptItem {
            id: "location",
            label: "Désactiver la localisation",
            description: "Désactive globalement les services de localisation et efface l'historique.",
            default: true,
            risk: RiskLevel::Low,
        },
        OptItem {
            id: "activity_history",
            label: "Désactiver l'historique d'activité",
            description: "Empêche Windows de collecter vos activités et de les synchroniser avec le cloud.",
            default: true,
            risk: RiskLevel::Low,
        },
        OptItem {
            id: "cortana",
            label: "Désactiver Cortana et suggestions de recherche",
            description: "Désactive Cortana et empêche les résultats Web d'apparaître dans la recherche Windows.",
            default: true,
            risk: RiskLevel::Low,
        },
        OptItem {
            id: "recall",
            label: "Désactiver Windows Recall (IA)",
            description: "Bloque la fonction Windows Recall pour empêcher les captures et l'analyse de l'activité.",
            default: true,
            risk: RiskLevel::Low,
        },
        OptItem {
            id: "context_menu",
            label: "Menu contextuel classique (Win11)",
            description: "Restaure le menu contextuel classique de Windows 10 et supprime l'encombrement.",
            default: true,
            risk: RiskLevel::Low,
        },
    ]
}
