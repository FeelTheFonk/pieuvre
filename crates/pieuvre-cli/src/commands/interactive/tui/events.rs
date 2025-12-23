use crossterm::event::{self, Event as CrosstermEvent, KeyEvent};
use futures::{FutureExt, StreamExt};
use std::time::Duration;
use tokio::sync::mpsc;

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Resize(u16, u16),
}

pub struct EventHandler {
    rx: mpsc::UnboundedReceiver<Event>,
    _handler: tokio::task::JoinHandle<()>,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let _tx = tx.clone();
        let _handler = tokio::spawn(async move {
            let mut reader = event::EventStream::new();
            let mut tick = tokio::time::interval(tick_rate);
            let mut last_key_time: Option<std::time::Instant> = None;

            loop {
                let tick_delay = tick.tick();
                let crossterm_event = reader.next().fuse();
                tokio::select! {
                    _ = tick_delay => {
                        if _tx.send(Event::Tick).is_err() {
                            break;
                        }
                    }
                    Some(Ok(evt)) = crossterm_event => {
                        match evt {
                            CrosstermEvent::Key(key) => {
                                if key.kind != event::KeyEventKind::Press {
                                    continue;
                                }
                                let now = std::time::Instant::now();
                                if let Some(last) = last_key_time {
                                    // Debounce: 150ms pour garantir la stabilité sur toutes les consoles
                                    if now.duration_since(last) < Duration::from_millis(150) {
                                        continue;
                                    }
                                }
                                last_key_time = Some(now);
                                if _tx.send(Event::Key(key)).is_err() {
                                    break;
                                }
                            }
                            CrosstermEvent::Resize(x, y) => {
                                if _tx.send(Event::Resize(x, y)).is_err() {
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                };
            }
        });
        Self { rx, _handler }
    }

    pub async fn next(&mut self) -> Option<Event> {
        self.rx.recv().await
    }
}
