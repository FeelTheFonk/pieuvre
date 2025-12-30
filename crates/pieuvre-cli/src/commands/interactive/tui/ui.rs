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
    let mut constraints = vec![
        Constraint::Length(3), // Header
        Constraint::Min(0),    // Main content
        Constraint::Length(8), // Logs Panel
    ];
    
    if state.total_progress > 0 {
        constraints.push(Constraint::Length(3)); // Progress Bar
    }
    
    constraints.push(Constraint::Length(3)); // Footer

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
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

    // Render Progress Bar or Footer
    if state.total_progress > 0 {
        render_progress_bar(f, chunks[3], state);
        let _ = Footer.render(f, chunks[4], state);
    } else {
        let _ = Footer.render(f, chunks[3], state);
    }

    // Render Confirmation Popup
    if state.show_confirm {
        render_confirm_popup(f, state);
    }
}

fn render_progress_bar(f: &mut Frame, area: Rect, state: &AppState) {
    let gauge = ratatui::widgets::Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Color::Rgb(60, 60, 60))),
        )
        .gauge_style(Style::default().fg(Color::White).bg(Color::Rgb(30, 30, 30)))
        .percent(((state.progress as f32 / state.total_progress as f32) * 100.0) as u16)
        .label(format!("{} / {}", state.progress, state.total_progress));

    f.render_widget(gauge, area);
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
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(Color::White));

    let text = vec![
        Line::from(""),
        Line::from(Span::styled(msg, Style::default().fg(Color::Rgb(200, 200, 200)))),
        Line::from(""),
        Line::from(Span::styled(
            i18n::CONFIRM_KEYS,
            Style::default().fg(Color::Rgb(150, 150, 150)),
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
                Style::default().fg(Color::Rgb(200, 200, 200))
            } else if log.contains(i18n::LOG_ERROR) {
                Style::default().fg(Color::Rgb(150, 50, 50)) // Rouge très sombre/désaturé pour l'erreur
            } else if log.contains(i18n::LOG_RUNNING) {
                Style::default().fg(Color::Rgb(150, 150, 150))
            } else {
                Style::default().fg(Color::Rgb(80, 80, 80))
            };
            Line::from(Span::styled(log, style))
        })
        .collect();

    let logs_widget = Paragraph::new(logs)
        .block(
            Block::default()
                .title(Line::from(vec![
                    Span::styled(" [ ", Style::default().fg(Color::Gray)),
                    Span::styled(i18n::LOGS_TITLE, Style::default().fg(Color::White).add_modifier(ratatui::style::Modifier::BOLD)),
                    Span::styled(" ] ", Style::default().fg(Color::Gray)),
                ]))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(logs_widget, area);
}
