use ratatui::{buffer::Buffer, crossterm::event::KeyCode, layout::Rect, widgets::Widget};

use crate::{
    collection::SelectibleArray,
    tui::{Event, HandleEvent, VisualComponent},
    widget::sequence,
};

use super::Table;

pub struct Collection<const LENGTH: usize> {
    tables: SelectibleArray<Box<dyn Table>, LENGTH>,
}

impl<const LENGTH: usize> Collection<LENGTH> {
    pub fn new(tables: [Box<dyn Table>; LENGTH]) -> Self {
        Self {
            tables: SelectibleArray::new(tables),
        }
    }
}

impl<const N: usize> HandleEvent for Collection<N> {
    fn handle_event(&mut self, event: &Event) {
        match event.key_code() {
            Some(KeyCode::Left) => self.tables.select_previous(),
            Some(KeyCode::Right) => self.tables.select_next(),
            _other => self.tables.mut_current().handle_event(event),
        }
    }
}

impl<const N: usize> VisualComponent for Collection<N> {
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
