use player_progress::{Arenas, PlayerProgress, Status};
use ratatui::crossterm::event::KeyCode;
use tokio::sync::watch;

use crate::{
    collection::SelectibleArray,
    progress::widget::{TogglesContent, TogglesTable},
    tui::{Event, HandleEvent},
};

use super::InteractibleTable;

pub struct Table {
    progress: watch::Sender<PlayerProgress>,
    arenas: SelectibleArray<Status, { Arenas::AMOUNT }>,
}

impl Table {
    pub fn new(progress: watch::Sender<PlayerProgress>) -> Self {
        let arenas = progress.borrow().arenas.to_array();
        Self {
            progress,
            arenas: SelectibleArray::new(arenas),
        }
    }
}

impl HandleEvent for Table {
    fn handle_event(&mut self, event: &Event) {
        match event.key_code() {
            Some(KeyCode::Up) => self.arenas.select_previous(),
            Some(KeyCode::Down) => self.arenas.select_next(),
            Some(KeyCode::Enter) => {
                self.arenas.modify_current(|status| *status = !*status);
                self.progress
                    .send_modify(|progress| progress.arenas = self.arenas.to_array().into());
            }
            _ => (),
        }
    }
}

impl InteractibleTable for Table {
    fn as_widget(&self, is_active: bool) -> TogglesTable<'_> {
        TogglesTable {
            name: "Arenas".into(),
            content: TogglesContent::new(self.progress.borrow().arenas.iter())
                .with_current(self.arenas.current_index()),
            is_active,
        }
    }
}
