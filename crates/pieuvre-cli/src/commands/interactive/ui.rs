use anyhow::Result;
use console::style;
use dialoguer::{theme::ColorfulTheme, Select};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

/// Actions available from the main menu
pub enum MainAction {
    /// Standard interactive mode with granular selection
    Interactive(String),
    /// Quick apply of a profile (gaming, privacy, workstation)
    QuickApply(String),
    /// Display current system status
    Status,
    /// Snapshot management and rollback
    Rollback,
    /// Exit the application
    Exit,
}

/// Displays the interactive mode header (The Ghost style)
pub fn print_header(is_laptop: bool, profile: &str) {
    println!();
    println!(
        "  {}  {} · {}",
        style("⣠⟬ ⊚ ⟭⣄").cyan().dim(),
        if is_laptop {
            style("LAPTOP").yellow().dim()
        } else {
            style("DESKTOP").green().dim()
        },
        style(profile.to_uppercase()).white().bold()
    );
}

/// Displays the professional welcome screen (ASCII Art)
pub fn print_welcome_screen() {
    println!();
    println!("  {}", style("⣠⟬ ⊚ ⟭⣄").cyan().bold());
    println!(
        "  {}",
        style(format!("PIEUVRE v{}", env!("CARGO_PKG_VERSION"))).dim()
    );
    println!();
}

/// Checks administrator status and displays a warning if necessary
pub fn check_admin_status() {
    if is_elevated() {
        println!("  [OK] Administrator privileges detected");
    } else {
        println!(
            "  {} Running as standard user",
            style("[WARN]").yellow().bold()
        );
        println!("       Some optimizations require elevated privileges.");
        println!("       Right-click > Run as administrator recommended.");
        println!();
    }
}

/// Displays a quick system state summary
pub fn print_quick_status() {
    println!();
    println!("  {}", style("SYSTEM STATE").bold());

    // Hardware
    let is_laptop = pieuvre_audit::hardware::is_laptop();
    println!(
        "  Type:        {}",
        if is_laptop { "Laptop" } else { "Desktop" }
    );

    if let Ok(hw) = pieuvre_audit::hardware::probe_hardware() {
        if hw.cpu.is_hybrid {
            println!(
                "  CPU:         {} [Hybrid: {}P/{}E]",
                hw.cpu.model_name,
                style(hw.cpu.p_cores.len()).cyan(),
                style(hw.cpu.e_cores.len()).yellow()
            );
        } else {
            println!(
                "  CPU:         {} [{} Cores]",
                hw.cpu.model_name, hw.cpu.physical_cores
            );
        }
    }

    // Timer
    match pieuvre_sync::timer::get_timer_resolution() {
        Ok(info) => {
            let status = if info.current_ms() <= 0.55 {
                "[OK]"
            } else {
                "[--]"
            };
            println!("  Timer:       {:.2}ms {}", info.current_ms(), status);
        }
        Err(_) => println!("  Timer:       Not available"),
    }

    // Power Plan
    match pieuvre_sync::power::get_active_power_plan() {
        Ok(plan) => {
            let status = if plan.contains("High") || plan.contains("Ultimate") {
                "[OK]"
            } else {
                "[--]"
            };
            println!("  Power Plan:  {} {}", plan, status);
        }
        Err(_) => println!("  Power Plan:  Not available"),
    }

    // DiagTrack
    match pieuvre_sync::services::get_service_start_type("DiagTrack") {
        Ok(4) => println!("  DiagTrack:   Disabled [OK]"),
        Ok(_) => println!("  DiagTrack:   Running [--]"),
        Err(_) => println!("  DiagTrack:   Not found"),
    }

    // ETW Latency
    match pieuvre_audit::etw::session::EtwSession::check_active() {
        Ok(true) => {
            let max_lat = pieuvre_audit::etw::monitor::LatencyMonitor::global().get_max_latency();
            let lat_style = if max_lat < 100 {
                style(format!("{}us", max_lat)).green()
            } else if max_lat < 500 {
                style(format!("{}us", max_lat)).yellow()
            } else {
                style(format!("{}us", max_lat)).red().bold()
            };
            println!("  Latency:     {} [ACTIVE]", lat_style);
        }
        _ => println!("  Latency:     {} [OFF]", style("Not started").dim()),
    }

    println!();
}

