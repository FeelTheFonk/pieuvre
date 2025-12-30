use crate::commands::interactive::types::{OptItem, RiskLevel};

pub fn get_options() -> Vec<OptItem> {
    vec![OptItem {
        id: "sync_persist",
        label: "Alignement de la persistance",
        description: "Assure la persistance des optimisations après le redémarrage.",
        default: true,
        risk: RiskLevel::Low,
    }]
}
