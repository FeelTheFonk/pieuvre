//! Status command
//!
//! Real-time dashboard displaying the state of the 9 optimization sections.

use anyhow::Result;
use console::style;
use pieuvre_sync::timer;

pub fn run(live: bool) -> Result<()> {
    if live {
        let term = console::Term::stdout();
        loop {
            term.clear_screen()?;
            render_dashboard()?;
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    } else {
        render_dashboard()
    }
}

fn render_dashboard() -> Result<()> {
    println!(
        "{}",
        style("┌──────────────────────────────────────────────────────────────────┐").cyan()
    );
    println!(
        "{}",
        style("│                 PIEUVRE - SYSTEM DASHBOARD                       │")
            .cyan()
            .bold()
    );
    println!(
        "{}",
        style("└──────────────────────────────────────────────────────────────────┘").cyan()
    );
    println!();

    // 1. HARDWARE & KERNEL
    println!("  {}", style("KERNEL & LATENCY").bold().underlined());
    match timer::get_timer_resolution() {
        Ok(info) => {
            let res = info.current_ms();
            let color = if res <= 0.55 {
                "green"
            } else if res <= 1.0 {
                "yellow"
            } else {
                "red"
            };
            println!(
                "    Timer Resolution:  {}",
                style(format!("{:.4}ms", res)).color256(match color {
                    "green" => 10,
                    "yellow" => 11,
                    _ => 9,
                })
            );
        }
        Err(_) => println!("    Timer Resolution:  {}", style("Unknown").red()),
    }

    let msi_status = if pieuvre_sync::msi::is_msi_enabled_on_gpu() {
        style("ACTIVE").green()
    } else {
        style("OFF").dim()
    };
    println!("    GPU MSI Mode:      {}", msi_status);

    match pieuvre_audit::registry::read_dword_value(
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile",
        "SystemResponsiveness",
    ) {
        Ok(v) => println!("    Responsiveness:    {}% (Gaming)", style(v).green()),
        Err(_) => println!("    Responsiveness:    20% (Default)"),
    }
    println!();

    // 2. TELEMETRY & PRIVACY
    println!("  {}", style("PRIVACY & TELEMETRY").bold().underlined());
    match pieuvre_audit::registry::get_telemetry_status() {
        Ok(status) => {
            let diag = if status.diagtrack_enabled {
                style("ACTIVE").red()
            } else {
                style("OFF").green()
            };
            println!("    DiagTrack Svc:     {}", diag);
            println!(
                "    Telemetry Level:   {}",
                style(status.data_collection_level).yellow()
            );
            let adv = if status.advertising_id_enabled {
                style("ACTIVE").red()
            } else {
                style("OFF").green()
            };
            println!("    Advertising ID:    {}", adv);
        }
        Err(_) => println!("    Telemetry:         {}", style("Read error").red()),
    }
    let hosts_active = if pieuvre_sync::hosts::is_hosts_blocking_active() {
        style("ACTIVE").green()
    } else {
        style("OFF").dim()
    };
    println!("    Hosts Blocking:    {}", hosts_active);
    println!();

    // 3. SECURITY
    println!("  {}", style("SECURITY HARDENING").bold().underlined());
    let defender = pieuvre_audit::registry::get_defender_status().ok();
    if let Some(d) = defender {
        let tp = if d.tamper_protection {
            style("ON").green()
        } else {
            style("OFF").red()
        };
        println!("    Tamper Protection: {}", tp);
        let rt = if d.realtime_protection {
            style("ON").green()
        } else {
            style("OFF").red()
        };
        println!("    Real-time Guard:   {}", rt);
    }
    let uac = pieuvre_audit::registry::get_uac_status().ok();
    if let Some(u) = uac {
        let uac_st = if u.enabled {
            style("ON").green()
        } else {
            style("OFF").red()
        };
        println!("    UAC Status:        {}", uac_st);
    }
    println!();

    // 4. NETWORK & SYNC
    println!("  {}", style("NETWORK & SYNC").bold().underlined());
    match pieuvre_sync::firewall::list_pieuvre_rules() {
        Ok(rules) => println!("    pieuvre Rules:     {}", style(rules.len()).green()),
        Err(_) => println!("    pieuvre Rules:     0"),
    }
    match pieuvre_sync::power::get_active_power_plan() {
        Ok(plan) => println!("    Power Profile:     {}", style(plan).cyan()),
        Err(_) => println!("    Power Profile:     Unknown"),
    }
    println!();

    // 5. TOP OFFENDERS (LATENCY ETW)
    println!(
        "  {}",
        style("LATENCY TOP OFFENDERS (LIVE ETW)")
            .bold()
            .underlined()
    );
    let stats = pieuvre_audit::etw::monitor::LatencyMonitor::global().get_all_stats();
    let mut stats_vec: Vec<_> = stats.into_iter().collect();
    stats_vec.sort_by(|a, b| b.1.dpc_max_us.cmp(&a.1.dpc_max_us));

    if stats_vec.is_empty() {
        println!("    {}", style("No ETW data (run an audit or wait)").dim());
    } else {
        for (driver, stat) in stats_vec.iter().take(5) {
            let color = if stat.dpc_max_us > 500 {
                "red"
            } else if stat.dpc_max_us > 100 {
                "yellow"
            } else {
                "green"
            };
            println!(
                "    {:18} DPC Max: {}us | Count: {}",
                style(driver).cyan(),
                style(format!("{:>4}", stat.dpc_max_us)).color256(match color {
                    "red" => 9,
                    "yellow" => 11,
                    _ => 10,
                }),
                style(stat.dpc_count).dim()
            );
        }
    }
    println!();

    // 6. SYSTEM INTEGRITY
    println!("  {}", style("SYSTEM INTEGRITY").bold().underlined());
    match pieuvre_persist::list_snapshots() {
        Ok(snapshots) => {
            println!("    Snapshots:         {}", style(snapshots.len()).green());
            if let Some(last) = snapshots.first() {
                println!(
                    "    Last:              {} ({})",
                    style(last.timestamp.format("%d/%m %H:%M")).dim(),
                    style(&last.description).italic()
                );
            }
        }
        Err(_) => println!("    Snapshots:         0"),
    }

    println!();
    println!(
        "{}",
        style("────────────────────────────────────────────────────────────────────").dim()
    );
    println!(
        "  {} Use {} for a full audit or {} to optimize.",
        style("TIP:").yellow().bold(),
        style("pieuvre audit").cyan(),
        style("pieuvre interactive").cyan()
    );
    println!();

    Ok(())
}