/// Displays the main menu and returns the chosen action
pub fn show_main_menu() -> Result<MainAction> {
    let options = &[
        "Custom Selection     - Choose optimizations one by one",
        "Apply GAMING Profile - Recommended gaming optimizations",
        "Apply PRIVACY Profile- Personal data protection",
        "Apply WORKSTATION    - Balance performance/stability",
        "Display Full Status  - Detailed system state",
        "Manage Snapshots     - Rollback modifications",
        "Exit",
    ];

    println!("  {}", style("WHAT DO YOU WANT TO DO?").bold());
    println!();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(options)
        .default(0)
        .interact()?;

    match selection {
        0 => {
            let profile = select_profile()?;
            Ok(MainAction::Interactive(profile))
        }
        1 => Ok(MainAction::QuickApply("gaming".to_string())),
        2 => Ok(MainAction::QuickApply("privacy".to_string())),
        3 => Ok(MainAction::QuickApply("workstation".to_string())),
        4 => Ok(MainAction::Status),
        5 => Ok(MainAction::Rollback),
        _ => Ok(MainAction::Exit),
    }
}

/// Base profile selection for customization
fn select_profile() -> Result<String> {
    let profiles = &[
        "GAMING      - Minimum latency, maximum performance",
        "PRIVACY     - Telemetry and tracking disabled",
        "WORKSTATION - Balance productivity/performance",
    ];

    println!();
    println!("  {}", style("BASE PROFILE").bold());
    println!("  The profile determines pre-checked options by default.");
    println!();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(profiles)
        .default(0)
        .interact()?;

    Ok(match selection {
        0 => "gaming",
        1 => "privacy",
        _ => "workstation",
    }
    .to_string())
}

/// Goodbye message
pub fn print_goodbye() {
    println!();
    println!("  Goodbye.");
    println!();
}

/// Waits for Enter key before exiting
pub fn wait_for_exit() {
    println!();
    println!("  {}", style("Press ENTER to exit...").dim());
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
}

/// Checks if the process has elevated privileges
fn is_elevated() -> bool {
    use windows::Win32::Foundation::HANDLE;
    use windows::Win32::Security::{
        GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY,
    };
    use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

    unsafe {
        let mut token = HANDLE::default();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).is_err() {
            return false;
        }

        let mut elevation = TOKEN_ELEVATION::default();
        let mut size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;

        let result = GetTokenInformation(
            token,
            TokenElevation,
            Some(&mut elevation as *mut _ as *mut _),
            size,
            &mut size,
        );

        let _ = windows::Win32::Foundation::CloseHandle(token);

        result.is_ok() && elevation.TokenIsElevated != 0
    }
}

/// Displays a section header
pub fn print_section_header(number: u8, total: u8, name: &str) {
    println!();
    println!(
        "  {} · {}  {}",
        style(number).cyan().bold(),
        style(total).dim(),
        style(name).bold()
    );
}

/// Displays selection summary (simplified version, 5 sections)
#[allow(dead_code)]
pub fn print_selection_summary(
    telem_count: usize,
    privacy_count: usize,
    perf_count: usize,
    sched_count: usize,
    appx_count: usize,
) {
    let total = telem_count + privacy_count + perf_count + sched_count + appx_count;

    println!();
    println!("  {}", style("SELECTION SUMMARY").bold());
    println!("  Telemetry:    {}", style(telem_count).green().bold());
    println!("  Privacy:      {}", style(privacy_count).green().bold());
    println!("  Performance:  {}", style(perf_count).green().bold());
    println!("  Scheduler:    {}", style(sched_count).green().bold());
    println!("  AppX:         {}", style(appx_count).green().bold());
    println!();
    println!(
        "  {}: {} optimizations selected",
        style("Total").bold(),
        style(total).cyan().bold()
    );
}

