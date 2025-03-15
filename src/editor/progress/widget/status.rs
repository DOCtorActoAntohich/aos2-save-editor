use ratatui::{
    style::{Color, Style},
    text::Text,
    widgets::Cell,
};

use crate::style::{IndexedColor, WithColor};

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

impl From<Status> for Cell<'_> {
    fn from(Status(is_enabled): Status) -> Self {
        let style = if is_enabled {
            Style::new()
                .with_bg(IndexedColor::DarkGreen)
                .with_fg(Color::White)
        } else {
            Style::new()
                .with_bg(IndexedColor::DarkRed)
                .with_fg(Color::White)
        };

        let text = if is_enabled { "+" } else { "X" };

        Cell::new(Text::from(text).centered()).style(style)
    }
}
