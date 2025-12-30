use crate::commands::interactive::types::{OptItem, RiskLevel};

pub fn get_options() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "diagtrack",
            label: "Disable Telemetry Service (DiagTrack)",
            description: "Disables the 'Connected User Experiences and Telemetry' service. This is the primary telemetry collector in Windows.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "dmwappush",
            label: "Disable WAP Push Service",
            description: "Disables the 'dmwappushservice' used for routing telemetry data to Microsoft servers.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "wersvc",
            label: "Disable Error Reporting Service",
            description: "Disables Windows Error Reporting (WER). Prevents the system from sending crash dumps and error logs to Microsoft.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "firewall",
            label: "Block Telemetry via Firewall (Native)",
            description: "Creates outbound block rules for known Microsoft telemetry endpoints using the native Windows Firewall API.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "sched_tasks",
            label: "Disable Telemetry Scheduled Tasks",
            description: "Disables over 30+ scheduled tasks related to data collection, CEIP, and SQM.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "hosts",
            label: "Block Telemetry via Hosts File",
            description: "Appends 500+ known telemetry and tracking domains to the system hosts file.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "onedrive",
            label: "Uninstall Microsoft OneDrive",
            description: "Completely removes OneDrive from the system and cleans up residual files and registry keys.",
            default: false,
            risk: RiskLevel::Safe,
        },
    ]
}
