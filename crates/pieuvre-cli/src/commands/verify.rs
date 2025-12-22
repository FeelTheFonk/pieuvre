//! Commande verify
//!
//! Verification de l'integrite des optimisations appliquees.

use anyhow::Result;

pub fn run(repair: bool) -> Result<()> {
    println!();
    println!("================================================================");
    println!("           PIEUVRE - Verification Integrite");
    println!("================================================================");
    println!();

    let mut issues = Vec::new();
    let mut ok_count = 0;

    // Verification Timer
    print!("[*] Timer Resolution... ");
    match pieuvre_sync::timer::get_timer_resolution() {
        Ok(info) => {
            if info.current_ms() <= 0.55 {
                println!("OK ({:.2}ms)", info.current_ms());
                ok_count += 1;
            } else {
                println!("WARN ({:.2}ms > 0.5ms)", info.current_ms());
                issues.push((
                    "Timer",
                    format!("Resolution {:.2}ms, attendu 0.5ms", info.current_ms()),
                ));
            }
        }
        Err(e) => {
            println!("ERREUR: {}", e);
            issues.push(("Timer", e.to_string()));
        }
    }

    // Verification Power Plan
    print!("[*] Power Plan... ");
    match pieuvre_sync::power::get_active_power_plan() {
        Ok(plan) => {
            if plan.contains("High") || plan.contains("Ultimate") || plan.contains("Bitsum") {
                println!("OK ({})", plan);
                ok_count += 1;
            } else {
                println!("WARN ({})", plan);
                issues.push((
                    "Power",
                    format!("Plan actif: {}, attendu High/Ultimate", plan),
                ));
            }
        }
        Err(e) => {
            println!("ERREUR: {}", e);
            issues.push(("Power", e.to_string()));
        }
    }

    // Verification Telemetrie DiagTrack
    print!("[*] DiagTrack... ");
    match pieuvre_sync::services::get_service_start_type("DiagTrack") {
        Ok(start_type) => {
            if start_type == 4 {
                println!("OK (Disabled)");
                ok_count += 1;
            } else {
                println!("WARN (start_type={})", start_type);
                issues.push((
                    "DiagTrack",
                    format!("start_type={}, attendu 4 (Disabled)", start_type),
                ));
            }
        }
        Err(_) => {
            println!("OK (non trouve)");
            ok_count += 1;
        }
    }

    // Verification MSI Mode
    print!("[*] MSI Mode GPU... ");
    if pieuvre_sync::msi::is_msi_enabled_on_gpu() {
        println!("OK");
        ok_count += 1;
    } else {
        println!("WARN (non active)");
        issues.push(("MSI", "MSI Mode non active sur GPU".to_string()));
    }

    // Verification Firewall rules
    print!("[*] Firewall rules... ");
    match pieuvre_sync::firewall::list_pieuvre_rules() {
        Ok(rules) => {
            if !rules.is_empty() {
                println!("OK ({} regles)", rules.len());
                ok_count += 1;
            } else {
                println!("WARN (aucune regle)");
                issues.push(("Firewall", "Aucune regle pieuvre active".to_string()));
            }
        }
        Err(e) => {
            println!("ERREUR: {}", e);
            issues.push(("Firewall", e.to_string()));
        }
    }

    // Verification Registry Win32PrioritySeparation
    print!("[*] Scheduler... ");
    match pieuvre_audit::registry::read_dword_value(
        r"SYSTEM\CurrentControlSet\Control\PriorityControl",
        "Win32PrioritySeparation",
    ) {
        Ok(value) => {
            if value == 0x26 {
                println!("OK (0x{:02X})", value);
                ok_count += 1;
            } else {
                println!("WARN (0x{:02X})", value);
                issues.push((
                    "Scheduler",
                    format!("Win32PrioritySeparation=0x{:02X}, attendu 0x26", value),
                ));
            }
        }
        Err(e) => {
            println!("WARN: {}", e);
            issues.push(("Scheduler", e.to_string()));
        }
    }

    // Verification MMCSS SystemResponsiveness
    print!("[*] MMCSS Gaming... ");
    match pieuvre_audit::registry::read_dword_value(
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile",
        "SystemResponsiveness",
    ) {
        Ok(value) => {
            if value == 10 {
                println!("OK ({}%)", value);
                ok_count += 1;
            } else {
                println!("WARN ({}%)", value);
                issues.push((
                    "MMCSS",
                    format!("SystemResponsiveness={}%, attendu 10%", value),
                ));
            }
        }
        Err(_) => {
            println!("WARN (non configure)");
            issues.push(("MMCSS", "SystemResponsiveness non configure".to_string()));
        }
    }

    // Verification Network Throttling
    print!("[*] Network Throttling... ");
    match pieuvre_audit::registry::read_dword_value(
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile",
        "NetworkThrottlingIndex",
    ) {
        Ok(value) => {
            if value == 0xFFFFFFFF {
                println!("OK (OFF)");
                ok_count += 1;
            } else {
                println!("WARN (0x{:08X})", value);
                issues.push((
                    "NetworkThrottle",
                    format!("NetworkThrottlingIndex=0x{:08X}, attendu OFF", value),
                ));
            }
        }
        Err(_) => {
            println!("WARN (non configure)");
            issues.push(("NetworkThrottle", "Non configure".to_string()));
        }
    }

    // Resume
    println!();
    println!("================================================================");
    println!("                      RESUME");
    println!("================================================================");
    println!();
    println!("  Verifications OK: {}", ok_count);
    println!("  Problemes:        {}", issues.len());

    if !issues.is_empty() {
        println!();
        println!("  Problemes detectes:");
        for (name, desc) in &issues {
            println!("    - {}: {}", name, desc);
        }
    }

    if repair && !issues.is_empty() {
        println!();
        println!("[*] Mode reparation active...");
        println!("    Executez 'pieuvre interactive --profile gaming' pour corriger");
    }

    println!();

    if issues.is_empty() {
        println!("[OK] Toutes les verifications passees avec succes.");
    } else {
        println!("[!] {} probleme(s) detecte(s).", issues.len());
    }

    Ok(())
}