/// Creates a progress bar for execution
pub fn create_progress_bar(total: u64, multi: &MultiProgress) -> ProgressBar {
    let pb = multi.add(ProgressBar::new(total));
    pb.set_style(
        ProgressStyle::with_template("  {spinner:.cyan} {bar:40.white/dim} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("█ "),
    );
    pb
}

/// Creates a spinner for an operation
#[allow(dead_code)]
pub fn create_spinner(multi: &MultiProgress, message: &str) -> ProgressBar {
    let sp = multi.add(ProgressBar::new_spinner());
    sp.set_style(ProgressStyle::with_template("  {spinner:.cyan} {msg}").unwrap());
    sp.set_message(message.to_string());
    sp
}

/// Displays operation result
pub fn print_operation_result(name: &str, success: bool, message: &str) {
    if success {
        println!(
            "  {} {} - {}",
            style("[OK]").green().bold(),
            name,
            style(message).dim()
        );
    } else {
        println!(
            "  {} {} - {}",
            style("[ERR]").red().bold(),
            name,
            style(message).red()
        );
    }
}

/// Displays final result
pub fn print_final_result(success_count: usize, error_count: usize, snapshot_id: Option<&str>) {
    println!();
    println!("  {}", style("RESULT").bold());
    println!("  Success:  {}", style(success_count).green().bold());
    println!(
        "  Errors:   {}",
        if error_count > 0 {
            style(error_count).red().bold()
        } else {
            style(error_count).green().bold()
        }
    );

    if let Some(id) = snapshot_id {
        println!();
        println!("  Snapshot: {}", style(&id[..8]).dim());
    }

    println!();

    if error_count == 0 {
        println!(
            "{}",
            style("  [OK] All modifications applied successfully.")
                .green()
                .bold()
        );
    } else {
        println!(
            "{}",
            style("  [!] Some modifications failed.").yellow().bold()
        );
        println!("      Run as administrator if necessary.");
    }

    println!();
    println!("  To undo:   {}", style("pieuvre rollback --last").cyan());
    println!("  To verify: {}", style("pieuvre status").cyan());
    println!();
}

/// Displays cancellation message
pub fn print_cancelled() {
    println!();
    println!(
        "{}",
        style("  [*] Cancelled. No modifications performed.").yellow()
    );
    println!();
}

/// Displays message when no option selected
pub fn print_no_selection() {
    println!();
    println!(
        "{}",
        style("  [*] No optimization selected. Done.").yellow()
    );
    println!();
}

/// Displays security warning for Security section
pub fn print_security_warning() {
    println!();
    println!(
        "  {}",
        style("[WARN] CAUTION: High security risk options")
            .red()
            .bold()
    );
    println!(
        "  {}",
        style("    These options reduce system protection.").red()
    );
    println!(
        "  {}",
        style("    Use only on isolated gaming systems.").red()
    );
    println!();
}

/// Displays full selection summary (9 sections)
#[allow(clippy::too_many_arguments)]
pub fn print_selection_summary_full(
    telem_count: usize,
    privacy_count: usize,
    perf_count: usize,
    sched_count: usize,
    appx_count: usize,
    cpu_count: usize,
    dpc_count: usize,
    security_count: usize,
    net_adv_count: usize,
) {
    let total = telem_count
        + privacy_count
        + perf_count
        + sched_count
        + appx_count
        + cpu_count
        + dpc_count
        + security_count
        + net_adv_count;

    println!();
    println!("  {}", style("SELECTION SUMMARY").bold());
    println!("  Telemetry:       {}", style(telem_count).green().bold());
    println!("  Privacy:         {}", style(privacy_count).green().bold());
    println!("  Performance:     {}", style(perf_count).green().bold());
    println!("  Scheduler:       {}", style(sched_count).green().bold());
    println!("  AppX:            {}", style(appx_count).green().bold());
    println!("  CPU/Memory:      {}", style(cpu_count).green().bold());
    println!("  DPC Latency:     {}", style(dpc_count).green().bold());
    if security_count > 0 {
        println!("  Security:        {}", style(security_count).red().bold());
    } else {
        println!(
            "  Security:        {}",
            style(security_count).green().bold()
        );
    }
    println!("  Advanced Network:{}", style(net_adv_count).green().bold());
    println!();
    println!(
        "  {}: {} optimizations selected",
        style("Total").bold(),
        style(total).cyan().bold()
    );

    // Warning if critical options
    if security_count > 0 {
        println!();
        println!(
            "  {}",
            style("[!] Security options selected - Reboot required")
                .yellow()
                .bold()
        );
    }

    // Reboot indicator if DPC or security
    if dpc_count > 0 || security_count > 0 {
        println!("  {}", style("[!] Some options require a restart").dim());
    }
}

/// Displays final result with reboot indication
pub fn print_final_result_with_reboot(
    success_count: usize,
    error_count: usize,
    snapshot_id: Option<&str>,
    needs_reboot: bool,
) {
    print_final_result(success_count, error_count, snapshot_id);

    if needs_reboot && error_count == 0 {
        println!();
        println!(
            "{}",
            style("  [!] REBOOT RECOMMENDED to apply all modifications.")
                .yellow()
                .bold()
        );
        println!();
    }
}

#[cfg(test)]
mod tests {
    // UI tests are difficult to automate, we just verify compilation
    #[test]
    fn test_module_compiles() {
        // If this test compiles, the module is syntactically correct
    }
}
