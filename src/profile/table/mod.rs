pub mod generic;

use ratatui::{buffer::Buffer, crossterm::event::KeyCode, layout::Rect, widgets::Widget};

use crate::{
    collection::SelectibleArray,
    tui::{Event, HandleEvent, VisualComponent},
    widget::sequence,
};

use super::widget::RadioButtonsTable;

pub trait InteractibleTable: HandleEvent + Send {
    fn as_widget(&self, is_active: bool) -> RadioButtonsTable<'_>;
}

pub struct TablesCollection<const LENGTH: usize> {
    tables: SelectibleArray<Box<dyn InteractibleTable>, LENGTH>,
}

impl<const LENGTH: usize> TablesCollection<LENGTH> {
    pub fn new(tables: [Box<dyn InteractibleTable>; LENGTH]) -> Self {
        Self {
            tables: SelectibleArray::new(tables),
        }
    }
}

impl<const N: usize> HandleEvent for TablesCollection<N> {
    fn handle_event(&mut self, event: &Event) {
        match event.key_code() {
            Some(KeyCode::Left) => self.tables.select_previous(),
            Some(KeyCode::Right) => self.tables.select_next(),
            _other => self.tables.mut_current().handle_event(event),
        }
    }
}

impl<const N: usize> VisualComponent for TablesCollection<N> {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        sequence::VerticallySeparated {
            widgets: self.tables.iter().enumerate().map(|(index, table)| {
                let is_selected = index == self.tables.current_index();
                table.as_widget(is_selected)
            }),
        }
        .render(area, buf);
    }
}
