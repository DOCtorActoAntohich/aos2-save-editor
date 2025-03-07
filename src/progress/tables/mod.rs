mod generic;

use ratatui::{
    buffer::Buffer,
    crossterm::event::KeyCode,
    layout::{Constraint, Rect},
    widgets::Widget,
};

use crate::{
    collection::SelectibleArray,
    savefile::Savefile,
    tui::{Event, HandleEvent, VisualComponent},
    widget::sequence,
};

use super::widget::TogglesTable;

trait Table: HandleEvent + Send {
    fn as_widget(&self, is_active: bool) -> TogglesTable<'_>;
}

pub struct TablesCollection {
    tables: SelectibleArray<Box<dyn Table>, 3>,
}

impl TablesCollection {
    pub const CONSTRAINT: Constraint = Constraint::Fill(1);

    pub fn new(savefile: &Savefile) -> Self {
        let playable_characters = savefile.progress().write_playable_characters();
        let arenas = savefile.progress().write_arenas();
        let music_tracks = savefile.progress().write_music_tracks();

        let tables: [Box<dyn Table>; 3] = [
            Box::new(self::generic::Table::new("Characters", playable_characters)),
            Box::new(self::generic::Table::new("Arenas", arenas)),
            Box::new(self::generic::Table::new("Music", music_tracks)),
        ];
        Self {
            tables: SelectibleArray::new(tables),
        }
    }
}

impl HandleEvent for TablesCollection {
    fn handle_event(&mut self, event: &Event) {
        match event.key_code() {
            Some(KeyCode::Left) => self.tables.select_previous(),
            Some(KeyCode::Right) => self.tables.select_next(),
            _ => self.tables.mut_current().handle_event(event),
        }
    }
}

impl VisualComponent for TablesCollection {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let Self { tables } = self;

        sequence::VerticallySeparated {
            widgets: tables.iter().enumerate().map(|(index, table)| {
                let is_selected = index == self.tables.current_index();
                table.as_widget(is_selected)
            }),
        }
        .render(area, buf);
    }
}
