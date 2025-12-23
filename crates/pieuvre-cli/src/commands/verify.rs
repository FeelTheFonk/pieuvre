//! Commande verify
//!
//! Verification de l'integrite des optimisations appliquees.

use anyhow::Result;

pub fn run(repair: bool) -> Result<()> {
    println!();
    println!("================================================================");
    println!("           PIEUVRE - Integrity Verification");
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
                    format!("Resolution {:.2}ms, expected 0.5ms", info.current_ms()),
                ));
            }
        }
        Err(e) => {
            println!("ERROR: {}", e);
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
                    format!("Active plan: {}, expected High/Ultimate", plan),
                ));
            }
        }
        Err(e) => {
            println!("ERROR: {}", e);
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
                    format!("start_type={}, expected 4 (Disabled)", start_type),
                ));
            }
        }
        Err(_) => {
            println!("OK (not found)");
            ok_count += 1;
        }
    }

    // Verification MSI Mode
    print!("[*] MSI Mode GPU... ");
    if pieuvre_sync::msi::is_msi_enabled_on_gpu() {
        println!("OK");
        ok_count += 1;
    } else {
        println!("WARN (not active)");
        issues.push(("MSI", "MSI Mode not active on GPU".to_string()));
    }

    // Verification Firewall rules
    print!("[*] Firewall rules... ");
    match pieuvre_sync::firewall::list_pieuvre_rules() {
        Ok(rules) => {
            if !rules.is_empty() {
                println!("OK ({} regles)", rules.len());
                ok_count += 1;
            } else {
                println!("WARN (no rules)");
                issues.push(("Firewall", "No pieuvre rules active".to_string()));
            }
        }
        Err(e) => {
            println!("ERROR: {}", e);
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
                    format!("Win32PrioritySeparation=0x{:02X}, expected 0x26", value),
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
                    format!("SystemResponsiveness={}%, expected 10%", value),
                ));
            }
        }
        Err(_) => {
            println!("WARN (not configured)");
            issues.push(("MMCSS", "SystemResponsiveness not configured".to_string()));
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
                    format!("NetworkThrottlingIndex=0x{:08X}, expected OFF", value),
                ));
            }
        }
        Err(_) => {
            println!("WARN (not configured)");
            issues.push(("NetworkThrottle", "Not configured".to_string()));
        }
    }

    // Resume
    println!();
    println!("================================================================");
    println!("                      SUMMARY");
    println!("================================================================");
    println!();
    println!("  Verifications OK: {}", ok_count);
    println!("  Issues:           {}", issues.len());

    if !issues.is_empty() {
        println!();
        println!("  Issues detected:");
        for (name, desc) in &issues {
            println!("    - {}: {}", name, desc);
        }
    }

    if repair && !issues.is_empty() {
        println!();
        println!("[*] Repair mode active...");
        println!("    Run 'pieuvre interactive' to fix issues");
    }

    println!();

    if issues.is_empty() {
        println!("[OK] All verifications passed successfully.");
    } else {
        println!("[!] {} issue(s) detected.", issues.len());
    }

    Ok(())
}
