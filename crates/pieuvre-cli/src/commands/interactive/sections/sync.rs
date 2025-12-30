use crate::commands::interactive::types::{OptItem, RiskLevel};

pub fn get_options() -> Vec<OptItem> {
    vec![OptItem {
        id: "sync_persist",
        label: "Persistence Alignment",
        description: "Ensure optimization persistence across reboots.",
        default: true,
        risk: RiskLevel::Low,
    }]
}
