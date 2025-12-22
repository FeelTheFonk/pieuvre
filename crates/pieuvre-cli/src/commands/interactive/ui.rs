use anyhow::Result;
use console::style;
use dialoguer::{theme::ColorfulTheme, Select};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

/// Actions disponibles depuis le menu principal
pub enum MainAction {
    /// Mode interactif standard avec sélection granulaire
    Interactive(String),
    /// Application rapide d'un profil (gaming, privacy, workstation)
    QuickApply(String),
    /// Afficher le statut actuel du système
    Status,
    /// Gestion des snapshots et rollback
    Rollback,
    /// Quitter l'application
    Exit,
}

/// Affiche le header du mode interactif (style pro, sans emoji)
pub fn print_header(is_laptop: bool, profile: &str) {
    println!();
    println!(
        "{}",
        style("┌──────────────────────────────────────────────────────────────────┐").cyan()
    );
    println!(
        "{}",
        style("│           PIEUVRE - Selection des Optimisations                  │")
            .cyan()
            .bold()
    );
    println!(
        "{}",
        style("└──────────────────────────────────────────────────────────────────┘").cyan()
    );
    println!();
    println!("  {}", style("NAVIGATION:").bold());
    println!("    Fleches   Haut/Bas pour naviguer");
    println!("    Espace    Cocher/Decocher une option");
    println!("    Entree    Valider la selection");
    println!();
    println!(
        "  Systeme: {}",
        if is_laptop {
            style("LAPTOP").yellow()
        } else {
            style("DESKTOP").green()
        }
    );
    println!("  Profil:  {}", style(profile.to_uppercase()).cyan().bold());
    println!();

    if is_laptop {
        println!(
            "  {} Options [WARN][LAPTOP] deconseillees sur batterie",
            style("[!]").yellow().bold()
        );
        println!();
    }
}

/// Affiche l'écran d'accueil professionnel (ASCII Art SOTA)
pub fn print_welcome_screen() {
    println!();
    println!(
        "{}",
        style("┌──────────────────────────────────────────────────────────────────┐").cyan()
    );
    println!("│                                                                  │");
    println!(
        "│    pieuvre. - v{}                                         │",
        env!("CARGO_PKG_VERSION")
    );
    println!("│                                                                  │");
    println!(
        "{}",
        style("└──────────────────────────────────────────────────────────────────┘").cyan()
    );
    println!();
}

/// Vérifie le statut administrateur et affiche un avertissement si nécessaire
pub fn check_admin_status() {
    if is_elevated() {
        println!("  [OK] Privileges administrateur detectes");
    } else {
        println!(
            "  {} Execution en tant qu'utilisateur standard",
            style("[WARN]").yellow().bold()
        );
        println!("       Certaines optimisations necessitent des privileges eleves.");
        println!("       Clic droit > Executer en tant qu'administrateur recommande.");
        println!();
    }
}

/// Affiche un résumé rapide de l'état du système
pub fn print_quick_status() {
    println!();
    println!("  {}", style("ETAT SYSTEME").bold());
    println!("  {}", style("─".repeat(60)).dim());

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
        Err(_) => println!("  Timer:       Non disponible"),
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
        Err(_) => println!("  Power Plan:  Non disponible"),
    }

    // DiagTrack
    match pieuvre_sync::services::get_service_start_type("DiagTrack") {
        Ok(4) => println!("  DiagTrack:   Disabled [OK]"),
        Ok(_) => println!("  DiagTrack:   Running [--]"),
        Err(_) => println!("  DiagTrack:   Non trouve"),
    }

    // ETW Latency (SOTA 2026)
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
            println!("  Latency:     {} [SOTA]", lat_style);
        }
        _ => println!("  Latency:     {} [OFF]", style("Non demarre").dim()),
    }

    println!();
}

