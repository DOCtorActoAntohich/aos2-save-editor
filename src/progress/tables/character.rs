use player_progress::{PlayableCharacters, PlayerProgress, Status};
use ratatui::crossterm::event::KeyCode;
use tokio::sync::watch;

use crate::{
    collection::SelectibleArray,
    progress::widget,
    tui::{Event, HandleEvent},
};

use super::InteractibleTable;

pub struct Table {
    progress: watch::Sender<PlayerProgress>,
    characters: SelectibleArray<Status, { PlayableCharacters::AMOUNT }>,
}

impl Table {
    pub fn new(progress: watch::Sender<PlayerProgress>) -> Self {
        let characters = progress.borrow().playable_characters.to_array();
        Self {
            progress,
            characters: SelectibleArray::new(characters),
        }
    }
}

impl HandleEvent for Table {
    fn handle_event(&mut self, event: &Event) {
        match event.key_code() {
            Some(KeyCode::Up) => self.characters.select_previous(),
            Some(KeyCode::Down) => self.characters.select_next(),
            Some(KeyCode::Enter) => {
                self.characters.modify_current(|status| *status = !*status);
                self.progress.send_modify(|progress| {
                    progress.playable_characters = self.characters.to_array().into();
                });
            }
            _ => (),
        }
    }
}

impl InteractibleTable for Table {
    fn name(&self) -> &'static str {
        "Characters"
    }

    fn content_widget(&self) -> widget::Table {
        widget::Table::new(self.progress.borrow().playable_characters.iter())
            .with_current(self.characters.current_index())
    }
}
