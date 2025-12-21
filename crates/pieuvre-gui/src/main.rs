//! Pieuvre GUI — Point d'entrée principal

slint::include_modules!();

use anyhow::Result;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

fn main() -> Result<()> {
    // Configuration logging
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::new("info"))
        .init();

    tracing::info!("Démarrage Pieuvre GUI");

    // Création fenêtre principale
    let app = MainWindow::new()?;

    // Initialisation état
    pieuvre_gui::setup_initial_state(&app);

    // Configuration callbacks
    pieuvre_gui::setup_callbacks(&app);

    // Lancement event loop
    app.run()?;

    Ok(())
}
