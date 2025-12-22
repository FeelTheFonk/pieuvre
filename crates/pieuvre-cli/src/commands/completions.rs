//! Génération de shell completions SOTA
//!
//! Support Bash, Zsh, Fish, PowerShell, Elvish.

use clap::CommandFactory;
use clap_complete::{generate, Shell};
use std::io;

use crate::Cli;

/// Génère les completions pour le shell spécifié
pub fn run(shell: Shell) -> anyhow::Result<()> {
    let mut cmd = Cli::command();
    let name = cmd.get_name().to_string();

    generate(shell, &mut cmd, name, &mut io::stdout());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap_complete::Shell;

    #[test]
    fn test_shell_variants_exist() {
        // Vérifier que tous les shells sont supportés
        let shells = [
            Shell::Bash,
            Shell::Zsh,
            Shell::Fish,
            Shell::PowerShell,
            Shell::Elvish,
        ];

        for shell in shells {
            assert!(!format!("{:?}", shell).is_empty());
        }
    }

    #[test]
    fn test_command_factory() {
        // Vérifier que Cli::command() fonctionne
        let cmd = Cli::command();
        assert_eq!(cmd.get_name(), "pieuvre");
    }
}
