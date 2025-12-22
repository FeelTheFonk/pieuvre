use anyhow::Result;
use console::{style, Term};
use dialoguer::{theme::Theme, Select};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::fmt;

/// Actions available from the main menu
pub enum MainAction {
    Interactive(String),
    QuickApply(String),
    Status,
    Rollback,
    Exit,
}

// ══════════════════════════════════════════════════════════════════════════════
// GHOST THEME - SOTA 2026
// ══════════════════════════════════════════════════════════════════════════════

pub struct GhostTheme {
    pub prompt_style: console::Style,
    pub active_item_style: console::Style,
    pub inactive_item_style: console::Style,
    pub active_item_prefix: String,
    pub inactive_item_prefix: String,
}

impl Default for GhostTheme {
    fn default() -> Self {
        Self {
            prompt_style: console::Style::new().for_stderr().bold(),
            active_item_style: console::Style::new().for_stderr().cyan(),
            inactive_item_style: console::Style::new().for_stderr().dim(),
            active_item_prefix: "  ● ".to_string(),
            inactive_item_prefix: "  ○ ".to_string(),
        }
    }
}

impl Theme for GhostTheme {
    fn format_prompt(&self, f: &mut dyn fmt::Write, prompt: &str) -> fmt::Result {
        write!(f, "\n  {}\n", self.prompt_style.apply_to(prompt))
    }

    fn format_select_prompt_item(
        &self,
        f: &mut dyn fmt::Write,
        text: &str,
        active: bool,
    ) -> fmt::Result {
        let prefix = if active {
            &self.active_item_prefix
        } else {
            &self.inactive_item_prefix
        };
        let style = if active {
            &self.active_item_style
        } else {
            &self.inactive_item_style
        };
        write!(f, "{}{}", prefix, style.apply_to(text))
    }

