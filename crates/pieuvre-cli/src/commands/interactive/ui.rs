//! Interface utilisateur pour le mode interactif
//!
//! Module SOTA 2026: Gestion affichage avec indicatif progress bars.

use console::style;
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};

/// Affiche le header du mode interactif
pub fn print_header(is_laptop: bool, profile: &str) {
    println!();
    println!("{}", style("═".repeat(68)).cyan());
    println!("{}", style("           PIEUVRE - Mode Interactif SOTA").cyan().bold());
    println!("{}", style("═".repeat(68)).cyan());
    println!();
    println!("  {}", style("NAVIGATION:").bold());
    println!("    {}    Haut/Bas pour naviguer", style("Flèches").green());
    println!("    {}     Cocher/Décocher une option", style("Espace").green());
    println!("    {}     Valider la sélection", style("Entrée").green());
    println!();
    println!("  Système: {}", if is_laptop { 
        style("LAPTOP (batterie détectée)").yellow() 
    } else { 
        style("DESKTOP").green() 
    });
    println!("  Profil:  {}", style(profile.to_uppercase()).cyan().bold());
    println!();
    
    if is_laptop {
        println!("  {} Options avec [LAPTOP] déconseillées sur batterie", style("[!]").yellow().bold());
        println!();
    }
}

/// Affiche le header d'une section
pub fn print_section_header(number: u8, total: u8, name: &str) {
    println!("{}", style("─".repeat(68)).dim());
    println!("  {}/{}  {}", 
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
    println!("{}", style("═".repeat(68)).cyan());
    println!("{}", style("                    RÉSUMÉ SÉLECTION").cyan().bold());
    println!("{}", style("═".repeat(68)).cyan());
    println!();
    println!("  Télémétrie:   {}", style(telem_count).green().bold());
    println!("  Privacy:      {}", style(privacy_count).green().bold());
    println!("  Performance:  {}", style(perf_count).green().bold());
    println!("  Scheduler:    {}", style(sched_count).green().bold());
    println!("  AppX:         {}", style(appx_count).green().bold());
    println!();
    println!("  {}: {} optimisations sélectionnées", 
        style("Total").bold(), 
        style(total).cyan().bold()
    );
}

/// Crée une barre de progression pour l'exécution
pub fn create_progress_bar(total: u64, multi: &MultiProgress) -> ProgressBar {
    let pb = multi.add(ProgressBar::new(total));
    pb.set_style(
        ProgressStyle::with_template(
            "  {spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}"
        )
        .unwrap()
        .progress_chars("█▓▒░")
    );
    pb
}

/// Crée un spinner pour une opération
#[allow(dead_code)]
pub fn create_spinner(multi: &MultiProgress, message: &str) -> ProgressBar {
    let sp = multi.add(ProgressBar::new_spinner());
    sp.set_style(
        ProgressStyle::with_template("  {spinner:.green} {msg}")
            .unwrap()
    );
    sp.set_message(message.to_string());
    sp
}

/// Affiche le résultat d'une opération
pub fn print_operation_result(name: &str, success: bool, message: &str) {
    if success {
        println!("  {} {} - {}", 
            style("[OK]").green().bold(), 
            name,
            style(message).dim()
        );
    } else {
        println!("  {} {} - {}", 
            style("[ERR]").red().bold(), 
            name,
            style(message).red()
        );
    }
}

/// Affiche le résultat final
pub fn print_final_result(success_count: usize, error_count: usize, snapshot_id: Option<&str>) {
    println!();
    println!("{}", style("═".repeat(68)).cyan());
    println!("{}", style("                      RÉSULTAT").cyan().bold());
    println!("{}", style("═".repeat(68)).cyan());
    println!();
    println!("  Succès:  {}", style(success_count).green().bold());
    println!("  Erreurs: {}", if error_count > 0 { 
        style(error_count).red().bold() 
    } else { 
        style(error_count).green().bold() 
    });
    
    if let Some(id) = snapshot_id {
        println!();
        println!("  Snapshot: {}", style(&id[..8]).dim());
    }
    
    println!();
    
    if error_count == 0 {
        println!("{}", style("  [OK] Toutes les modifications appliquées avec succès.").green().bold());
    } else {
        println!("{}", style("  [!] Certaines modifications ont échoué.").yellow().bold());
        println!("      Exécutez en tant qu'administrateur si nécessaire.");
    }
    
    println!();
    println!("  Pour annuler: {}", style("pieuvre rollback --last").cyan());
    println!("  Pour vérifier: {}", style("pieuvre status").cyan());
    println!();
}

/// Affiche un message d'annulation
pub fn print_cancelled() {
    println!();
    println!("{}", style("  [*] Annulé. Aucune modification effectuée.").yellow());
    println!();
}

/// Affiche un message quand aucune option sélectionnée
pub fn print_no_selection() {
    println!();
    println!("{}", style("  [*] Aucune optimisation sélectionnée. Fin.").yellow());
    println!();
}

/// Affiche un avertissement de sécurité pour la section Security
pub fn print_security_warning() {
    println!();
    println!("  {}", style("⚠️  ATTENTION: Options à risque de sécurité élevé").red().bold());
    println!("  {}", style("    Ces options réduisent la protection système.").red());
    println!("  {}", style("    À utiliser uniquement sur systèmes de gaming isolés.").red());
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
    let total = telem_count + privacy_count + perf_count + sched_count + appx_count
        + cpu_count + dpc_count + security_count + net_adv_count;
    
    println!();
    println!("{}", style("═".repeat(68)).cyan());
    println!("{}", style("                    RÉSUMÉ SÉLECTION").cyan().bold());
    println!("{}", style("═".repeat(68)).cyan());
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
        println!("  Security:        {}", style(security_count).green().bold());
    }
    println!("  Network Avancé:  {}", style(net_adv_count).green().bold());
    println!();
    println!("  {}: {} optimisations sélectionnées", 
        style("Total").bold(), 
        style(total).cyan().bold()
    );
    
    // Avertissement si options critiques
    if security_count > 0 {
        println!();
        println!("  {}", style("[!] Options de sécurité sélectionnées - Reboot requis").yellow().bold());
    }
    
    // Indicateur reboot si DPC ou security
    if dpc_count > 0 || security_count > 0 {
        println!("  {}", style("[!] Certaines options nécessitent un redémarrage").dim());
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
        println!("{}", style("  [!] REDÉMARRAGE RECOMMANDÉ pour appliquer toutes les modifications.").yellow().bold());
        println!();
    }
}

#[cfg(test)]
mod tests {
    // Tests UI sont difficiles à automatiser, on vérifie juste la compilation
    #[test]
    fn test_module_compiles() {
        // Si ce test compile, le module est syntaxiquement correct
        assert!(true);
    }
}
