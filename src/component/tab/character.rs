use ratatui::{
    crossterm::event::{Event, KeyCode},
    layout::Constraint,
    widgets::{Block, Row, Table, Widget},
};
use savefile::file::game::{characters::full::FullCharacterValueGrid, PlayerProgress};
use tokio::sync::watch;

use crate::{
    keyboard::GetKeyCode,
    tui::{HandleEvent, VisualComponent},
};

use super::TabComponent;

#[derive(Debug)]
pub struct CharacterTab {
    progress: watch::Sender<PlayerProgress>,
    grid: FullCharacterValueGrid<bool>,
}

impl CharacterTab {
    pub fn new(progress: watch::Sender<PlayerProgress>) -> Self {
        let grid = progress.borrow().enabled_character.into();
        Self { progress, grid }
    }
}

impl HandleEvent for CharacterTab {
    type Error = anyhow::Error;

    fn handle_event(&mut self, event: &Event) -> Result<(), Self::Error> {
        match event.key_code() {
            Some(KeyCode::Up) => self.grid.switch_previous(),
            Some(KeyCode::Down) => self.grid.switch_next(),
            Some(KeyCode::Enter) => {
                self.grid.toggle_current();
                self.progress
                    .send_modify(|progress| progress.enabled_character = self.grid.clone().into());
            }
            _ => (),
        }

        Ok(())
    }
}

impl VisualComponent for CharacterTab {
    fn render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let rows: Vec<_> = self
            .grid
            .characters()
            .map(|(name, &is_enabled)| {
                Row::new(vec![
                    name.to_string(),
                    if is_enabled {
                        "[+]".to_owned()
                    } else {
                        "[ ]".to_owned()
                    },
                ])
            })
            .collect();
        let widths = (0..rows.len()).into_iter().map(|_| Constraint::Max(32));
        Table::new(rows, widths)
            .block(Block::bordered())
            .render(area, buf);
    }
}

impl TabComponent for CharacterTab {
    fn name(&self) -> &'static str {
        "Characters"
    }
}
