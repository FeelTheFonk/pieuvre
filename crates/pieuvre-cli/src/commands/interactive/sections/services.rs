use crate::commands::interactive::types::{OptItem, RiskLevel};

pub fn get_options() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "svc_telemetry",
            label: "Désactiver les services de télémétrie",
            description: "Désactive DiagTrack, dmwappushservice et WerSvc.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "svc_sysmain",
            label: "Désactiver SysMain (Superfetch)",
            description: "Désactive SysMain pour réduire les E/S disque et l'usage mémoire sur les SSD.",
            default: true,
            risk: RiskLevel::Low,
        },
        OptItem {
            id: "svc_search",
            label: "Désactiver Windows Search",
            description: "Désactive le service d'indexation. La recherche sera plus lente mais consommera moins de ressources.",
            default: false,
            risk: RiskLevel::Low,
        },
        OptItem {
            id: "svc_update",
            label: "Optimiser les services de mise à jour",
            description: "Passe les services Windows Update en manuel pour éviter l'activité en arrière-plan.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "svc_print",
            label: "Désactiver le spouleur d'impression",
            description: "Désactive les services d'impression si vous n'utilisez pas d'imprimante.",
            default: false,
            risk: RiskLevel::Safe,
        },
    ]
}
