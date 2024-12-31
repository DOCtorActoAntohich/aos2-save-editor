use ratatui::style::{Color, Style};

#[derive(Debug, derive_more::Into)]
pub struct Unlocked(Style);

#[derive(Debug, derive_more::Into)]
pub struct PossibleUnlocks(Style);

impl Default for Unlocked {
    fn default() -> Self {
        Self(Style::new().bg(Color::Green).fg(Color::Black))
    }
}

impl Default for PossibleUnlocks {
    fn default() -> Self {
        /// Powershell sux and doesn't display [`Color::Yellow`]
        const YELLOW: Color = Color::Indexed(220);

        Self(Style::new().bg(YELLOW).fg(Color::Black))
    }
}
