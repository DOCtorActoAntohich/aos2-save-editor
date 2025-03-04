mod collection;
mod generic;

pub use self::collection::Collection;
pub use self::generic::{Generic, Params};

use crate::tui::HandleEvent;

use super::widget::RadioButtonsTable;

pub trait Table: HandleEvent + Send {
    fn as_widget(&self, is_active: bool) -> RadioButtonsTable<'_>;
}
