use ratatui::{
    style::{Color, Style},
    text::Text,
    widgets::Cell,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Status(bool);

impl From<bool> for Status {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<player_progress::Status> for Status {
    fn from(status: player_progress::Status) -> Self {
        status.is_enabled().into()
    }
}

impl Status {
    pub fn into_cell(self) -> Cell<'static> {
        let Self(is_enabled) = self;
        if is_enabled {
            Cell::new(Text::from("+").centered())
                .style(Style::new().bg(Color::Green).fg(Color::Black))
        } else {
            Cell::new(Text::from("X").centered())
                .style(Style::new().bg(Color::Red).fg(Color::White))
        }
    }
}
