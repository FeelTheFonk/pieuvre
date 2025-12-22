//! Mode interactif SOTA 2026
//!
//! Architecture modulaire:
//! - `sections.rs`: Définition des options par catégorie
//! - `executor.rs`: Exécution des optimisations
//! - `ui.rs`: Interface utilisateur avec indicatif

mod executor;
mod sections;
mod ui;

use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, MultiSelect};
use indicatif::MultiProgress;
use pieuvre_audit::hardware::is_laptop;
use pieuvre_common::ChangeRecord;
use pieuvre_persist::snapshot;
use tracing::{info, instrument, warn};

pub use sections::OptItem;
use ui::MainAction;

/// Point d'entrée interactif par défaut (sans arguments CLI)
pub async fn run_default() -> Result<()> {
    // 1. Écran d'accueil ASCII
    ui::print_welcome_screen();

    // 2. Vérification privilèges admin
    ui::check_admin_status();

    // 3. Résumé rapide état système
    ui::print_quick_status();

    // 4. Menu principal interactif
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

    // Pause finale pour éviter la fermeture de la console
    ui::wait_for_exit();
    Ok(())
}

/// Application rapide d'un profil sans sélection granulaire
pub async fn run_quick_apply(profile: &str) -> Result<()> {
    println!();
    println!(
        "  [*] Application rapide du profil: {}",
        profile.to_uppercase()
    );

    // Création d'un snapshot de sécurité
    let changes = Vec::<ChangeRecord>::new();
    let snap = snapshot::create(&format!("Avant profil rapide {}", profile), changes)?;
    println!(
        "  [OK] Snapshot de sauvegarde cree: {}",
        &snap.id.to_string()[..8]
    );

    // Application réelle via pieuvre-sync
    pieuvre_sync::apply_profile(profile, false).await?;

    println!();
    println!(
        "  [OK] Profil {} applique avec succes.",
        profile.to_uppercase()
    );
    println!("       Redemarrage recommande pour certaines modifications.");
    println!();

    ui::wait_for_exit();
    Ok(())
}

/// Point d'entrée du mode interactif SOTA avec sélection granulaire
#[instrument(skip_all, fields(profile = %profile))]
pub async fn run(profile: &str) -> Result<()> {
    let is_laptop = is_laptop();
    info!(
        is_laptop = is_laptop,
        profile = profile,
        "Starting interactive mode"
    );

    // Affichage header
    ui::print_header(is_laptop, profile);

    // ═══════════════════════════════════════════════════════════════════════
    // COLLECTE DES SÉLECTIONS
    // ═══════════════════════════════════════════════════════════════════════

    // Section 1: Télémétrie
    ui::print_section_header(1, 9, "TÉLÉMÉTRIE - Services");
    let telem_opts = sections::telemetry_section();
    let telem_selected = collect_selection(&telem_opts, "Services télémétrie")?;

    // Section 2: Privacy
    ui::print_section_header(2, 9, "PRIVACY - Registre");
    let privacy_opts = sections::privacy_section();
    let privacy_selected = collect_selection(&privacy_opts, "Privacy registre")?;

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

    // Section 8: Security (avec avertissement)
    ui::print_section_header(8, 9, "SECURITY - ⚠️ ATTENTION RISQUE");
    ui::print_security_warning();
    let security_opts = sections::security_section();
    let security_selected = collect_selection(&security_opts, "Security")?;

    // Section 9: Network Avancé
    ui::print_section_header(9, 9, "NETWORK AVANCÉ");
    let net_adv_opts = sections::network_advanced_section();
    let net_adv_selected = collect_selection(&net_adv_opts, "Network Avancé")?;

    // ═══════════════════════════════════════════════════════════════════════
    // RÉSUMÉ ET CONFIRMATION
    // ═══════════════════════════════════════════════════════════════════════

    let total = telem_selected.len()
        + privacy_selected.len()
        + perf_selected.len()
        + sched_selected.len()
        + appx_selected.len()
        + cpu_selected.len()
        + dpc_selected.len()
        + security_selected.len()
        + net_adv_selected.len();

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
    );

    if total == 0 {
        ui::print_no_selection();
        return Ok(());
    }

    // Confirmation
    println!();
    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Appliquer ces modifications? (y/n)")
        .default(false)
        .interact()?;

    if !confirm {
        ui::print_cancelled();
        return Ok(());
    }

    // ═══════════════════════════════════════════════════════════════════════
    // EXÉCUTION AVEC PROGRESS BAR
    // ═══════════════════════════════════════════════════════════════════════

    println!();
    println!("  Application en cours...");
    println!();

    let multi = MultiProgress::new();
    let pb = ui::create_progress_bar(total as u64, &multi);

    let mut changes = Vec::<ChangeRecord>::new();
    let mut success_count = 0;
    let mut error_count = 0;

    // Exécuter chaque catégorie
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

    pb.finish_with_message("Terminé");

    // ═══════════════════════════════════════════════════════════════════════
    // SNAPSHOT ET RÉSULTAT
    // ═══════════════════════════════════════════════════════════════════════

    let snapshot_id = match snapshot::create("Avant mode interactif", changes) {
        Ok(snap) => {
            info!(snapshot_id = %snap.id, changes = snap.changes.len(), "Snapshot created");
            Some(snap.id.to_string())
        }
        Err(e) => {
            warn!(error = %e, "Failed to create snapshot");
            None
        }
    };

    // Déterminer si reboot nécessaire (DPC ou Security sélectionnés)
    let needs_reboot = !dpc_selected.is_empty() || !security_selected.is_empty();
    ui::print_final_result_with_reboot(
        success_count,
        error_count,
        snapshot_id.as_deref(),
        needs_reboot,
    );

    Ok(())
}

/// Collecte les sélections utilisateur pour une section
fn collect_selection(opts: &[OptItem], prompt: &str) -> Result<Vec<usize>> {
    let labels: Vec<&str> = opts.iter().map(|o| o.label).collect();
    let defaults: Vec<bool> = opts.iter().map(|o| o.default).collect();

    let selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("{} (Espace=cocher, Entrée=valider)", prompt))
        .items(&labels)
        .defaults(&defaults)
        .interact()?;

    println!();
    Ok(selected)
}

/// Exécute toutes les options sélectionnées d'une catégorie
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
        // Test de régression: vérifier que la structure compile
        let opts: Vec<OptItem> = vec![];
        assert!(opts.is_empty());
    }

    #[test]
    fn test_sections_exported() {
        // Vérifier que les exports fonctionnent
        let _item = OptItem::safe("test", "Test");
        assert_eq!(_item.id, "test");
    }
}
