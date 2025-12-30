use crate::commands::interactive::types::{OptItem, RiskLevel};

pub fn get_options() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "oo_telemetry",
            label: "O&O : Désactiver toute la télémétrie (SOTA)",
            description: "Applique tous les blocages de télémétrie recommandés par O&O, incluant CEIP et SQM.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "oo_advertising",
            label: "O&O : Désactiver l'ID de publicité",
            description: "Désactive l'ID de publicité pour tous les utilisateurs (HKLM + HKU).",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "oo_copilot",
            label: "O&O : Désactiver Windows Copilot",
            description: "Désactive globalement l'intégration de Copilot et les services en arrière-plan.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "oo_recall",
            label: "O&O : Bloquer Windows Recall",
            description: "Empêche l'analyse des données par l'IA et la sauvegarde des captures (Recall).",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "oo_widgets",
            label: "O&O : Désactiver les Widgets et Actualités",
            description: "Supprime les Widgets de la barre des tâches et désactive le service Dsh.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "oo_search_highlights",
            label: "O&O : Désactiver les points forts de la recherche",
            description: "Supprime les suggestions Web et les points forts de la recherche Windows.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "oo_wudo",
            label: "O&O : Désactiver WUDO (Optimisation de livraison)",
            description: "Force Windows Update à utiliser uniquement le HTTP local.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "oo_wifi_sense",
            label: "O&O : Désactiver Wi-Fi Sense",
            description: "Empêche la connexion automatique aux points d'accès ouverts.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "oo_app_permissions",
            label: "O&O : Verrouiller les permissions d'applications",
            description: "Refuse globalement l'accès à la caméra, au microphone et à la localisation.",
            default: false,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "oo_bg_apps",
            label: "O&O : Désactiver les applications en arrière-plan",
            description: "Empêche les applications non essentielles de s'exécuter en arrière-plan.",
            default: true,
            risk: RiskLevel::Safe,
        },
    ]
}
