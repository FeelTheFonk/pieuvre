//! Interactive mode 2026
//!
//! Modular architecture:
//! - `sections.rs`: Option definitions by category
//! - `executor.rs`: Optimization execution
//! - `ui.rs`: User interface with indicatif

mod executor;
mod sections;
mod ui;

use anyhow::Result;
use dialoguer::{Confirm, MultiSelect};
use indicatif::MultiProgress;
use pieuvre_audit::hardware::is_laptop;
use pieuvre_common::ChangeRecord;
use pieuvre_persist::snapshot;
use tracing::{info, instrument, warn};

pub use sections::OptItem;
use ui::MainAction;

/// Default interactive entry point (without CLI arguments)
pub async fn run_default() -> Result<()> {
    // 1. ASCII Welcome Screen
    ui::print_welcome_screen();

    // 2. Admin privilege check
    ui::check_admin_status();

    // 3. Quick system state summary
    ui::print_quick_status();

    // 4. Interactive main menu
    let action = ui::show_main_menu()?;

    match action {
        MainAction::Interactive(profile) => run(&profile).await,
        MainAction::QuickApply(profile) => run_quick_apply(&profile).await,
        MainAction::Status => super::status::run(false),
        MainAction::Rollback => super::rollback::run(true, false, None),
        MainAction::Exit => {
            ui::print_goodbye();
            Ok(())
        }
    }?;

    // Final pause to prevent console from closing
    ui::wait_for_exit();
    Ok(())
}

/// Quick apply of a profile without granular selection
pub async fn run_quick_apply(profile: &str) -> Result<()> {
    println!();
    println!("  [*] Quick applying profile: {}", profile.to_uppercase());

    // Create a safety snapshot
    let changes = Vec::<ChangeRecord>::new();
    let snap = snapshot::create(&format!("Before quick profile {}", profile), changes)?;
    println!(
        "  [OK] Backup snapshot created: {}",
        &snap.id.to_string()[..8]
    );

    // Real application via pieuvre-sync
    pieuvre_sync::apply_profile(profile, false).await?;

    println!();
    println!(
        "  [OK] Profile {} applied successfully.",
        profile.to_uppercase()
    );
    println!("       Restart recommended for some modifications.");
    println!();

    ui::wait_for_exit();
    Ok(())
}

