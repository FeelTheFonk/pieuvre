use crate::commands::interactive::tui::app::AppState;
use crate::commands::interactive::tui::components::Component;
use crate::commands::interactive::tui::i18n;
use anyhow::Result;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem},
    Frame,
};

pub struct Sidebar;

impl Component for Sidebar {
    fn render(&self, f: &mut Frame, area: Rect, state: &AppState) -> Result<()> {
        let items: Vec<ListItem> = state
            .tabs
            .iter()
            .enumerate()
            .map(|(i, tab)| {
                let style = if i == state.active_tab {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Gray)
                };
                ListItem::new(format!("  {}  ", tab)).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title(i18n::CATEGORIES)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::DarkGray)),
            )
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));

        f.render_widget(list, area);
        Ok(())
    }
}
