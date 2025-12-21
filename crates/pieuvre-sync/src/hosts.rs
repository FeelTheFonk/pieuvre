//! Hosts File Management
//!
//! Block telemetry domains via Windows hosts file.

use pieuvre_common::{PieuvreError, Result};
use std::fs;
use std::path::Path;

const HOSTS_PATH: &str = r"C:\Windows\System32\drivers\etc\hosts";
const PIEUVRE_MARKER_START: &str = "# === PIEUVRE TELEMETRY BLOCK START ===";
const PIEUVRE_MARKER_END: &str = "# === PIEUVRE TELEMETRY BLOCK END ===";

/// Telemetry domains to block via hosts (SOTA - privacy.sexy reference)
const TELEMETRY_HOSTS: &[&str] = &[
    // Core telemetry
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
    // SmartScreen
    "smartscreen.microsoft.com",
    "smartscreen-prod.microsoft.com",
    // Spotlight / Ads
    "arc.msn.com",
    "ris.api.iris.microsoft.com",
    "g.live.com",
    "c.msn.com",
    "ntp.msn.com",
    // Copilot
    "copilot.microsoft.com",
    "sydney.bing.com",
    // Bing
    "bing.com",
    "www.bing.com",
    "login.live.com",
    // Edge telemetry
    "edge.microsoft.com",
    "config.edge.skype.com",
    // Office telemetry
    "nexus.officeapps.live.com",
    "nexusrules.officeapps.live.com",
    "c.bing.com",
    // Feedback
    "feedback.microsoft.com",
    "feedback.search.microsoft.com",
    // OneDrive ads
    "g.msn.com",
    "query.prod.cms.rt.microsoft.com",
    // Cortana
    "fp.msedge.net",
    "fp.msedge.win",
    "I-ring.msedge.net",
    // App telemetry
    "clientconfig.passport.net",
    "v10.vortex-win.data.microsoft.com",
    "cy2.vortex.data.microsoft.com",
];

/// Add telemetry block entries to hosts file
pub fn add_telemetry_blocks() -> Result<u32> {
    let hosts_content = fs::read_to_string(HOSTS_PATH)
        .map_err(|e| PieuvreError::Io(e))?;
    
    // Check if already added
    if hosts_content.contains(PIEUVRE_MARKER_START) {
        tracing::info!("Hosts block already exists");
        return Ok(0);
    }
    
    // Build block section
    let mut block = String::new();
    block.push('\n');
    block.push_str(PIEUVRE_MARKER_START);
    block.push('\n');
    
    for domain in TELEMETRY_HOSTS {
        block.push_str(&format!("0.0.0.0 {}\n", domain));
        block.push_str(&format!("0.0.0.0 www.{}\n", domain));
    }
    
    block.push_str(PIEUVRE_MARKER_END);
    block.push('\n');
    
    // Append to hosts
    let new_content = format!("{}{}", hosts_content, block);
    fs::write(HOSTS_PATH, new_content)
        .map_err(|e| PieuvreError::Io(e))?;
    
    tracing::info!("Added {} domains to hosts file", TELEMETRY_HOSTS.len());
    Ok(TELEMETRY_HOSTS.len() as u32)
}

/// Remove Pieuvre entries from hosts file
pub fn remove_telemetry_blocks() -> Result<()> {
    let hosts_content = fs::read_to_string(HOSTS_PATH)
        .map_err(|e| PieuvreError::Io(e))?;
    
    if !hosts_content.contains(PIEUVRE_MARKER_START) {
        tracing::info!("No Pieuvre hosts block found");
        return Ok(());
    }
    
    // Find and remove block
    let start = hosts_content.find(PIEUVRE_MARKER_START);
    let end = hosts_content.find(PIEUVRE_MARKER_END);
    
    if let (Some(s), Some(e)) = (start, end) {
        let before = &hosts_content[..s];
        let after = &hosts_content[e + PIEUVRE_MARKER_END.len()..];
        let new_content = format!("{}{}", before.trim_end(), after);
        
        fs::write(HOSTS_PATH, new_content)
            .map_err(|e| PieuvreError::Io(e))?;
        
        tracing::info!("Removed Pieuvre hosts block");
    }
    
    Ok(())
}

/// Check if hosts blocking is active
pub fn is_hosts_blocking_active() -> bool {
    if let Ok(content) = fs::read_to_string(HOSTS_PATH) {
        content.contains(PIEUVRE_MARKER_START)
    } else {
        false
    }
}

/// Get count of blocked domains
pub fn get_blocked_domains_count() -> usize {
    TELEMETRY_HOSTS.len()
}
