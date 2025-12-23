use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use std::collections::VecDeque;

// ══════════════════════════════════════════════════════════════════════════════
// TUI DASHBOARD ENGINE
// ══════════════════════════════════════════════════════════════════════════════

pub struct AppState {
    pub logs: VecDeque<LogEntry>,
    pub is_laptop: bool,
    pub admin_status: bool,
    pub current_view: ViewMode,
    // Navigation granulaire
    pub categories: Vec<(&'static str, Vec<super::sections::OptItem>)>,
    pub selected_category: usize,
    pub selected_option: usize,
    pub options_state: std::collections::HashMap<String, bool>,
}

#[derive(PartialEq, Debug)]
pub enum ViewMode {
    Dashboard,
    Audit,
}

pub struct LogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub message: String,
    pub details: Option<String>,
}

pub enum LogLevel {
    Info,
    Success,
    Warning,
    Error,
}

impl AppState {
    pub fn new() -> Self {
        let is_laptop = pieuvre_audit::hardware::is_laptop();
        let categories = vec![
            ("Telemetry", super::sections::telemetry_section()),
            ("Privacy", super::sections::privacy_section()),
            (
                "Performance",
                super::sections::performance_section(is_laptop),
            ),
            ("Scheduler", super::sections::scheduler_section()),
            ("AppX Bloat", super::sections::appx_section()),
            ("CPU/Mem", super::sections::cpu_section(is_laptop)),
            ("DPC Latency", super::sections::dpc_section()),
            ("Security", super::sections::security_section()),
            ("Network", super::sections::network_advanced_section()),
            ("DNS", super::sections::dns_section()),
            ("Cleanup", super::sections::cleanup_section()),
        ];

        let mut options_state = std::collections::HashMap::new();
        for (_, section) in &categories {
            for opt in section {
                options_state.insert(opt.id.to_string(), opt.default);
            }
        }

        Self {
            logs: VecDeque::with_capacity(200),
            is_laptop,
            admin_status: is_elevated(),
            current_view: ViewMode::Dashboard,
            categories,
            selected_category: 0,
            selected_option: 0,
            options_state,
        }
    }

    pub fn add_log(&mut self, level: LogLevel, message: &str, details: Option<&str>) {
        let timestamp = chrono::Local::now().format("%H:%M:%S").to_string();
        if self.logs.len() >= 200 {
            self.logs.pop_front();
        }
        self.logs.push_back(LogEntry {
            timestamp,
            level,
            message: message.to_string(),
            details: details.map(|s| s.to_string()),
        });
    }

    pub fn select_all_in_category(&mut self) {
        let (_, options) = &self.categories[self.selected_category];
        for opt in options {
            self.options_state.insert(opt.id.to_string(), true);
        }
    }

    pub fn deselect_all_in_category(&mut self) {
        let (_, options) = &self.categories[self.selected_category];
        for opt in options {
            self.options_state.insert(opt.id.to_string(), false);
        }
    }
}

pub fn draw_ui(f: &mut Frame, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(10),   // Main Content (Categories + Options)
            Constraint::Length(3), // Footer / Help
        ])
        .split(f.size());

    draw_header(f, chunks[0], state);
    draw_main_content(f, chunks[1], state);
    draw_footer(f, chunks[2], state);
}

fn draw_main_content(f: &mut Frame, area: Rect, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30), // Categories
            Constraint::Percentage(70), // Options / Logs
        ])
        .split(area);

    draw_categories(f, chunks[0], state);

    if state.current_view == ViewMode::Dashboard {
        draw_options(f, chunks[1], state);
    } else {
        draw_logs(f, chunks[1], state);
    }
}

