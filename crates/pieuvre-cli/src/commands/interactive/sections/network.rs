use crate::commands::interactive::types::{OptItem, RiskLevel};

pub fn get_options() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "net_doh",
            label: "Activer le DNS-over-HTTPS",
            description: "Configure le DoH Cloudflare pour des requêtes DNS chiffrées et privées.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "net_firewall",
            label: "Appliquer le durcissement du pare-feu",
            description: "Bloque les plages IP de télémétrie et de suivi connues dans le pare-feu Windows.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "net_hosts",
            label: "Appliquer la liste de blocage Hosts",
            description: "Ajoute les domaines de télémétrie au fichier hosts du système.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "net_ipv6",
            label: "Désactiver l'IPv6",
            description: "Désactive l'IPv6 si non nécessaire, ce qui peut résoudre certains problèmes de connexion.",
            default: false,
            risk: RiskLevel::Low,
        },
    ]
}
