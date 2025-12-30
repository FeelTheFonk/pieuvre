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
            id: "dns_doh",
            label: "Enable DNS-over-HTTPS (Cloudflare)",
            description:
                "Configures system-wide DoH using Cloudflare DNS for better privacy and security.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "dns_flush",
            label: "Flush DNS Cache",
            description: "Clears the local DNS resolver cache.",
            default: true,
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
            id: "hardening_lock",
            label: "Lock Critical Registry Keys",
            description:
                "Applies read-only ACLs to critical privacy and system keys to prevent resets.",
            default: false,
            risk: RiskLevel::Warning,
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
            id: "hardening_ppl",
            label: "Enable PPL Protection",
            description: "Enables Protected Process Light for the pieuvre process.",
            default: false,
            risk: RiskLevel::Safe,
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
