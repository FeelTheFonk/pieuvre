use crate::commands::interactive::types::{OptItem, RiskLevel};

pub fn get_options() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "maint_cleanup_full",
            label: "Full System Purge",
            description: "Deep cleaning of Temp, WinSxS, and all browser caches.",
            default: false,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "maint_updates_pause",
            label: "Pause Windows Updates",
            description: "Pauses Windows Updates for 35 days to prevent forced reboots.",
            default: false,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "maint_tasks",
            label: "Disable Telemetry Tasks",
            description: "Disables scheduled tasks that collect and send data to Microsoft.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "maint_hibernation",
            label: "Disable Hibernation",
            description: "Disables hibernation and deletes hiberfil.sys to save disk space.",
            default: false,
            risk: RiskLevel::Safe,
        },
    ]
}