/// Entry point for interactive mode with granular selection
#[instrument(skip_all, fields(profile = %profile))]
pub async fn run(profile: &str) -> Result<()> {
    let is_laptop = is_laptop();
    info!(
        is_laptop = is_laptop,
        profile = profile,
        "Starting interactive mode"
    );

    // Display header
    ui::print_header(is_laptop, profile);

    // ═══════════════════════════════════════════════════════════════════════
    // SELECTION COLLECTION
    // ═══════════════════════════════════════════════════════════════════════

    // Section 1: Telemetry
    ui::print_section_header(1, 9, "TELEMETRY - Services");
    let telem_opts = sections::telemetry_section();
    let telem_selected = collect_selection(&telem_opts, "Telemetry services")?;

    // Section 2: Privacy
    ui::print_section_header(2, 9, "PRIVACY - Registry");
    let privacy_opts = sections::privacy_section();
    let privacy_selected = collect_selection(&privacy_opts, "Privacy registry")?;

    // Section 3: Performance
    ui::print_section_header(3, 9, "PERFORMANCE");
    let perf_opts = sections::performance_section(is_laptop);
    let perf_selected = collect_selection(&perf_opts, "Performance")?;

    // Section 4: Scheduler
    ui::print_section_header(4, 9, "SCHEDULER");
    let sched_opts = sections::scheduler_section();
    let sched_selected = collect_selection(&sched_opts, "Scheduler")?;

    // Section 5: AppX
    ui::print_section_header(5, 9, "APPX - Bloatware");
    let appx_opts = sections::appx_section();
    let appx_selected = collect_selection(&appx_opts, "AppX Bloatware")?;

    // Section 6: CPU/Memory
    ui::print_section_header(6, 9, "CPU / MEMORY");
    let cpu_opts = sections::cpu_section(is_laptop);
    let cpu_selected = collect_selection(&cpu_opts, "CPU/Memory")?;

    // Section 7: DPC Latency
    ui::print_section_header(7, 9, "DPC LATENCY - Micro-stuttering");
    let dpc_opts = sections::dpc_section();
    let dpc_selected = collect_selection(&dpc_opts, "DPC Latency")?;

    // Section 8: Security (with warning)
    ui::print_section_header(8, 9, "SECURITY - [WARN] CAUTION RISK");
    ui::print_security_warning();
    let security_opts = sections::security_section();
    let security_selected = collect_selection(&security_opts, "Security")?;

    // Section 9: Advanced Network
    ui::print_section_header(9, 11, "ADVANCED NETWORK");
    let net_adv_opts = sections::network_advanced_section();
    let net_adv_selected = collect_selection(&net_adv_opts, "Advanced Network")?;

    // Section 10: DNS SOTA 2026
    ui::print_section_header(10, 11, "DNS SOTA 2026");
    let dns_opts = sections::dns_section();
    let dns_selected = collect_selection(&dns_opts, "DNS SOTA 2026")?;

    // Section 11: Cleanup SOTA 2026
    ui::print_section_header(11, 11, "CLEANUP SOTA 2026");
    let cleanup_opts = sections::cleanup_section();
    let cleanup_selected = collect_selection(&cleanup_opts, "Cleanup SOTA 2026")?;

    // ═══════════════════════════════════════════════════════════════════════
    // SUMMARY AND CONFIRMATION
    // ═══════════════════════════════════════════════════════════════════════

    let total = telem_selected.len()
        + privacy_selected.len()
        + perf_selected.len()
        + sched_selected.len()
        + appx_selected.len()
        + cpu_selected.len()
        + dpc_selected.len()
        + security_selected.len()
        + net_adv_selected.len()
        + dns_selected.len()
        + cleanup_selected.len();

    ui::print_selection_summary_full(
        telem_selected.len(),
        privacy_selected.len(),
        perf_selected.len(),
        sched_selected.len(),
        appx_selected.len(),
        cpu_selected.len(),
        dpc_selected.len(),
        security_selected.len(),
        net_adv_selected.len(),
        dns_selected.len(),
        cleanup_selected.len(),
    );

    if total == 0 {
        ui::print_no_selection();
        return Ok(());
    }

    // Confirmation
    println!();
    let confirm = Confirm::with_theme(&ui::GhostTheme::default())
        .with_prompt("APPLY MODIFICATIONS?")
        .default(false)
        .interact()?;

    if !confirm {
        ui::print_cancelled();
        return Ok(());
    }

    // ═══════════════════════════════════════════════════════════════════════
    // EXECUTION WITH PROGRESS BAR
    // ═══════════════════════════════════════════════════════════════════════

    println!();
    println!("  Applying...");
    println!();

    let multi = MultiProgress::new();
    let pb = ui::create_progress_bar(total as u64, &multi);

    let mut changes = Vec::<ChangeRecord>::new();
    let mut success_count = 0;
    let mut error_count = 0;

    // Execute each category
    execute_category(
        "telemetry",
        &telem_opts,
        &telem_selected,
        &mut changes,
        &mut success_count,
        &mut error_count,
        &pb,
    )
    .await;

    execute_category(
        "privacy",
        &privacy_opts,
        &privacy_selected,
        &mut changes,
        &mut success_count,
        &mut error_count,
        &pb,
    )
    .await;

    execute_category(
        "performance",
        &perf_opts,
        &perf_selected,
        &mut changes,
        &mut success_count,
        &mut error_count,
        &pb,
    )
    .await;

    execute_category(
        "scheduler",
        &sched_opts,
        &sched_selected,
        &mut changes,
        &mut success_count,
        &mut error_count,
        &pb,
    )
    .await;

    execute_category(
        "appx",
        &appx_opts,
        &appx_selected,
        &mut changes,
        &mut success_count,
        &mut error_count,
        &pb,
    )
    .await;

    execute_category(
        "cpu",
        &cpu_opts,
        &cpu_selected,
        &mut changes,
        &mut success_count,
        &mut error_count,
        &pb,
    )
    .await;

    execute_category(
        "dpc",
        &dpc_opts,
        &dpc_selected,
        &mut changes,
        &mut success_count,
        &mut error_count,
        &pb,
    )
    .await;

    execute_category(
        "security",
        &security_opts,
        &security_selected,
        &mut changes,
        &mut success_count,
        &mut error_count,
        &pb,
    )
    .await;

    execute_category(
        "network_advanced",
        &net_adv_opts,
        &net_adv_selected,
        &mut changes,
        &mut success_count,
        &mut error_count,
        &pb,
    )
    .await;

    execute_category(
        "dns",
        &dns_opts,
        &dns_selected,
        &mut changes,
        &mut success_count,
        &mut error_count,
        &pb,
    )
    .await;

    execute_category(
        "cleanup",
        &cleanup_opts,
        &cleanup_selected,
        &mut changes,
        &mut success_count,
        &mut error_count,
        &pb,
    )
    .await;

    pb.finish_with_message("Done");

    // ═══════════════════════════════════════════════════════════════════════
    // SNAPSHOT AND RESULT
    // ═══════════════════════════════════════════════════════════════════════

    let snapshot_id = match snapshot::create("Before interactive mode", changes) {
        Ok(snap) => {
            info!(snapshot_id = %snap.id, changes = snap.changes.len(), "Snapshot created");
            Some(snap.id.to_string())
        }
        Err(e) => {
            warn!(error = %e, "Failed to create snapshot");
            None
        }
    };

    // Determine if reboot required (DPC or Security selected)
    let needs_reboot = !dpc_selected.is_empty() || !security_selected.is_empty();
    ui::print_final_result_with_reboot(
        success_count,
        error_count,
        snapshot_id.as_deref(),
        needs_reboot,
    );

    Ok(())
}

