use player_progress::PlayableCharacters;
use ratatui::crossterm::event::KeyCode;
use tokio::sync::watch;

use crate::{
    collection::HoveringIndex,
    progress::widget::{TogglesContent, TogglesTable},
    tui::{Event, HandleEvent},
};

pub struct Table {
    playable_characters: watch::Sender<PlayableCharacters>,
    current_index: usize,
}

impl Table {
    pub fn new(playable_characters: watch::Sender<PlayableCharacters>) -> Self {
        Self {
            playable_characters,
            current_index: 0,
        }
    }
}

impl HandleEvent for Table {
    fn handle_event(&mut self, event: &Event) {
        let hover = HoveringIndex::from_collection(self.playable_characters.borrow().as_ref())
            .with_current(self.current_index);
        match event.key_code() {
            Some(KeyCode::Up) => {
                self.current_index = hover.previous().into_index().unwrap_or_default()
            }
            Some(KeyCode::Down) => {
                self.current_index = hover.next().into_index().unwrap_or_default()
            }
            Some(KeyCode::Home) => {
                self.current_index = hover.first().into_index().unwrap_or_default()
            }
            Some(KeyCode::End) => {
                self.current_index = hover.last().into_index().unwrap_or_default()
            }
            Some(KeyCode::Enter) => {
                self.playable_characters
                    .send_modify(|characters| characters.toggle_at(self.current_index));
            }
            _ => (),
        }
    }
}

impl super::Table for Table {
    fn as_widget(&self, is_active: bool) -> TogglesTable<'_> {
        TogglesTable {
            name: "Characters".into(),
            content: TogglesContent::new(self.playable_characters.borrow().list())
                .with_current(self.current_index),
            is_active,
        }
    }
}
