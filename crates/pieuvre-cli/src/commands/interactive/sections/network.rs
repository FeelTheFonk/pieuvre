use crate::commands::interactive::types::{OptItem, RiskLevel};

pub fn get_options() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "net_doh",
            label: "Enable DNS-over-HTTPS",
            description: "Configures Cloudflare DoH for encrypted and private DNS queries.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "net_firewall",
            label: "Apply Firewall Hardening",
            description: "Blocks known telemetry and tracking IP ranges in Windows Firewall.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "net_hosts",
            label: "Apply Hosts Blocklist",
            description: "Adds telemetry domains to the system hosts file.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "net_ipv6",
            label: "Disable IPv6",
            description: "Disables IPv6 if not needed, which can solve some connection issues.",
            default: false,
            risk: RiskLevel::Low,
        },
    ]
}
