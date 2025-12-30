use crate::commands::interactive::types::OptItem;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct SystemMetrics {
    pub cpu_usage: f32,
    pub mem_used_gb: f32,
    pub mem_total_gb: f32,
    pub uptime: u64,
}

#[derive(Debug, Clone)]
pub enum Action {
    Tick,
    Quit,
    NextTab,
    PrevTab,
    NextItem,
    PrevItem,
    ToggleSelected,
    Execute,
    AddLog(String),
    UpdateMetrics(SystemMetrics),
}

pub struct AppState {
    pub should_quit: bool,
    pub active_tab: usize,
    pub selected_index: usize,
    pub tabs: Vec<String>,
    pub tab_options: HashMap<String, Vec<OptItem>>,
    pub options_state: HashMap<String, bool>,
    pub metrics: SystemMetrics,
    #[allow(dead_code)]
    pub logs: Vec<String>,
    pub is_admin: bool,
    pub action_tx: Option<tokio::sync::mpsc::UnboundedSender<Action>>,
}

impl AppState {
    pub fn new() -> Self {
        let _is_laptop = pieuvre_audit::hardware::is_laptop();
        let mut tab_options = HashMap::new();
        let mut options_state = HashMap::new();
        let mut tabs = Vec::new();

        // Nouveau système modulaire SOTA
        for (name, items) in crate::commands::interactive::sections::get_all_sections() {
            let name_str: &str = name;
            tabs.push(name_str.to_string());
            for item in &items {
                options_state.insert(item.id.to_string(), item.default);
            }
            tab_options.insert(name.to_string(), items);
        }

        Self {
            should_quit: false,
            active_tab: 0,
            selected_index: 0,
            tabs,
            tab_options,
            options_state,
            metrics: SystemMetrics::default(),
            logs: Vec::new(),
            is_admin: crate::commands::interactive::tui::is_elevated(),
            action_tx: None,
        }
    }

    pub fn set_action_tx(&mut self, tx: tokio::sync::mpsc::UnboundedSender<Action>) {
        self.action_tx = Some(tx);
    }

    pub fn dispatch(&self, action: Action) {
        if let Some(tx) = &self.action_tx {
            let _ = tx.send(action);
        }
    }

    pub fn current_options(&self) -> &[OptItem] {
        let cat = &self.tabs[self.active_tab];
        self.tab_options
            .get(cat)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn update(&mut self, action: Action) {
        match action {
            Action::Tick => {}
            Action::Quit => self.should_quit = true,
            Action::NextTab => {
                self.active_tab = (self.active_tab + 1) % self.tabs.len();
                self.selected_index = 0;
            }
            Action::PrevTab => {
                if self.active_tab == 0 {
                    self.active_tab = self.tabs.len() - 1;
                } else {
                    self.active_tab -= 1;
                }
                self.selected_index = 0;
            }
            Action::NextItem => {
                let len = self.current_options().len();
                if len > 0 {
                    self.selected_index = (self.selected_index + 1) % len;
                }
            }
            Action::PrevItem => {
                let len = self.current_options().len();
                if len > 0 {
                    if self.selected_index == 0 {
                        self.selected_index = len - 1;
                    } else {
                        self.selected_index -= 1;
                    }
                }
            }
            Action::ToggleSelected => {
                let options = self.current_options();
                if let Some(opt) = options.get(self.selected_index) {
                    let current = self
                        .options_state
                        .get(opt.id as &str)
                        .cloned()
                        .unwrap_or(false);
                    self.options_state.insert(opt.id.to_string(), !current);
                }
            }
            Action::Execute => {
                // Triggered in the main loop or by a component
            }
            Action::AddLog(log) => {
                self.logs.push(log);
                if self.logs.len() > 100 {
                    self.logs.remove(0);
                }
            }
            Action::UpdateMetrics(m) => {
                self.metrics = m;
            }
        }
    }
}
