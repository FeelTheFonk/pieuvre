use crate::commands::interactive::types::{OptItem, RiskLevel};

pub fn get_options() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "maint_cleanup_full",
            label: "Purge complète du système",
            description: "Nettoyage approfondi des fichiers temporaires, WinSxS et de tous les caches de navigateurs.",
            default: false,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "maint_updates_pause",
            label: "Suspendre Windows Update",
            description: "Suspend les mises à jour Windows pendant 35 jours pour éviter les redémarrages forcés.",
            default: false,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "maint_tasks",
            label: "Désactiver les tâches de télémétrie",
            description: "Désactive les tâches planifiées qui collectent et envoient des données à Microsoft.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "maint_hibernation",
            label: "Désactiver l'hibernation",
            description: "Désactive l'hibernation et supprime hiberfil.sys pour économiser de l'espace disque.",
            default: false,
            risk: RiskLevel::Safe,
        },
    ]
}