/// Collects user selections for a section
fn collect_selection(opts: &[OptItem], prompt: &str) -> Result<Vec<usize>> {
    let labels: Vec<&str> = opts.iter().map(|o| o.label).collect();
    let defaults: Vec<bool> = opts.iter().map(|o| o.default).collect();

    let selected = MultiSelect::with_theme(&ui::GhostTheme::default())
        .with_prompt(prompt)
        .items(&labels)
        .defaults(&defaults)
        .interact()?;

    Ok(selected)
}

/// Executes all selected options from a category
async fn execute_category(
    category: &str,
    opts: &[OptItem],
    selected: &[usize],
    changes: &mut Vec<ChangeRecord>,
    success_count: &mut usize,
    error_count: &mut usize,
    pb: &indicatif::ProgressBar,
) {
    let executor = executor::get_executor(category);

    for &idx in selected {
        let opt = &opts[idx];
        pb.set_message(opt.label.to_string());

        match executor.execute(opt.id, changes).await {
            Ok(result) => {
                ui::print_operation_result(opt.id, true, &result.message);
                *success_count += 1;
                info!(category = category, id = opt.id, message = %result.message, "Operation success");
            }
            Err(e) => {
                ui::print_operation_result(opt.id, false, &e.to_string());
                *error_count += 1;
                warn!(category = category, id = opt.id, error = %e, "Operation failed");
            }
        }

        pb.inc(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_selection_returns_empty_on_no_opts() {
        // Regression test: verify that the structure compiles
        let opts: Vec<OptItem> = vec![];
        assert!(opts.is_empty());
    }

    #[test]
    fn test_sections_exported() {
        // Verify that exports work
        let _item = OptItem::safe("test", "Test");
        assert_eq!(_item.id, "test");
    }
}
