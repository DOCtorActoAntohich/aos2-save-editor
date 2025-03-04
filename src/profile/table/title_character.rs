use online_profile::{title, PlayerOnlineProfile};
use ratatui::crossterm::event::KeyCode;
use tokio::sync::watch;

use crate::{
    collection::RadioButtonArray,
    profile::widget::TableContent,
    tui::{Event, HandleEvent},
};

use super::InteractibleTable;

pub struct Table {
    profile: watch::Sender<PlayerOnlineProfile>,
    characters: RadioButtonArray<title::Character, { title::Character::MEMBERS_COUNT }>,
}

impl Table {
    pub fn new(profile: watch::Sender<PlayerOnlineProfile>) -> Self {
        let characters = title::Character::members();
        let current = profile.borrow().title_character_in_background;

        let selected = characters
            .iter()
            .enumerate()
            .find_map(
                |(index, &value)| {
                    if value == current {
                        Some(index)
                    } else {
                        None
                    }
                },
            )
            .unwrap_or_default();

        Self {
            profile,
            characters: RadioButtonArray::new(characters, selected),
        }
    }
}

impl HandleEvent for Table {
    fn handle_event(&mut self, event: &Event) {
        match event.key_code() {
            Some(KeyCode::Up) => self.characters.hover_previous(),
            Some(KeyCode::Down) => self.characters.hover_next(),
            Some(KeyCode::Enter) => {
                self.characters.select_current();
                self.profile.send_modify(|profile| {
                    profile.title_character_in_background = *self.characters.current()
                });
            }
            Some(KeyCode::Char(_)) => {
                let jump_to = self
                    .characters
                    .find_by_text(event.accumulated_input())
                    .unwrap_or(self.characters.hovered_index());
                self.characters.hover_at(jump_to);
            }
            _ => (),
        }
    }
}

impl InteractibleTable for Table {
    fn name(&self) -> &'static str {
        "Background Character"
    }

    fn content_widget(&self) -> TableContent {
        TableContent::new(
            self.characters
                .to_array()
                .into_iter()
                .map(|character| character.to_string()),
        )
        .with_hovered(self.characters.hovered_index())
        .with_selected(self.characters.selected_index())
    }
}