fn draw_header(f: &mut Frame, area: Rect, state: &AppState) {
    let admin_str = if state.admin_status { "ADMIN" } else { "USER" };
    let admin_color = if state.admin_status {
        Color::Cyan
    } else {
        Color::Yellow
    };

    let header_text = vec![Line::from(vec![
        Span::styled(
            " PIEUVRE ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" │ "),
        Span::styled(admin_str, Style::default().fg(admin_color)),
        Span::raw(" │ "),
        Span::styled(
            if state.is_laptop { "LAPTOP" } else { "DESKTOP" },
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        ),
        Span::raw(" │ "),
        Span::styled(
            format!("{:?}", state.current_view).to_uppercase(),
            Style::default().fg(Color::Cyan),
        ),
    ])];

    let header = Paragraph::new(header_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );
    f.render_widget(header, area);
}

fn draw_categories(f: &mut Frame, area: Rect, state: &AppState) {
    let items: Vec<ListItem> = state
        .categories
        .iter()
        .enumerate()
        .map(|(i, (name, _))| {
            let is_selected = i == state.selected_category;
            let style = if is_selected {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let prefix = if is_selected { "» " } else { "  " };
            let name_str = name.to_string().to_uppercase();

            ListItem::new(Line::from(vec![
                Span::styled(prefix.to_string(), style),
                Span::styled(name_str, style),
            ]))
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title(" CATEGORIES ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );
    f.render_widget(list, area);
}

fn draw_options(f: &mut Frame, area: Rect, state: &AppState) {
    let (cat_name, options) = &state.categories[state.selected_category];
    let items: Vec<ListItem> = options
        .iter()
        .enumerate()
        .map(|(i, opt)| {
            let is_selected = i == state.selected_option;
            let is_checked = state.options_state.get(opt.id).cloned().unwrap_or(false);

            let checkbox = if is_checked { "[x]" } else { "[ ]" };
            let check_color = if is_checked {
                Color::Cyan
            } else {
                Color::DarkGray
            };

            let style = if is_selected {
                Style::default().fg(Color::White).bg(Color::DarkGray)
            } else {
                Style::default().fg(Color::White)
            };

            let risk_color = match opt.risk {
                super::sections::RiskLevel::Safe => Color::Green,
                super::sections::RiskLevel::Conditional => Color::Yellow,
                super::sections::RiskLevel::Performance => Color::Cyan,
                super::sections::RiskLevel::Warning => Color::Red,
                super::sections::RiskLevel::Critical => Color::Magenta,
            };

            ListItem::new(vec![Line::from(vec![
                Span::styled(format!(" {} ", checkbox), Style::default().fg(check_color)),
                Span::styled(opt.label.to_string(), style),
                Span::raw(" "),
                Span::styled(
                    format!("({:?})", opt.risk),
                    Style::default()
                        .fg(risk_color)
                        .add_modifier(Modifier::ITALIC),
                ),
            ])])
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title(format!(" {} OPTIONS ", cat_name.to_uppercase()))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );
    f.render_widget(list, area);
}

fn draw_logs(f: &mut Frame, area: Rect, state: &AppState) {
    let logs: Vec<ListItem> = state
        .logs
        .iter()
        .rev()
        .map(|log| {
            let (icon, color) = match log.level {
                LogLevel::Info => ("●", Color::Cyan),
                LogLevel::Success => ("●", Color::Green),
                LogLevel::Warning => ("!", Color::Yellow),
                LogLevel::Error => ("○", Color::Red),
            };

            let mut lines = vec![Line::from(vec![
                Span::styled(
                    format!(" {} ", log.timestamp),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(icon, Style::default().fg(color)),
                Span::raw(" "),
                Span::styled(
                    &log.message,
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
            ])];

            if let Some(details) = &log.details {
                lines.push(Line::from(vec![
                    Span::raw("      "),
                    Span::styled("└─ ", Style::default().fg(Color::DarkGray)),
                    Span::styled(details, Style::default().fg(Color::DarkGray)),
                ]));
            }

            ListItem::new(lines)
        })
        .collect();

    let log_list = List::new(logs).block(
        Block::default()
            .title(Span::styled(
                " SYSTEM LOGS ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );

    f.render_widget(log_list, area);
}

fn draw_footer(f: &mut Frame, area: Rect, state: &AppState) {
    let help_text = match state.current_view {
        ViewMode::Dashboard => vec![
            Span::styled(" Q ", Style::default().fg(Color::Black).bg(Color::DarkGray)),
            Span::raw(" Quit │ "),
            Span::styled(
                " ↑↓←→ ",
                Style::default().fg(Color::Black).bg(Color::DarkGray),
            ),
            Span::raw(" Navigate │ "),
            Span::styled(
                " SPACE ",
                Style::default().fg(Color::Black).bg(Color::DarkGray),
            ),
            Span::raw(" Toggle │ "),
            Span::styled(" A ", Style::default().fg(Color::Black).bg(Color::DarkGray)),
            Span::raw(" Select All │ "),
            Span::styled(" D ", Style::default().fg(Color::Black).bg(Color::DarkGray)),
            Span::raw(" Deselect All │ "),
            Span::styled(" C ", Style::default().fg(Color::Black).bg(Color::Cyan)),
            Span::raw(" Quick Apply │ "),
            Span::styled(" ENTER ", Style::default().fg(Color::Black).bg(Color::Cyan)),
            Span::raw(" Run Batch "),
        ],
        _ => vec![
            Span::styled(
                " ESC ",
                Style::default().fg(Color::Black).bg(Color::DarkGray),
            ),
            Span::raw(" Back to Dashboard "),
        ],
    };

    let footer = Paragraph::new(Line::from(help_text))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(footer, area);
}

pub fn is_elevated() -> bool {
    #[cfg(windows)]
    {
        use windows::Win32::Foundation::HANDLE;
        use windows::Win32::Security::{
            GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY,
        };
        use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};
        unsafe {
            let mut token = HANDLE::default();
            if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).is_err() {
                return false;
            }
            let mut elevation = TOKEN_ELEVATION::default();
            let mut size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;
            let result = GetTokenInformation(
                token,
                TokenElevation,
                Some(&mut elevation as *mut _ as *mut _),
                size,
                &mut size,
            );
            let _ = windows::Win32::Foundation::CloseHandle(token);
            result.is_ok() && elevation.TokenIsElevated != 0
        }
    }
    #[cfg(not(windows))]
    false
}
