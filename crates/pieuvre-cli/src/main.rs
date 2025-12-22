//! Pieuvre - Outil d'alignement système Windows 11
//!
//! Interface CLI principale SOTA 2026.

mod commands;

#[cfg(test)]
mod tests;

use clap::{Parser, Subcommand};
use clap_complete::Shell;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Parser)]
#[command(name = "pieuvre")]
#[command(author = "Pieuvre Contributors")]
#[command(version)]
#[command(about = "Outil SOTA d'alignement systeme Windows 11")]
#[command(long_about = "Outil SOTA d'alignement systeme Windows 11.\n\nLancez sans arguments pour le mode interactif guide.")]
struct Cli {
    /// Niveau de verbosité (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Effectue un audit du système
    Audit {
        /// Audit complet (tous les modules)
        #[arg(long)]
        full: bool,

        /// Fichier de sortie JSON
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Affiche les recommandations d'optimisation
    Analyze {
        /// Profil à utiliser (gaming, privacy, workstation)
        #[arg(short, long, default_value = "gaming")]
        profile: String,
    },

    /// Applique les optimisations
    Sync {
        /// Profil à appliquer
        #[arg(short, long)]
        profile: String,

        /// Mode simulation (aucune modification)
        #[arg(long)]
        dry_run: bool,
    },

    /// Affiche le statut actuel
    Status,

    /// Gère les snapshots et rollbacks
    Rollback {
        /// Liste les snapshots disponibles
        #[arg(long)]
        list: bool,

        /// Restaure le dernier snapshot
        #[arg(long)]
        last: bool,

        /// ID du snapshot à restaurer
        #[arg(long)]
        id: Option<String>,
    },

    /// Verifie l'integrite des optimisations
    Verify {
        /// Repare automatiquement les derives
        #[arg(long)]
        repair: bool,
    },

    /// Mode interactif - selection granulaire des optimisations
    Interactive {
        /// Profil de base (gaming, privacy, workstation)
        #[arg(short, long, default_value = "gaming")]
        profile: String,
    },

    /// Génère les scripts d'autocomplétion shell
    Completions {
        /// Shell cible (bash, zsh, fish, powershell, elvish)
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Configuration du logging
    let filter = match cli.verbose {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::new(filter))
        .init();

    match cli.command {
        // Mode interactif par defaut si aucune commande fournie
        None => commands::interactive::run_default().await,
        Some(Commands::Audit { full, output }) => commands::audit::run(full, output),
        Some(Commands::Analyze { profile }) => commands::analyze::run(&profile),
        Some(Commands::Sync { profile, dry_run }) => commands::sync::run(&profile, dry_run).await,
        Some(Commands::Status) => commands::status::run(),
        Some(Commands::Rollback { list, last, id }) => commands::rollback::run(list, last, id),
        Some(Commands::Verify { repair }) => commands::verify::run(repair),
        Some(Commands::Interactive { profile }) => commands::interactive::run(&profile).await,
        Some(Commands::Completions { shell }) => commands::completions::run(shell),
    }
}
