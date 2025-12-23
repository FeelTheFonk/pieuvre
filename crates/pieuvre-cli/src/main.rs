//! pieuvre - Windows 11 system alignment tool
//!
//! Main CLI interface.

mod commands;

#[cfg(test)]
mod tests;

use clap::{Parser, Subcommand};
use clap_complete::Shell;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Parser)]
#[command(name = "pieuvre")]
#[command(author = "pieuvre Contributors")]
#[command(version)]
#[command(about = "Advanced Windows 11 system alignment tool")]
#[command(
    long_about = "Advanced Windows 11 system alignment tool.\n\nRun without arguments for guided interactive mode."
)]
struct Cli {
    /// Verbosity level (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Perform a system audit
    Audit {
        /// Full audit (all modules)
        #[arg(long)]
        full: bool,

        /// JSON output file
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Display current status
    Status {
        /// Live mode (continuous refresh)
        #[arg(short, long)]
        live: bool,
    },

    /// Manage snapshots and rollbacks
    Rollback {
        /// List available snapshots
        #[arg(long)]
        list: bool,

        /// Restore the last snapshot
        #[arg(long)]
        last: bool,

        /// Snapshot ID to restore
        #[arg(long)]
        id: Option<String>,
    },

    /// Verify optimization integrity
    Verify {
        /// Automatically repair drifts
        #[arg(long)]
        repair: bool,
    },

    /// Interactive mode - granular optimization selection
    Interactive,

    /// Generate shell completion scripts
    Completions {
        /// Target shell (bash, zsh, fish, powershell, elvish)
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Logging configuration
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
        // Default to interactive mode if no command provided
        None => commands::interactive::dashboard::run_dashboard().await,
        Some(Commands::Audit { full, output }) => {
            commands::audit::run(full, output, None).map(|_| ())
        }
        Some(Commands::Status { live }) => commands::status::run(live),
        Some(Commands::Rollback { list, last, id }) => commands::rollback::run(list, last, id),
        Some(Commands::Verify { repair }) => commands::verify::run(repair),
        Some(Commands::Interactive) => commands::interactive::dashboard::run_dashboard().await,
        Some(Commands::Completions { shell }) => commands::completions::run(shell),
    }
}
