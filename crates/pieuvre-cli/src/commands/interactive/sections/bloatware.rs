use crate::commands::interactive::types::{OptItem, RiskLevel};

pub fn get_options() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "bloat_copilot",
            label: "Supprimer Microsoft Copilot",
            description: "Désinstalle Copilot et supprime son intégration de la barre des tâches et des paramètres.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "bloat_onedrive",
            label: "Désinstaller Microsoft OneDrive",
            description: "Supprime complètement OneDrive du système et arrête ses processus en arrière-plan.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "bloat_edge",
            label: "Désactiver le superflu de Microsoft Edge",
            description: "Désactive les services en arrière-plan d'Edge, le boost au démarrage et la télémétrie.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "bloat_standard",
            label: "Supprimer les Bloatwares standards",
            description: "Supprime les applications pré-installées communes (Solitaire, People, Maps, etc.).",
            default: false,
            risk: RiskLevel::Low,
        },
        OptItem {
            id: "bloat_cortana",
            label: "Désactiver Cortana",
            description: "Désactive l'assistant vocal Cortana et son intégration à la recherche.",
            default: true,
            risk: RiskLevel::Safe,
        },
    ]
}
