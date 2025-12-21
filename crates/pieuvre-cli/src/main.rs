//! Pieuvre - Outil d'alignement système Windows 11
//!
//! Interface CLI principale.

mod commands;

use clap::{Parser, Subcommand};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Parser)]
#[command(name = "pieuvre")]
#[command(author = "Pieuvre Contributors")]
#[command(version)]
#[command(about = "Outil SOTA d'alignement système Windows 11", long_about = None)]
struct Cli {
    /// Niveau de verbosité (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Commands,
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
        #[arg(short, long, default_value = "balanced")]
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

    /// Vérifie l'intégrité des optimisations
    Verify {
        /// Répare automatiquement les dérives
        #[arg(long)]
        repair: bool,
    },
}

fn main() -> anyhow::Result<()> {
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
        Commands::Audit { full, output } => commands::audit::run(full, output),
        Commands::Analyze { profile } => commands::analyze::run(&profile),
        Commands::Sync { profile, dry_run } => commands::sync::run(&profile, dry_run),
        Commands::Status => commands::status::run(),
        Commands::Rollback { list, last, id } => commands::rollback::run(list, last, id),
        Commands::Verify { repair } => commands::verify::run(repair),
    }
}
