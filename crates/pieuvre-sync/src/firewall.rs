//! Firewall Rules
//!
//! Création de règles Windows Firewall pour bloquer la télémétrie.
//! Utilise netsh pour éviter les dépendances COM complexes.

use pieuvre_common::{PieuvreError, Result};
use std::process::Command;

/// Domaines télémétrie Microsoft à bloquer (SOTA)
const TELEMETRY_DOMAINS: &[&str] = &[
    // Telemetry core
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
    "diagnostic.data.microsoft.com",
    "activity.windows.com",
    // Events
    "umwatson.events.data.microsoft.com",
    "ceuswatcab01.blob.core.windows.net",
    "ceuswatcab02.blob.core.windows.net",
    // NCSI / connectivity check
    "dns.msftncsi.com",
    "www.msftconnecttest.com",
    // SmartScreen
    "smartscreen.microsoft.com",
    "smartscreen-prod.microsoft.com",
    // Spotlight / Ads
    "arc.msn.com",
    "ris.api.iris.microsoft.com",
    "g.live.com",
    "c.msn.com",
    "c.microsoft.com",
    "ntp.msn.com",
    // Copilot / AI
    "copilot.microsoft.com",
    "sydney.bing.com",
];

/// Plages IP Microsoft télémétrie (Azure + M365 ranges)
const TELEMETRY_IP_RANGES: &[&str] = &[
    // Azure telemetry endpoints
    "13.64.0.0/11",
    "13.96.0.0/13",
    "20.33.0.0/16",
    "20.40.0.0/13",
    "20.128.0.0/16",
    "23.96.0.0/13",
    "40.64.0.0/10",
    "40.76.0.0/14",
    "51.104.0.0/15",
    "52.96.0.0/12",
    "52.112.0.0/14",
    "104.40.0.0/13",
    "104.208.0.0/13",
    "131.253.0.0/16",
    "134.170.0.0/16",
    "157.55.0.0/16",
    "204.79.195.0/24",
];

#[derive(Debug, Clone)]
pub struct FirewallRule {
    pub name: String,
    pub description: String,
    pub remote_addresses: Vec<String>,
    pub enabled: bool,
}

/// Crée les règles firewall pour bloquer la télémétrie via netsh
pub fn create_telemetry_block_rules() -> Result<Vec<String>> {
    let mut created_rules = Vec::new();
    
    // Règle principale pour bloquer les IPs Microsoft télémétrie
    let rule_name = "Pieuvre-BlockTelemetry";
    let ip_list = TELEMETRY_IP_RANGES.join(",");
    
    let output = Command::new("netsh")
        .args([
            "advfirewall", "firewall", "add", "rule",
            &format!("name={}", rule_name),
            "dir=out",
            "action=block",
            &format!("remoteip={}", ip_list),
            "enable=yes",
        ])
        .output()
        .map_err(PieuvreError::Io)?;
    
    if output.status.success() {
        created_rules.push(rule_name.to_string());
        tracing::info!("Règle firewall créée: {}", rule_name);
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        tracing::warn!("Échec création règle firewall: {}", error);
    }
    
    Ok(created_rules)
}

/// Supprime les règles firewall Pieuvre
pub fn remove_pieuvre_rules() -> Result<u32> {
    let mut removed = 0u32;
    
    let rule_names = ["Pieuvre-BlockTelemetry", "Pieuvre-BlockTelemetryDomains"];
    
    for name in rule_names {
        let output = Command::new("netsh")
            .args([
                "advfirewall", "firewall", "delete", "rule",
                &format!("name={}", name),
            ])
            .output()
            .map_err(PieuvreError::Io)?;
        
        if output.status.success() {
            removed += 1;
            tracing::info!("Règle supprimée: {}", name);
        }
    }
    
    Ok(removed)
}

/// Liste les règles firewall Pieuvre existantes
pub fn list_pieuvre_rules() -> Result<Vec<FirewallRule>> {
    let mut rules = Vec::new();
    
    let output = Command::new("netsh")
        .args([
            "advfirewall", "firewall", "show", "rule",
            "name=Pieuvre-BlockTelemetry",
        ])
        .output()
        .map_err(PieuvreError::Io)?;
    
    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        if output_str.contains("Pieuvre") {
            rules.push(FirewallRule {
                name: "Pieuvre-BlockTelemetry".to_string(),
                description: "Bloque IPs télémétrie Microsoft".to_string(),
                remote_addresses: TELEMETRY_IP_RANGES.iter().map(|s| s.to_string()).collect(),
                enabled: true,
            });
        }
    }
    
    Ok(rules)
}

/// Retourne les domaines télémétrie pour blocage hosts
pub fn get_telemetry_domains() -> &'static [&'static str] {
    TELEMETRY_DOMAINS
}

/// Retourne les IPs télémétrie
pub fn get_telemetry_ip_ranges() -> &'static [&'static str] {
    TELEMETRY_IP_RANGES
}
