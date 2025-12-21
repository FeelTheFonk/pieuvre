//! Network Inspector
//!
//! Détection des connexions et endpoints télémétrie.

use pieuvre_common::Result;
use std::net::ToSocketAddrs;

/// Endpoints Microsoft connus pour la télémétrie
const MS_TELEMETRY_DOMAINS: &[&str] = &[
    "vortex.data.microsoft.com",
    "vortex-win.data.microsoft.com",
    "telecommand.telemetry.microsoft.com",
    "telemetry.microsoft.com",
    "watson.telemetry.microsoft.com",
    "watson.microsoft.com",
    "settings-win.data.microsoft.com",
    "settings.data.microsoft.com",
    "self.events.data.microsoft.com",
    "v10.events.data.microsoft.com",
    "v20.events.data.microsoft.com",
    "browser.events.data.msn.com",
    "arc.msn.com",
    "ris.api.iris.microsoft.com",
    "diagnostic.data.microsoft.com",
    "activity.windows.com",
    "data.microsoft.com",
    "dmd.metaservices.microsoft.com",
    "nw-umwatson.events.data.microsoft.com",
];

/// Plages IP Microsoft télémétrie
const MS_TELEMETRY_IP_RANGES: &[&str] = &[
    "13.64.0.0/11",
    "13.96.0.0/13",
    "13.104.0.0/14",
    "20.33.0.0/16",
    "20.40.0.0/13",
    "20.128.0.0/16",
    "40.74.0.0/15",
    "40.76.0.0/14",
    "40.96.0.0/12",
    "52.96.0.0/12",
    "52.112.0.0/14",
    "104.40.0.0/13",
];

#[derive(Debug, Clone)]
pub struct TelemetryEndpoint {
    pub domain: String,
    pub resolved_ips: Vec<String>,
    pub is_blocked: bool,
}

#[derive(Debug, Clone)]
pub struct NetworkStatus {
    pub telemetry_endpoints: Vec<TelemetryEndpoint>,
    pub blocked_count: usize,
    pub reachable_count: usize,
}

/// Analyse le statut réseau télémétrie
pub fn inspect_network() -> Result<NetworkStatus> {
    let mut endpoints = Vec::new();
    let mut blocked = 0;
    let mut reachable = 0;
    
    for domain in MS_TELEMETRY_DOMAINS {
        let resolved_ips = resolve_domain(domain);
        let is_blocked = resolved_ips.is_empty() || 
                         resolved_ips.iter().any(|ip| ip.starts_with("0.0.0.0") || ip.starts_with("127.0.0.1"));
        
        if is_blocked {
            blocked += 1;
        } else {
            reachable += 1;
        }
        
        endpoints.push(TelemetryEndpoint {
            domain: domain.to_string(),
            resolved_ips,
            is_blocked,
        });
    }
    
    Ok(NetworkStatus {
        telemetry_endpoints: endpoints,
        blocked_count: blocked,
        reachable_count: reachable,
    })
}

fn resolve_domain(domain: &str) -> Vec<String> {
    let addr = format!("{}:443", domain);
    match addr.to_socket_addrs() {
        Ok(addrs) => addrs.map(|a| a.ip().to_string()).collect(),
        Err(_) => Vec::new(),
    }
}

/// Vérifie si un domaine est dans la liste télémétrie
pub fn is_telemetry_domain(domain: &str) -> bool {
    let lower = domain.to_lowercase();
    MS_TELEMETRY_DOMAINS.iter().any(|d| lower.contains(&d.to_lowercase()))
}

/// Retourne la liste des domaines télémétrie pour blocage
pub fn get_telemetry_domains() -> &'static [&'static str] {
    MS_TELEMETRY_DOMAINS
}

/// Retourne les plages IP télémétrie
pub fn get_telemetry_ip_ranges() -> &'static [&'static str] {
    MS_TELEMETRY_IP_RANGES
}
