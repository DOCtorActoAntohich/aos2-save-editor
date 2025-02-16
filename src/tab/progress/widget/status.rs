use ratatui::widgets::Cell;

use crate::widget::status_cell::StatusCell;

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
        let text = if is_enabled { "+" } else { "X" };
        StatusCell::new(text).enabled(is_enabled).into()
    }
}
