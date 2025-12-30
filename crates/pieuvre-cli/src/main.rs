//! pieuvre - Windows 11 system alignment tool
//!
//! Main CLI interface.

mod commands;

use pieuvre_common::Result;

#[cfg(test)]
mod tests;

use clap::{Parser, Subcommand};
use clap_complete::Shell;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Parser)]
#[command(name = "pieuvre")]
#[command(author = "pieuvre Contributors")]
#[command(version)]
#[command(about = "Windows 11 system alignment tool")]
#[command(
    long_about = "Windows 11 system alignment tool.\n\nRun without arguments for guided interactive mode."
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

    /// Manage specific system tweaks (SOTA v0.7.0)
    Tweak {
        #[command(subcommand)]
        action: TweakAction,
    },

    /// Generate shell completion scripts
    Completions {
        /// Target shell (bash, zsh, fish, powershell, elvish)
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(Subcommand)]
pub enum TweakAction {
    /// List all available tweaks by category
    List,
    /// Apply a specific tweak by its ID
    Apply {
        /// The ID of the tweak to apply (e.g., 'diagtrack', 'timer')
        id: String,
    },
    /// Apply all recommended SOTA optimizations
    ApplyAll,
}

#[tokio::main]
async fn main() -> Result<()> {
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
        // Launch SOTA Interactive Mode by default
        None => commands::interactive::tui::run().await,
        Some(Commands::Audit { full, output }) => {
            commands::audit::run(full, output, None).map(|_| ())
        }
        Some(Commands::Status { live }) => commands::status::run(live),
        Some(Commands::Rollback { list, last, id }) => commands::rollback::run(list, last, id),
        Some(Commands::Verify { repair }) => commands::verify::run(repair),

        Some(Commands::Interactive) => commands::interactive::tui::run().await,
        Some(Commands::Tweak { action }) => match action {
            TweakAction::List => {
                println!("Available SOTA Tweaks (v0.7.0):");
                for (section, items) in commands::interactive::sections::get_all_sections() {
                    println!("\n[{}]", section);
                    for item in items {
                        println!("  - {:<20} : {}", item.id, item.label);
                    }
                }
                Ok(())
            }
            TweakAction::Apply { id } => {
                let registry = commands::interactive::executor::CommandRegistry::new();
                match registry.execute(&id).await {
                    Ok(res) => {
                        println!("SUCCESS: {}", res.message);
                        Ok(())
                    }
                    Err(e) => {
                        eprintln!("ERROR: Failed to apply tweak '{}': {}", id, e);
                        Err(pieuvre_common::PieuvreError::Internal(e.to_string()))
                    }
                }
            }
            TweakAction::ApplyAll => {
                println!("Applying all recommended SOTA optimizations...");
                let registry = commands::interactive::executor::CommandRegistry::new();
                for (_, items) in commands::interactive::sections::get_all_sections() {
                    for item in items {
                        if item.default {
                            print!("Applying {}... ", item.id);
                            match registry.execute(item.id).await {
                                Ok(_) => println!("OK"),
                                Err(e) => println!("FAILED: {}", e),
                            }
                        }
                    }
                }
                Ok(())
            }
        },
        Some(Commands::Completions { shell }) => commands::completions::run(shell),
    }
}
