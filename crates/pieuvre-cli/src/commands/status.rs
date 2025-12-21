//! Commande status
//!
//! Affiche l'etat actuel du systeme et des optimisations.

use anyhow::Result;
use pieuvre_sync::timer;

pub fn run() -> Result<()> {
    println!();
    println!("================================================================");
    println!("                 PIEUVRE - Status Systeme");
    println!("================================================================");
    println!();
    
    // Timer resolution
    println!("----------------------------------------------------------------");
    println!("  TIMER RESOLUTION");
    println!("----------------------------------------------------------------");
    match timer::get_timer_resolution() {
        Ok(info) => {
            let status = if info.current_ms() <= 0.55 { "[OK]" } else if info.current_ms() <= 1.0 { "[WARN]" } else { "[HIGH]" };
            println!("  Actuelle:      {:.2}ms {}", info.current_ms(), status);
            println!("  Minimum:       {:.2}ms (coarsest)", info.min_ms());
            println!("  Maximum:       {:.2}ms (finest)", info.max_ms());
        }
        Err(e) => {
            println!("  Erreur: {}", e);
        }
    }
    
    // Telemetrie
    println!();
    println!("----------------------------------------------------------------");
    println!("  TELEMETRIE");
    println!("----------------------------------------------------------------");
    match pieuvre_audit::registry::get_telemetry_status() {
        Ok(status) => {
            let diag = if status.diagtrack_enabled { "[ACTIF]" } else { "[OFF]" };
            println!("  DiagTrack:      {}", diag);
            println!("  Data Level:     {} ({})", status.data_collection_level, 
                match status.data_collection_level {
                    0 => "Security",
                    1 => "Basic",
                    2 => "Enhanced",
                    _ => "Full",
                });
            let adv = if status.advertising_id_enabled { "[ACTIF]" } else { "[OFF]" };
            println!("  Advertising:    {}", adv);
        }
        Err(_) => {
            println!("  Erreur lecture registre");
        }
    }
    
    // Power Plan
    println!();
    println!("----------------------------------------------------------------");
    println!("  POWER PLAN");
    println!("----------------------------------------------------------------");
    match pieuvre_sync::power::get_active_power_plan() {
        Ok(plan) => {
            println!("  Plan actif:     {}", plan);
        }
        Err(_) => {
            println!("  Plan actif:     Inconnu");
        }
    }
    
    // Services
    println!();
    println!("----------------------------------------------------------------");
    println!("  SERVICES TELEMETRIE");
    println!("----------------------------------------------------------------");
    let services = ["DiagTrack", "dmwappushservice", "WerSvc", "SysMain", "WSearch"];
    for svc in services {
        match pieuvre_sync::services::get_service_start_type(svc) {
            Ok(st) => {
                let status = match st {
                    2 => "Automatic",
                    3 => "Manual",
                    4 => "Disabled",
                    _ => "Unknown",
                };
                println!("  {:16} {}", svc, status);
            }
            Err(_) => {
                println!("  {:16} Not found", svc);
            }
        }
    }
    
    // MMCSS
    println!();
    println!("----------------------------------------------------------------");
    println!("  MMCSS GAMING");
    println!("----------------------------------------------------------------");
    match pieuvre_audit::registry::read_dword_value(
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile",
        "SystemResponsiveness"
    ) {
        Ok(v) => println!("  SystemResponsiveness: {}%", v),
        Err(_) => println!("  SystemResponsiveness: Default (20%)"),
    }
    match pieuvre_audit::registry::read_dword_value(
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile",
        "NetworkThrottlingIndex"
    ) {
        Ok(v) => {
            let status = if v == 0xFFFFFFFF { "OFF" } else { "ON" };
            println!("  NetworkThrottling:    {}", status);
        }
        Err(_) => println!("  NetworkThrottling:    Default (ON)"),
    }
    
    // MSI Mode
    println!();
    println!("----------------------------------------------------------------");
    println!("  MSI MODE");
    println!("----------------------------------------------------------------");
    let msi_status = if pieuvre_sync::msi::is_msi_enabled_on_gpu() { "[OK]" } else { "[OFF]" };
    println!("  GPU MSI Mode:   {}", msi_status);
    
    // Firewall
    println!();
    println!("----------------------------------------------------------------");
    println!("  FIREWALL PIEUVRE");
    println!("----------------------------------------------------------------");
    match pieuvre_sync::firewall::list_pieuvre_rules() {
        Ok(rules) => {
            println!("  Regles actives: {}", rules.len());
        }
        Err(_) => {
            println!("  Regles actives: 0");
        }
    }
    
    // Hosts
    println!();
    println!("----------------------------------------------------------------");
    println!("  HOSTS BLOCKING");
    println!("----------------------------------------------------------------");
    let hosts_active = if pieuvre_sync::hosts::is_hosts_blocking_active() { "[ACTIVE]" } else { "[OFF]" };
    println!("  Status:         {}", hosts_active);
    
    // Snapshots
    println!();
    println!("----------------------------------------------------------------");
    println!("  SNAPSHOTS");
    println!("----------------------------------------------------------------");
    match pieuvre_persist::list_snapshots() {
        Ok(snapshots) => {
            println!("  Disponibles:    {}", snapshots.len());
            if !snapshots.is_empty() {
                println!("  Dernier:        {} - {}", 
                    snapshots[0].timestamp.format("%Y-%m-%d %H:%M"),
                    snapshots[0].description);
            }
        }
        Err(_) => {
            println!("  Disponibles:    0");
        }
    }
    
    println!();
    println!("================================================================");
    println!("  Commandes: audit, analyze, interactive, verify, rollback");
    println!("================================================================");
    println!();
    
    Ok(())
}