/// Affiche le menu principal et retourne l'action choisie
pub fn show_main_menu() -> Result<MainAction> {
    let options = &[
        "Selection personnalisee   - Choisir les optimisations une par une",
        "Appliquer profil GAMING   - Optimisations recommandees gaming",
        "Appliquer profil PRIVACY  - Protection donnees personnelles",
        "Appliquer profil WORKSTATION - Equilibre performance/stabilite",
        "Afficher statut complet   - Etat detaille du systeme",
        "Gerer les snapshots       - Rollback des modifications",
        "Quitter",
    ];

    println!("  {}", style("QUE VOULEZ-VOUS FAIRE ?").bold());
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

/// Sélection du profil de base pour la personnalisation
fn select_profile() -> Result<String> {
    let profiles = &[
        "GAMING      - Latence minimale, performance maximale",
        "PRIVACY     - Telemetrie et tracking desactives",
        "WORKSTATION - Equilibre productivite/performance",
    ];

    println!();
    println!("  {}", style("PROFIL DE BASE").bold());
    println!("  Le profil determine les options pre-cochees par defaut.");
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

/// Message de fin
pub fn print_goodbye() {
    println!();
    println!("  A bientot.");
    println!();
}

/// Attend que l'utilisateur appuie sur Entrée avant de quitter
pub fn wait_for_exit() {
    println!();
    println!("  {}", style("Appuyez sur ENTREE pour quitter...").dim());
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
}

/// Vérifie si le processus a des privilèges élevés (SOTA Native)
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

/// Affiche le header d'une section
pub fn print_section_header(number: u8, total: u8, name: &str) {
    println!("{}", style("─".repeat(68)).dim());
    println!(
        "  {}/{}  {}",
        style(number).cyan().bold(),
        style(total).dim(),
        style(name).bold()
    );
    println!("{}", style("─".repeat(68)).dim());
}

/// Affiche le résumé des sélections (version simplifiée, 5 sections)
/// Conservée pour rétro-compatibilité
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
    println!(
        "{}",
        style("┌──────────────────────────────────────────────────────────────────┐").cyan()
    );
    println!(
        "{}",
        style("│                    RÉSUMÉ SÉLECTION                              │")
            .cyan()
            .bold()
    );
    println!(
        "{}",
        style("└──────────────────────────────────────────────────────────────────┘").cyan()
    );
    println!();
    println!("  Télémétrie:   {}", style(telem_count).green().bold());
    println!("  Privacy:      {}", style(privacy_count).green().bold());
    println!("  Performance:  {}", style(perf_count).green().bold());
    println!("  Scheduler:    {}", style(sched_count).green().bold());
    println!("  AppX:         {}", style(appx_count).green().bold());
    println!();
    println!(
        "  {}: {} optimisations sélectionnées",
        style("Total").bold(),
        style(total).cyan().bold()
    );
}

/// Crée une barre de progression pour l'exécution
pub fn create_progress_bar(total: u64, multi: &MultiProgress) -> ProgressBar {
    let pb = multi.add(ProgressBar::new(total));
    pb.set_style(
        ProgressStyle::with_template("  {spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("█▓▒░"),
    );
    pb
}

/// Crée un spinner pour une opération
#[allow(dead_code)]
pub fn create_spinner(multi: &MultiProgress, message: &str) -> ProgressBar {
    let sp = multi.add(ProgressBar::new_spinner());
    sp.set_style(ProgressStyle::with_template("  {spinner:.green} {msg}").unwrap());
    sp.set_message(message.to_string());
    sp
}

/// Affiche le résultat d'une opération
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

/// Affiche le résultat final
pub fn print_final_result(success_count: usize, error_count: usize, snapshot_id: Option<&str>) {
    println!();
    println!(
        "{}",
        style("┌──────────────────────────────────────────────────────────────────┐").cyan()
    );
    println!(
        "{}",
        style("│                      RÉSULTAT                                    │")
            .cyan()
            .bold()
    );
    println!(
        "{}",
        style("└──────────────────────────────────────────────────────────────────┘").cyan()
    );
    println!();
    println!("  Succès:  {}", style(success_count).green().bold());
    println!(
        "  Erreurs: {}",
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
            style("  [OK] Toutes les modifications appliquées avec succès.")
                .green()
                .bold()
        );
    } else {
        println!(
            "{}",
            style("  [!] Certaines modifications ont échoué.")
                .yellow()
                .bold()
        );
        println!("      Exécutez en tant qu'administrateur si nécessaire.");
    }

    println!();
    println!(
        "  Pour annuler: {}",
        style("pieuvre rollback --last").cyan()
    );
    println!("  Pour vérifier: {}", style("pieuvre status").cyan());
    println!();
}

/// Affiche un message d'annulation
pub fn print_cancelled() {
    println!();
    println!(
        "{}",
        style("  [*] Annulé. Aucune modification effectuée.").yellow()
    );
    println!();
}

/// Affiche un message quand aucune option sélectionnée
pub fn print_no_selection() {
    println!();
    println!(
        "{}",
        style("  [*] Aucune optimisation sélectionnée. Fin.").yellow()
    );
    println!();
}

/// Affiche un avertissement de sécurité pour la section Security
pub fn print_security_warning() {
    println!();
    println!(
        "  {}",
        style("⚠️  ATTENTION: Options à risque de sécurité élevé")
            .red()
            .bold()
    );
    println!(
        "  {}",
        style("    Ces options réduisent la protection système.").red()
    );
    println!(
        "  {}",
        style("    À utiliser uniquement sur systèmes de gaming isolés.").red()
    );
    println!();
}

/// Affiche le résumé complet des sélections (9 sections)
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
    println!(
        "{}",
        style("┌──────────────────────────────────────────────────────────────────┐").cyan()
    );
    println!(
        "{}",
        style("│                    RÉSUMÉ SÉLECTION                              │")
            .cyan()
            .bold()
    );
    println!(
        "{}",
        style("└──────────────────────────────────────────────────────────────────┘").cyan()
    );
    println!();
    println!("  Télémétrie:      {}", style(telem_count).green().bold());
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
    println!("  Network Avancé:  {}", style(net_adv_count).green().bold());
    println!();
    println!(
        "  {}: {} optimisations sélectionnées",
        style("Total").bold(),
        style(total).cyan().bold()
    );

    // Avertissement si options critiques
    if security_count > 0 {
        println!();
        println!(
            "  {}",
            style("[!] Options de sécurité sélectionnées - Reboot requis")
                .yellow()
                .bold()
        );
    }

    // Indicateur reboot si DPC ou security
    if dpc_count > 0 || security_count > 0 {
        println!(
            "  {}",
            style("[!] Certaines options nécessitent un redémarrage").dim()
        );
    }
}

/// Affiche le résumé final avec indication des modifications nécessitant reboot
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
            style("  [!] REDÉMARRAGE RECOMMANDÉ pour appliquer toutes les modifications.")
                .yellow()
                .bold()
        );
        println!();
    }
}

#[cfg(test)]
mod tests {
    // Tests UI sont difficiles à automatiser, on vérifie juste la compilation
    #[test]
    fn test_module_compiles() {
        // Si ce test compile, le module est syntaxiquement correct
    }
}
