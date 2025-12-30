use crate::commands::interactive::types::{OptItem, RiskLevel};

pub fn get_options() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "cleanup_temp",
            label: "Nettoyer les fichiers temporaires",
            description: "Supprime les fichiers temporaires de Windows et des répertoires utilisateur.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "cleanup_winsxs",
            label: "Nettoyer WinSxS (Windows Update)",
            description: "Exécute le nettoyage des composants DISM pour réduire l'usage disque système.",
            default: false,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "explorer_optimize",
            label: "Optimiser les paramètres de l'Explorateur",
            description: "Affiche les extensions, les fichiers cachés et désactive les éléments récents.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "hardening_unlock",
            label: "Déverrouiller les clés de registre critiques",
            description: "Restaure les permissions par défaut (à utiliser avant une désinstallation).",
            default: false,
            risk: RiskLevel::Warning,
        },
        OptItem {
            id: "windows_update",
            label: "Configurer Windows Update (Manuel)",
            description: "Passe Windows Update en mode manuel pour éviter les redémarrages inattendus.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "cleanup_edge",
            label: "Nettoyer le cache Edge",
            description: "Supprime le cache et les données temporaires du navigateur Microsoft Edge.",
            default: true,
            risk: RiskLevel::Safe,
        },
    ]
}
