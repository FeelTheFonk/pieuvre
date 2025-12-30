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

    // Render Confirmation Popup
    if state.show_confirm {
        render_confirm_popup(f, state);
    }
}

fn render_confirm_popup(f: &mut Frame, state: &AppState) {
    let area = f.area();
    let popup_area = Rect {
        x: area.width / 2 - 30,
        y: area.height / 2 - 5,
        width: 60,
        height: 10,
    };

    let active_tab_name = &state.tabs[state.active_tab];
    let is_scan_tab = active_tab_name == "Scan";
    let msg = if is_scan_tab {
        i18n::CONFIRM_SCAN_MSG
    } else {
        i18n::CONFIRM_APPLY_MSG
    };

    let block = Block::default()
        .title(i18n::CONFIRM_APPLY_TITLE)
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(Color::Cyan));

    let text = vec![
        Line::from(""),
        Line::from(Span::styled(msg, Style::default().fg(Color::White))),
        Line::from(""),
        Line::from(Span::styled(
            i18n::CONFIRM_KEYS,
            Style::default().fg(Color::Yellow),
        )),
    ];

    let paragraph = Paragraph::new(text)
        .block(block)
        .alignment(ratatui::layout::Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(ratatui::widgets::Clear, popup_area);
    f.render_widget(paragraph, popup_area);
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
