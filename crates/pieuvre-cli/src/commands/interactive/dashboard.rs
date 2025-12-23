use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::{Duration, Instant};

use crate::commands::interactive::ui::{draw_ui, AppState, LogLevel, ViewMode};
use tokio::sync::mpsc;

enum LogMessage {
    Log {
        level: LogLevel,
        message: String,
        details: Option<String>,
    },
    Finished,
}

pub async fn run_dashboard() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut state = AppState::new();
    state.add_log(LogLevel::Info, "Optimization engine initialized", None);
    state.add_log(LogLevel::Info, "Waiting for user input...", None);

    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    let (tx, mut rx) = mpsc::channel::<LogMessage>(100);
    let mut is_executing = false;

    loop {
        // Handle incoming logs
        while let Ok(msg) = rx.try_recv() {
            match msg {
                LogMessage::Log {
                    level,
                    message,
                    details,
                } => {
                    state.add_log(level, &message, details.as_deref());
                }
                LogMessage::Finished => {
                    is_executing = false;
                }
            }
        }

        terminal.draw(|f| draw_ui(f, &state))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Esc => {
                            if state.current_view != ViewMode::Dashboard {
                                state.current_view = ViewMode::Dashboard;
                                state.add_log(LogLevel::Info, "Returned to Dashboard", None);
                            }
                        }
                        KeyCode::Up => {
                            if state.current_view == ViewMode::Dashboard
                                && state.selected_option > 0
                            {
                                state.selected_option -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if state.current_view == ViewMode::Dashboard {
                                let (_, options) = &state.categories[state.selected_category];
                                if state.selected_option < options.len() - 1 {
                                    state.selected_option += 1;
                                }
                            }
                        }
                        KeyCode::Left => {
                            if state.current_view == ViewMode::Dashboard
                                && state.selected_category > 0
                            {
                                state.selected_category -= 1;
                                state.selected_option = 0;
                            }
                        }
                        KeyCode::Right => {
                            if state.current_view == ViewMode::Dashboard
                                && state.selected_category < state.categories.len() - 1
                            {
                                state.selected_category += 1;
                                state.selected_option = 0;
                            }
                        }
                        KeyCode::Char(' ') => {
                            if state.current_view == ViewMode::Dashboard {
                                let (_, options) = &state.categories[state.selected_category];
                                let opt_id = options[state.selected_option].id.to_string();
                                let current_state =
                                    state.options_state.get(&opt_id).cloned().unwrap_or(false);
                                state.options_state.insert(opt_id, !current_state);
                            }
                        }
                        KeyCode::Enter => {
                            if state.current_view == ViewMode::Dashboard && !is_executing {
                                let mut to_execute = Vec::new();
                                for (cat_name, section) in &state.categories {
                                    for opt in section {
                                        if state.options_state.get(opt.id).cloned().unwrap_or(false)
                                        {
                                            to_execute.push((cat_name.to_string(), opt.clone()));
                                        }
                                    }
                                }

                                if !to_execute.is_empty() {
                                    is_executing = true;
                                    state.current_view = ViewMode::Audit;
                                    let tx_clone = tx.clone();
                                    tokio::spawn(async move {
                                        let _ = execute_batch_async(to_execute, tx_clone).await;
                                    });
                                } else {
                                    state.add_log(LogLevel::Warning, "No options selected", None);
                                }
                            }
                        }
                        KeyCode::Char('a') => {
                            if state.current_view == ViewMode::Dashboard {
                                state.select_all_in_category();
                                state.add_log(LogLevel::Info, "Selected all in category", None);
                            } else {
                                state.current_view = ViewMode::Audit;
                                state.add_log(LogLevel::Info, "Starting System Audit...", None);
                                terminal.draw(|f| draw_ui(f, &state))?;

                                let mut log_cb = |level: &str, msg: &str| {
                                    let lvl = match level {
                                        "SUCCESS" => LogLevel::Success,
                                        "WARNING" => LogLevel::Warning,
                                        "ERROR" => LogLevel::Error,
                                        _ => LogLevel::Info,
                                    };
                                    state.add_log(lvl, msg, None);
                                };

                                if let Err(e) =
                                    crate::commands::audit::run(false, None, Some(&mut log_cb))
                                {
                                    state.add_log(
                                        LogLevel::Error,
                                        &format!("Audit failed: {}", e),
                                        None,
                                    );
                                }
                            }
                        }
                        KeyCode::Char('d') => {
                            if state.current_view == ViewMode::Dashboard {
                                state.deselect_all_in_category();
                                state.add_log(LogLevel::Info, "Deselected all in category", None);
                            }
                        }
                        KeyCode::Char('c') => {
                            if state.current_view == ViewMode::Dashboard && !is_executing {
                                state.select_all_in_category();
                                state.add_log(
                                    LogLevel::Info,
                                    "Executing current category...",
                                    None,
                                );

                                let mut to_execute = Vec::new();
                                let (cat_name, options) =
                                    &state.categories[state.selected_category];
                                for opt in options {
                                    to_execute.push((cat_name.to_string(), opt.clone()));
                                }

                                if !to_execute.is_empty() {
                                    is_executing = true;
                                    state.current_view = ViewMode::Audit;
                                    let tx_clone = tx.clone();
                                    tokio::spawn(async move {
                                        let _ = execute_batch_async(to_execute, tx_clone).await;
                                    });
                                }
                            }
                        }
                        KeyCode::Char('r') => {
                            if state.current_view == ViewMode::Dashboard {
                                state.add_log(
                                    LogLevel::Warning,
                                    "Rollback menu integration pending",
                                    None,
                                );
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

async fn execute_batch_async(
    to_execute: Vec<(String, super::sections::OptItem)>,
    tx: mpsc::Sender<LogMessage>,
) -> Result<()> {
    let _ = tx
        .send(LogMessage::Log {
            level: LogLevel::Info,
            message: format!(
                "Starting execution of {} optimizations...",
                to_execute.len()
            ),
            details: None,
        })
        .await;

    for (cat, opt) in to_execute {
        execute_single_opt_async(&cat, &opt, tx.clone()).await?;
    }

    let _ = tx
        .send(LogMessage::Log {
            level: LogLevel::Success,
            message: "Batch execution completed".to_string(),
            details: None,
        })
        .await;

    let _ = tx.send(LogMessage::Finished).await;
    Ok(())
}

async fn execute_single_opt_async(
    cat: &str,
    opt: &super::sections::OptItem,
    tx: mpsc::Sender<LogMessage>,
) -> Result<()> {
    let _ = tx
        .send(LogMessage::Log {
            level: LogLevel::Info,
            message: format!("Applying: {} [{}]", opt.label, cat),
            details: None,
        })
        .await;

    match crate::commands::interactive::executor::get_executor(cat) {
        Ok(executor) => {
            let mut changes = Vec::new();
            match executor.execute(opt.id, &mut changes).await {
                Ok(res) => {
                    let _ = tx
                        .send(LogMessage::Log {
                            level: LogLevel::Success,
                            message: format!("Done: {}", res.message),
                            details: None,
                        })
                        .await;

                    for change in changes {
                        let detail = match change {
                            pieuvre_common::ChangeRecord::Registry {
                                key,
                                value_name,
                                original_data,
                                ..
                            } => format!(
                                "REG: {}\\{} (was {} bytes)",
                                key,
                                value_name,
                                original_data.len()
                            ),
                            pieuvre_common::ChangeRecord::Service {
                                name,
                                original_start_type,
                            } => format!("SVC: {} (was start type {})", name, original_start_type),
                            _ => format!("{:?}", change),
                        };
                        let _ = tx
                            .send(LogMessage::Log {
                                level: LogLevel::Info,
                                message: format!("  └─ {}", detail),
                                details: None,
                            })
                            .await;
                    }
                }
                Err(e) => {
                    let _ = tx
                        .send(LogMessage::Log {
                            level: LogLevel::Error,
                            message: format!("Failed: {} - {}", opt.label, e),
                            details: None,
                        })
                        .await;
                }
            }
        }
        Err(e) => {
            let _ = tx
                .send(LogMessage::Log {
                    level: LogLevel::Error,
                    message: format!("Dispatch error: {}", e),
                    details: None,
                })
                .await;
        }
    }
    Ok(())
}

// Suppression des anciennes fonctions synchrones
