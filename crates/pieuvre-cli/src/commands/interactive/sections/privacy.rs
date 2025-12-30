use crate::commands::interactive::types::{OptItem, RiskLevel};

pub fn get_options() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "telemetry_level",
            label: "Set Telemetry to Security (Level 0)",
            description: "Forces the telemetry level to 'Security' (Enterprise/Education only) or 'Basic' via Group Policy.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "advertising_id",
            label: "Disable Advertising ID",
            description: "Prevents apps from using the advertising ID for experiences across apps.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "location",
            label: "Disable Location Tracking",
            description: "Globally disables location services and clears location history.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "activity_history",
            label: "Disable Activity History",
            description: "Prevents Windows from collecting activities and syncing them to the cloud.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "cortana",
            label: "Disable Cortana & Search Suggestions",
            description: "Completely disables Cortana and prevents web results from appearing in Windows Search.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "recall",
            label: "Disable Windows Recall (AI)",
            description: "Blocks the Windows Recall feature from taking snapshots and analyzing user activity.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "context_menu",
            label: "Classic Context Menu (Win11)",
            description: "Restores the classic Windows 10 context menu and removes clutter.",
            default: true,
            risk: RiskLevel::Safe,
        },
    ]
}