    fn format_multi_select_prompt_item(
        &self,
        f: &mut dyn fmt::Write,
        text: &str,
        checked: bool,
        active: bool,
    ) -> fmt::Result {
        let prefix = if checked { "  ● " } else { "  ○ " };
        let style = if active {
            &self.active_item_style
        } else {
            &self.inactive_item_style
        };
        write!(f, "{}{}", prefix, style.apply_to(text))
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// LAYOUT ENGINE
// ══════════════════════════════════════════════════════════════════════════════

pub fn print_box_top(title: &str) {
    let width = 60;
    let title_len = title.len();
    let left_line = (width - title_len - 4) / 2;
    let right_line = width - title_len - 4 - left_line;

    println!(
        "  {}{}{}{}",
        style("┌").dim(),
        style("─".repeat(left_line)).dim(),
        style(format!(" {} ", title.to_uppercase())).bold(),
        style("─".repeat(right_line) + "┐").dim()
    );
}

pub fn print_box_bottom() {
    println!("  {}\n", style(format!("└{}┘", "─".repeat(58))).dim());
}

pub fn print_line(content: &str) {
    println!(
        "  {} {:<56} {}",
        style("│").dim(),
        content,
        style("│").dim()
    );
}

// ══════════════════════════════════════════════════════════════════════════════
// UI COMPONENTS
// ══════════════════════════════════════════════════════════════════════════════

pub fn print_header(is_laptop: bool, profile: &str) {
    println!();
    println!(
        "  {}  {} · {}",
        style(".⣠⟬ ⊚ ⟭⣄.").cyan(),
        if is_laptop {
            style("LAPTOP").yellow().dim()
        } else {
            style("DESKTOP").green().dim()
        },
        style(profile.to_uppercase()).white().bold()
    );
}

pub fn print_welcome_screen() {
    let term = Term::stdout();
    let _ = term.clear_screen();
    println!();
    println!("  {}", style(".⣠⟬ ⊚ ⟭⣄.").cyan().bold());
    println!(
        "  {}",
        style(format!("pieuvre v{}", env!("CARGO_PKG_VERSION"))).dim()
    );
    println!();
}

pub fn check_admin_status() {
    if is_elevated() {
        println!("  {} Administrator privileges active", style("●").green());
    } else {
        println!("  {} Standard user - Limited mode", style("○").yellow());
        println!(
            "    {} Run as administrator for full control",
            style("└─").dim()
        );
        println!();
    }
}

pub fn print_quick_status() {
    print_box_top("Apex Intelligence");

    let is_laptop = pieuvre_audit::hardware::is_laptop();
    print_line(&format!(
        "Chassis:     {}",
        if is_laptop {
            "Mobile / Laptop"
        } else {
            "Stationary / Desktop"
        }
    ));

    if let Ok(hw) = pieuvre_audit::hardware::probe_hardware() {
        if hw.cpu.is_hybrid {
            print_line(&format!(
                "CPU:         {} ({}P/{}E)",
                hw.cpu.model_name,
                hw.cpu.p_cores.len(),
                hw.cpu.e_cores.len()
            ));
        } else {
            print_line(&format!(
                "CPU:         {} ({} Cores)",
                hw.cpu.model_name, hw.cpu.physical_cores
            ));
        }
    }

    // Sentinel & Hardening Status
    let sentinel_status = style("ACTIVE / LOCKED").green();
    print_line(&format!("Sentinel:    {}", sentinel_status));

    // DNS & AI Status
    if let Ok(doh) = pieuvre_sync::registry::read_dword_value(
        r"SYSTEM\CurrentControlSet\Services\Dnscache\Parameters",
        "EnableAutoDoh",
    ) {
        let status = if doh == 2 {
            style("SOTA (DoH)").green()
        } else {
            style("STOCK").dim()
        };
        print_line(&format!("DNS Engine:  {}", status));
    }

    if let Ok(recall) = pieuvre_sync::registry::read_dword_value(
        r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
        "DisableAIDataAnalysis",
    ) {
        let status = if recall == 1 {
            style("NEUTRALIZED").green()
        } else {
            style("ACTIVE").red()
        };
        print_line(&format!("AI De-bloat: {}", status));
    }

    if let Ok(info) = pieuvre_sync::timer::get_timer_resolution() {
        let status = if info.current_ms() <= 0.55 {
            style("OPTIMIZED").green()
        } else {
            style("STOCK").dim()
        };
        print_line(&format!(
            "Timer:       {:.2}ms ({})",
            info.current_ms(),
            status
        ));
    }

    if let Ok(plan) = pieuvre_sync::power::get_active_power_plan() {
        let status = if plan.contains("High") || plan.contains("Ultimate") {
            style("PERF").green()
        } else {
            style("BALANCED").dim()
        };
        print_line(&format!("Power:       {} ({})", plan, status));
    }

    print_box_bottom();
}

pub fn show_main_menu() -> Result<MainAction> {
    let options = &[
        "Custom Selection     - Granular Apex control",
        "Apply GAMING Profile - Maximum performance Apex",
        "Apply PRIVACY Profile- Data protection Apex",
        "Apply WORKSTATION    - Stability & Speed Apex",
        "Display Apex Status  - Deep system audit",
        "Manage Snapshots     - Persistence & Rollback",
        "Exit",
    ];

    let selection = Select::with_theme(&GhostTheme::default())
        .with_prompt("SELECT OPERATION")
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

fn select_profile() -> Result<String> {
    let profiles = &["GAMING", "PRIVACY", "WORKSTATION"];
    let selection = Select::with_theme(&GhostTheme::default())
        .with_prompt("BASE PROFILE")
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

pub fn print_section_header(number: u8, total: u8, name: &str) {
    println!(
        "\n  {} {} / {}  {}",
        style("»").cyan(),
        style(number).bold(),
        style(total).dim(),
        style(name).bold()
    );
}

pub fn create_progress_bar(total: u64, multi: &MultiProgress) -> ProgressBar {
    let pb = multi.add(ProgressBar::new(total));
    pb.set_style(
        ProgressStyle::with_template("  {spinner:.cyan} {msg} [{bar:30.white/dim}] {pos}/{len}")
            .unwrap()
            .progress_chars("· "),
    );
    pb
}

pub fn print_operation_result(name: &str, success: bool, message: &str) {
    let prefix = if success {
        style("  ●").green()
    } else {
        style("  ○").red()
    };
    println!("{} {:<20} {}", prefix, style(name).dim(), message);
}

pub fn print_final_result_with_reboot(
    success: usize,
    errors: usize,
    snap_id: Option<&str>,
    reboot: bool,
) {
    println!("\n  {}", style("EXECUTION COMPLETE").bold());
    println!("  {} Success: {}", style("●").green(), success);
    if errors > 0 {
        println!("  {} Errors:  {}", style("●").red(), errors);
    }

    if let Some(id) = snap_id {
        println!("  {} Snapshot: {}", style("●").dim(), &id[..8]);
    }

    if reboot {
        println!("\n  {} REBOOT RECOMMENDED", style("!").yellow().bold());
    }
    println!();
}

pub fn print_goodbye() {
    println!("\n  Goodbye.\n");
}
pub fn wait_for_exit() {
    println!("  {}", style("Press ENTER to exit...").dim());
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
}

pub fn print_cancelled() {
    println!("\n  {} Operation cancelled.\n", style("!").yellow());
}
pub fn print_no_selection() {
    println!("\n  {} No options selected.\n", style("!").dim());
}
pub fn print_security_warning() {
    println!(
        "  {} {}",
        style("!").red().bold(),
        style("CAUTION: Security risk options ahead").red()
    );
}

pub fn print_selection_summary_full(
    telem: usize,
    priva: usize,
    perf: usize,
    sched: usize,
    appx: usize,
    cpu: usize,
    dpc: usize,
    sec: usize,
    net: usize,
    dns: usize,
    clean: usize,
) {
    let total = telem + priva + perf + sched + appx + cpu + dpc + sec + net + dns + clean;
    println!("\n  {} SELECTION SUMMARY", style("»").cyan());
    println!("  Total: {} optimizations", style(total).bold());
    if sec > 0 {
        println!("  {} Security options selected", style("!").yellow());
    }
}

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
