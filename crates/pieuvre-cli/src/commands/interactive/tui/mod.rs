use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use pieuvre_common::Result;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::Duration;
use tokio::sync::mpsc;

pub mod app;
pub mod components;
pub mod events;
pub mod i18n;
pub mod ui;

use crate::commands::interactive::tui::app::{Action, AppState, SystemMetrics};
use crate::commands::interactive::tui::events::{Event, EventHandler};

pub async fn run() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initialize state, events and CommandRegistry (SOTA v0.7.0)
    let mut app = AppState::new();
    let registry =
        std::sync::Arc::new(crate::commands::interactive::executor::CommandRegistry::new());
    let (action_tx, mut action_rx) = mpsc::unbounded_channel();
    app.set_action_tx(action_tx.clone());

    let mut events = EventHandler::new(Duration::from_millis(50));

    // Metrics Task
    let metrics_tx = action_tx.clone();
    tokio::spawn(async move {
        let mut sys = sysinfo::System::new_all();
        loop {
            sys.refresh_cpu_usage();
            sys.refresh_memory();
            let metrics = SystemMetrics {
                cpu_usage: sys.global_cpu_info().cpu_usage(),
                mem_used_gb: sys.used_memory() as f32 / 1024.0 / 1024.0 / 1024.0,
                mem_total_gb: sys.total_memory() as f32 / 1024.0 / 1024.0 / 1024.0,
                uptime: sysinfo::System::uptime(),
            };
            let _ = metrics_tx.send(Action::UpdateMetrics(metrics));
            tokio::time::sleep(Duration::from_millis(1000)).await;
        }
    });

    while !app.should_quit {
        // Draw UI
        terminal
            .draw(|f| ui::render(f, &app))
            .map_err(|e| pieuvre_common::PieuvreError::Tui(e.to_string()))?;

        // Handle events and actions
        tokio::select! {
            Some(event) = events.next() => {
                match event {
                    Event::Tick => app.dispatch(Action::Tick),
                    Event::Key(key) => {
                        use crossterm::event::KeyCode;
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => app.dispatch(Action::Quit),
                            KeyCode::Tab | KeyCode::Right => app.dispatch(Action::NextTab),
                            KeyCode::BackTab | KeyCode::Left => app.dispatch(Action::PrevTab),
                            KeyCode::Up => app.dispatch(Action::PrevItem),
                            KeyCode::Down => app.dispatch(Action::NextItem),
                            KeyCode::Char(' ') => app.dispatch(Action::ToggleSelected),
                            KeyCode::Enter => app.dispatch(Action::Execute),
                            _ => {}
                        }
                    }
                    Event::Resize(w, h) => {
                        let _ = terminal.resize(ratatui::layout::Rect::new(0, 0, w, h));
                    }
                }
            }
            Some(action) = action_rx.recv() => {
                match action {
                    Action::Execute => {
                        let options_to_run: Vec<(String, String)> = app.tabs.iter().flat_map(|tab| {
                            app.tab_options.get(tab).unwrap_or(&vec![]).iter()
                                .filter(|opt| *app.options_state.get(opt.id as &str).unwrap_or(&false))
                                .map(|opt| (opt.id.to_string(), opt.label.to_string()))
                                .collect::<Vec<_>>()
                        }).collect();

                        let log_tx = action_tx.clone();
                        let reg = registry.clone();
                        tokio::spawn(async move {
                            for (id, label) in options_to_run {
                                let _ = log_tx.send(Action::AddLog(format!("{} Applying {}...", i18n::LOG_RUNNING, label)));
                                match reg.execute(&id).await {
                                    Ok(res) => {
                                        let _ = log_tx.send(Action::AddLog(format!("{} {}: {}", i18n::LOG_SUCCESS, label, res.message)));
                                    }
                                    Err(e) => {
                                        let _ = log_tx.send(Action::AddLog(format!("{} {}: {}", i18n::LOG_ERROR, label, e)));
                                    }
                                }
                            }
                            let _ = log_tx.send(Action::AddLog(i18n::LOG_ALL_APPLIED.to_string()));
                        });
                    }
                    _ => app.update(action),
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
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
