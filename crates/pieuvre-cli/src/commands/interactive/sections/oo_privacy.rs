use crate::commands::interactive::types::{OptItem, RiskLevel};

pub fn get_options() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "oo_telemetry",
            label: "O&O: Disable All Telemetry (SOTA)",
            description:
                "Applies all O&O recommended telemetry blocks, including CEIP and SQM client.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "oo_advertising",
            label: "O&O: Disable Advertising ID",
            description: "Disables the advertising ID for all users (HKLM + HKU).",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "oo_copilot",
            label: "O&O: Disable Windows Copilot",
            description: "Globally disables Copilot integration and background services.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "oo_recall",
            label: "O&O: Block Windows Recall",
            description: "Prevents AI data analysis and snapshot saving (Recall).",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "oo_widgets",
            label: "O&O: Disable Widgets & News",
            description: "Removes Widgets from taskbar and disables the Dsh service.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "oo_search_highlights",
            label: "O&O: Disable Search Highlights",
            description: "Removes web-based suggestions and highlights from Windows Search.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "oo_wudo",
            label: "O&O: Disable WUDO (Delivery Optimization)",
            description: "Forces Windows Update to use local HTTP only.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "oo_wifi_sense",
            label: "O&O: Disable Wi-Fi Sense",
            description: "Prevents automatic connection to open hotspots.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "oo_app_permissions",
            label: "O&O: Lock App Permissions (Cam/Mic/Loc)",
            description: "Globally denies access to Camera, Microphone, and Location.",
            default: false,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "oo_bg_apps",
            label: "O&O: Disable Background Apps",
            description: "Prevents non-essential apps from running in the background.",
            default: true,
            risk: RiskLevel::Safe,
        },
    ]
}
