use crate::commands::interactive::types::{OptItem, RiskLevel};

pub fn get_options() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "cleanup_temp",
            label: "Cleanup Temporary Files",
            description: "Deletes temporary files from Windows and User temp directories.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "cleanup_winsxs",
            label: "Cleanup WinSxS (Windows Update)",
            description: "Runs DISM component cleanup to reduce system disk usage.",
            default: false,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "explorer_optimize",
            label: "Optimize Explorer Settings",
            description: "Shows file extensions, hidden files, and disables recent items.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "hardening_unlock",
            label: "Unlock Critical Registry Keys",
            description:
                "Restores default permissions to critical registry keys (use before uninstalling).",
            default: false,
            risk: RiskLevel::Warning,
        },
        OptItem {
            id: "windows_update",
            label: "Configure Windows Update (Manual)",
            description: "Sets Windows Update to manual mode to prevent unexpected reboots.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "cleanup_edge",
            label: "Cleanup Edge Cache",
            description: "Deletes Microsoft Edge browser cache and temporary data.",
            default: true,
            risk: RiskLevel::Safe,
        },
    ]
}
