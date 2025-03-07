use player_progress::{PlayableCharacters, PlayerProgress, Status};
use ratatui::crossterm::event::KeyCode;
use tokio::sync::watch;

use crate::{
    collection::SelectibleArray,
    progress::widget::{TogglesContent, TogglesTable},
    tui::{Event, HandleEvent},
};

pub struct Table {
    progress: watch::Sender<PlayerProgress>,
    characters: SelectibleArray<Status, { PlayableCharacters::AMOUNT }>,
}

impl Table {
    pub fn new(progress: watch::Sender<PlayerProgress>) -> Self {
        let characters = progress.borrow().playable_characters.clone().into();
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
            Some(KeyCode::Home) => self.characters.select_first(),
            Some(KeyCode::End) => self.characters.select_last(),
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

impl super::Table for Table {
    fn as_widget(&self, is_active: bool) -> TogglesTable<'_> {
        TogglesTable {
            name: "Characters".into(),
            content: TogglesContent::new(self.progress.borrow().playable_characters.iter())
                .with_current(self.characters.current_index()),
            is_active,
        }
    }
}
