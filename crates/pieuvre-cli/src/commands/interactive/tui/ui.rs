use crate::commands::interactive::tui::app::AppState;
use crate::commands::interactive::tui::components::{Component, Footer, Header, MainView, Sidebar};
use crate::commands::interactive::tui::i18n;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render(f: &mut Frame, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Main content
            Constraint::Length(8), // Logs Panel
            Constraint::Length(3), // Footer
        ])
        .split(f.area());

    // Render Header
    let _ = Header.render(f, chunks[0], state);

    // Render Main Content (Sidebar + MainView)
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(25), // Sidebar
            Constraint::Min(0),     // MainView
        ])
        .split(chunks[1]);

    let _ = Sidebar.render(f, main_chunks[0], state);
    let _ = MainView.render(f, main_chunks[1], state);

    // Render Logs
    render_logs(f, chunks[2], state);

    // Render Footer
    let _ = Footer.render(f, chunks[3], state);
}

fn render_logs(f: &mut Frame, area: Rect, state: &AppState) {
    let logs: Vec<Line> = state
        .logs
        .iter()
        .rev()
        .map(|log| {
            let style = if log.contains(i18n::LOG_SUCCESS) {
                Style::default().fg(Color::Green)
            } else if log.contains(i18n::LOG_ERROR) {
                Style::default().fg(Color::Red)
            } else if log.contains(i18n::LOG_RUNNING) {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::Gray)
            };
            Line::from(Span::styled(log, style))
        })
        .collect();

    let logs_widget = Paragraph::new(logs)
        .block(
            Block::default()
                .title(i18n::LOGS_TITLE)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(logs_widget, area);
}
