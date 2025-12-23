use crate::commands::interactive::tui::app::{Action, AppState};
use anyhow::Result;
use ratatui::{layout::Rect, Frame};

pub mod footer;
pub mod header;
pub mod main_view;
pub mod sidebar;

pub use footer::Footer;
pub use header::Header;
pub use main_view::MainView;
pub use sidebar::Sidebar;

/// Trait de base pour tous les composants de l'UI
pub trait Component {
    /// Gère les actions et met à jour l'état interne si nécessaire
    #[allow(dead_code)]
    fn handle_action(&mut self, _action: &Action) -> Result<Option<Action>> {
        Ok(None)
    }

    /// Rendu du composant sur le frame
    fn render(&self, f: &mut Frame, area: Rect, state: &AppState) -> Result<()>;
}
