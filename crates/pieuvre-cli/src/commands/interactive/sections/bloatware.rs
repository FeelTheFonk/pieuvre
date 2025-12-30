use crate::commands::interactive::types::{OptItem, RiskLevel};

pub fn get_options() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "bloat_copilot",
            label: "Remove Microsoft Copilot",
            description:
                "Uninstalls Copilot and removes its integration from the taskbar and settings.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "bloat_onedrive",
            label: "Uninstall Microsoft OneDrive",
            description:
                "Completely removes OneDrive from the system and stops its background processes.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "bloat_edge",
            label: "Disable Microsoft Edge Bloat",
            description: "Disables Edge background services, startup boost, and telemetry.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "bloat_standard",
            label: "Remove Standard Bloatware",
            description: "Removes common pre-installed apps like Solitaire, People, Maps, etc.",
            default: false,
            risk: RiskLevel::Low,
        },
        OptItem {
            id: "bloat_cortana",
            label: "Disable Cortana",
            description: "Disables Cortana voice assistant and its search integration.",
            default: true,
            risk: RiskLevel::Safe,
        },
    ]
}
