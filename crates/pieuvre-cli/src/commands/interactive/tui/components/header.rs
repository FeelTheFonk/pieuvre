use crate::commands::interactive::tui::app::AppState;
use crate::commands::interactive::tui::components::Component;
use crate::commands::interactive::tui::i18n;
use anyhow::Result;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub struct Header;

impl Component for Header {
    fn render(&self, f: &mut Frame, area: Rect, state: &AppState) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(20), // Title
                Constraint::Min(0),     // Spacer
                Constraint::Length(40), // Metrics
                Constraint::Length(15), // Mode
            ])
            .split(area);

        // Title
        let title = Paragraph::new(Line::from(vec![
            Span::styled(
                i18n::TITLE,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(i18n::VERSION, Style::default().fg(Color::Rgb(100, 100, 100))),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Color::Rgb(60, 60, 60))),
        );
        f.render_widget(title, chunks[0]);

        // Metrics (Sobriété : pas de couleurs de seuil vives)
        let cpu_color = Color::Rgb(200, 200, 200);
        let mem_color = Color::Rgb(200, 200, 200);

        let metrics = Paragraph::new(Line::from(vec![
            Span::styled(i18n::CPU, Style::default().fg(Color::Rgb(100, 100, 100))),
            Span::styled(
                format!("{:.1}% ", state.metrics.cpu_usage),
                Style::default().fg(cpu_color).add_modifier(Modifier::BOLD),
            ),
            Span::styled(i18n::MEM, Style::default().fg(Color::Rgb(100, 100, 100))),
            Span::styled(
                format!(
                    "{:.1}/{:.1} GB ",
                    state.metrics.mem_used_gb, state.metrics.mem_total_gb
                ),
                Style::default().fg(mem_color).add_modifier(Modifier::BOLD),
            ),
            Span::styled(i18n::UPTIME, Style::default().fg(Color::Rgb(100, 100, 100))),
            Span::styled(
                format!("{}s ", state.metrics.uptime),
                Style::default().fg(Color::Rgb(150, 150, 150)),
            ),
        ]))
        .alignment(Alignment::Right)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Color::Rgb(60, 60, 60))),
        );
        f.render_widget(metrics, chunks[2]);

        // Mode
        let admin_style = if state.is_admin {
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Rgb(150, 50, 50)).add_modifier(Modifier::BOLD)
        };

        let mode = Paragraph::new(Line::from(vec![Span::styled(
            if state.is_admin {
                i18n::ADMIN
            } else {
                i18n::USER
            },
            admin_style,
        )]))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Color::Rgb(60, 60, 60))),
        );
        f.render_widget(mode, chunks[3]);

        Ok(())
    }
}
