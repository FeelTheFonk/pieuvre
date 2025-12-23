use crate::commands::interactive::tui::app::AppState;
use crate::commands::interactive::tui::components::Component;
use crate::commands::interactive::tui::i18n;
use anyhow::Result;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub struct Footer;

impl Component for Footer {
    fn render(&self, f: &mut Frame, area: Rect, _state: &AppState) -> Result<()> {
        let help_text = vec![
            Span::styled(i18n::KEY_TABS, Style::default().fg(Color::Cyan)),
            Span::from(i18n::KEY_NEXT),
            Span::styled(i18n::KEY_NAV, Style::default().fg(Color::Cyan)),
            Span::from(i18n::KEY_NAVIGATE),
            Span::styled(i18n::KEY_SPACE, Style::default().fg(Color::Cyan)),
            Span::from(i18n::KEY_TOGGLE),
            Span::styled(i18n::KEY_ENTER, Style::default().fg(Color::Cyan)),
            Span::from(i18n::KEY_APPLY),
            Span::styled(i18n::KEY_Q, Style::default().fg(Color::Red)),
            Span::from(i18n::KEY_QUIT),
        ];

        let footer = Paragraph::new(Line::from(help_text))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::DarkGray)),
            );

        f.render_widget(footer, area);
        Ok(())
    }
}
