use ratatui::{
    crossterm::event::{Event, KeyCode},
    layout::Constraint,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Cell, Row, Table, Widget},
};
use savefile::file::game::{characters::full::FullCharacterSheet, PlayerProgress};
use tokio::sync::watch;

use crate::{
    keyboard::GetKeyCode,
    tui::{HandleEvent, VisualComponent},
};

use super::TabComponent;

#[derive(Debug)]
pub struct CharacterTab {
    progress: watch::Sender<PlayerProgress>,
    selected_character: usize,
}

impl CharacterTab {
    pub fn new(progress: watch::Sender<PlayerProgress>) -> Self {
        Self {
            progress,
            selected_character: 0,
        }
    }

    pub fn next_character(&mut self) {
        self.selected_character = self
            .selected_character
            .saturating_add(1)
            .clamp(0, FullCharacterSheet::N_CHARACTERS - 1);
    }

    pub fn previous_character(&mut self) {
        self.selected_character = self
            .selected_character
            .saturating_sub(1)
            .clamp(0, FullCharacterSheet::N_CHARACTERS - 1);
    }

    pub fn toggle_current_character(&mut self) {
        self.progress.send_modify(|progress| {
            let mut characters = progress.enabled_character.as_array();
            characters[self.selected_character] = !characters[self.selected_character];
            progress.enabled_character = characters.into();
        });
    }
}

impl HandleEvent for CharacterTab {
    type Error = anyhow::Error;

    fn handle_event(&mut self, event: &Event) -> Result<(), Self::Error> {
        match event.key_code() {
            Some(KeyCode::Up) => self.previous_character(),
            Some(KeyCode::Down) => self.next_character(),
            Some(KeyCode::Enter) => self.toggle_current_character(),
            _ => (),
        }

        Ok(())
    }
}

impl VisualComponent for CharacterTab {
    fn render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let rows = self
            .progress
            .borrow()
            .enabled_character
            .iter()
            .enumerate()
            .map(|(row_index, (character, is_enabled))| {
                let is_selected = row_index == self.selected_character;

                let character_name = Cell::new(character.to_string());

                let status_cell = if is_enabled {
                    Cell::new(Text::from("+").centered())
                        .style(Style::new().bg(Color::Green).fg(Color::Black))
                } else {
                    Cell::new(Text::from("X").centered())
                        .style(Style::new().bg(Color::Red).fg(Color::White))
                };
                let row = Row::new(vec![character_name, status_cell]);
                if is_selected {
                    row.style(Style::new().bg(Color::White).fg(Color::Black))
                } else {
                    row.style(Style::new().bg(Color::Black).fg(Color::White))
                }
            });

        let widths = [Constraint::Length(12), Constraint::Length(3)];
        Table::new(rows, widths)
            .style(Style::new().bg(Color::Black).fg(Color::White))
            .block(Block::bordered())
            .render(area, buf);
    }
}

impl TabComponent for CharacterTab {
    fn name(&self) -> &'static str {
        "Characters"
    }
}
