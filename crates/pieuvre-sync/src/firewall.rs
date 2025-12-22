//! Firewall Rules
//!
//! Création de règles Windows Firewall pour bloquer la télémétrie.
//! Utilise l'interface COM INetFwPolicy2 pour une gestion native SOTA.

use pieuvre_common::{PieuvreError, Result};
use windows::core::BSTR;
use windows::Win32::Foundation::VARIANT_BOOL;
use windows::Win32::NetworkManagement::WindowsFirewall::{
    INetFwPolicy2, INetFwRule, INetFwRules, NetFwPolicy2, NetFwRule, NET_FW_ACTION_BLOCK,
    NET_FW_RULE_DIR_OUT,
};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED,
};

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

/// Crée les règles firewall pour bloquer la télémétrie via API COM (SOTA Native)
pub fn create_telemetry_block_rules() -> Result<Vec<String>> {
    unsafe {
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

        let policy: INetFwPolicy2 = CoCreateInstance(&NetFwPolicy2, None, CLSCTX_ALL)
            .map_err(|e| PieuvreError::Internal(format!("Failed to create NetFwPolicy2: {}", e)))?;

        let rules: INetFwRules = policy
            .Rules()
            .map_err(|e| PieuvreError::Internal(format!("Failed to get FW rules: {}", e)))?;

        let rule_name = "pieuvre-BlockTelemetry";
        let ip_list = TELEMETRY_IP_RANGES.join(",");

        let rule: INetFwRule = CoCreateInstance(&NetFwRule, None, CLSCTX_ALL)
            .map_err(|e| PieuvreError::Internal(format!("Failed to create NetFwRule: {}", e)))?;

        rule.SetName(&BSTR::from(rule_name))
            .map_err(|e| PieuvreError::Internal(e.to_string()))?;
        rule.SetDescription(&BSTR::from(
            "Bloque les IPs de télémétrie Microsoft (pieuvre SOTA)",
        ))
        .map_err(|e| PieuvreError::Internal(e.to_string()))?;
        rule.SetDirection(NET_FW_RULE_DIR_OUT)
            .map_err(|e| PieuvreError::Internal(e.to_string()))?;
        rule.SetAction(NET_FW_ACTION_BLOCK)
            .map_err(|e| PieuvreError::Internal(e.to_string()))?;
        rule.SetRemoteAddresses(&BSTR::from(ip_list))
            .map_err(|e| PieuvreError::Internal(e.to_string()))?;
        rule.SetEnabled(VARIANT_BOOL::from(true))
            .map_err(|e| PieuvreError::Internal(e.to_string()))?;

        rules
            .Add(&rule)
            .map_err(|e| PieuvreError::Internal(format!("Failed to add rule: {}", e)))?;

        tracing::info!("Règle firewall créée via COM: {}", rule_name);
        Ok(vec![rule_name.to_string()])
    }
}

/// Supprime les règles firewall pieuvre via API COM
pub fn remove_pieuvre_rules() -> Result<u32> {
    unsafe {
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

        let policy: INetFwPolicy2 = CoCreateInstance(&NetFwPolicy2, None, CLSCTX_ALL)
            .map_err(|e| PieuvreError::Internal(format!("Failed to create NetFwPolicy2: {}", e)))?;

        let rules: INetFwRules = policy
            .Rules()
            .map_err(|e| PieuvreError::Internal(format!("Failed to get FW rules: {}", e)))?;

        let mut removed = 0u32;
        let rule_names = ["pieuvre-BlockTelemetry", "pieuvre-BlockTelemetryDomains"];

        for name in rule_names {
            if rules.Remove(&BSTR::from(name)).is_ok() {
                removed += 1;
                tracing::info!("Règle supprimée via COM: {}", name);
            }
        }

        Ok(removed)
    }
}

/// Liste les règles firewall pieuvre existantes via API COM
pub fn list_pieuvre_rules() -> Result<Vec<FirewallRule>> {
    unsafe {
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

        let policy: INetFwPolicy2 = CoCreateInstance(&NetFwPolicy2, None, CLSCTX_ALL)
            .map_err(|e| PieuvreError::Internal(format!("Failed to create NetFwPolicy2: {}", e)))?;

        let rules: INetFwRules = policy
            .Rules()
            .map_err(|e| PieuvreError::Internal(format!("Failed to get FW rules: {}", e)))?;

        let mut result = Vec::new();

        // COM Enumeration is complex in Rust, we check by name for now as it's our primary use case
        let rule_name = "pieuvre-BlockTelemetry";
        if let Ok(rule) = rules.Item(&BSTR::from(rule_name)) {
            result.push(FirewallRule {
                name: rule_name.to_string(),
                description: rule
                    .Description()
                    .map(|b| b.to_string())
                    .unwrap_or_default(),
                remote_addresses: TELEMETRY_IP_RANGES.iter().map(|s| s.to_string()).collect(),
                enabled: rule.Enabled().map(|v| v.as_bool()).unwrap_or(false),
            });
        }

        Ok(result)
    }
}

/// Retourne les domaines télémétrie pour blocage hosts
pub fn get_telemetry_domains() -> &'static [&'static str] {
    TELEMETRY_DOMAINS
}

/// Retourne les IPs télémétrie
pub fn get_telemetry_ip_ranges() -> &'static [&'static str] {
    TELEMETRY_IP_RANGES
}
