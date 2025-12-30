use crate::commands::interactive::tui::app::AppState;
use crate::commands::interactive::tui::components::Component;
use crate::commands::interactive::tui::i18n;
use crate::commands::interactive::types::RiskLevel;
use anyhow::Result;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub struct MainView;

impl Component for MainView {
    fn render(&self, f: &mut Frame, area: Rect, state: &AppState) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // List
                Constraint::Percentage(50), // Details
            ])
            .split(area);

        // List of options
        let options = state.current_options();
        let items: Vec<ListItem> = options
            .iter()
            .enumerate()
            .map(|(i, opt)| {
                let is_selected = state
                    .options_state
                    .get(opt.id as &str)
                    .cloned()
                    .unwrap_or(false);
                let prefix = if is_selected { " [X] " } else { " [ ] " };

                let style = if i == state.selected_index {
                    Style::default().fg(Color::Black).bg(Color::Cyan)
                } else {
                    Style::default().fg(Color::White)
                };

                ListItem::new(format!("{}{}", prefix, opt.label)).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title(i18n::OPTIMIZATIONS)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::DarkGray)),
            )
            .highlight_symbol(">> ");

        f.render_widget(list, chunks[0]);

        // Details
        if let Some(opt) = options.get(state.selected_index) {
            let risk_color = match opt.risk {
                RiskLevel::Safe => Color::Green,
                RiskLevel::Low => Color::Cyan,
                RiskLevel::Medium => Color::Yellow,
                RiskLevel::High => Color::Red,
                RiskLevel::Critical => Color::LightRed,
                RiskLevel::Performance => Color::Blue,
                RiskLevel::Conditional => Color::Magenta,
                RiskLevel::Warning => Color::LightYellow,
            };

            let details_text = vec![
                Line::from(vec![
                    Span::styled(i18n::ID, Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        opt.id,
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    Span::styled(i18n::RISK, Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        format!("{:?}", opt.risk).to_uppercase(),
                        Style::default().fg(risk_color).add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(""),
                Line::from(Span::styled(
                    i18n::DESCRIPTION,
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::UNDERLINED),
                )),
                Line::from(""),
                Line::from(opt.description),
            ];

            let details = Paragraph::new(details_text)
                .block(
                    Block::default()
                        .title(i18n::DETAILS)
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(Style::default().fg(Color::DarkGray)),
                )
                .wrap(Wrap { trim: true });

            f.render_widget(details, chunks[1]);
        }

        Ok(())
    }
}
