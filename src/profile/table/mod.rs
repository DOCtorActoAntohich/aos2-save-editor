pub mod color;

use ratatui::{
    buffer::Buffer,
    crossterm::event::KeyCode,
    layout::{Constraint, Rect},
    widgets::{Paragraph, Widget},
};

use crate::{
    collection::SelectibleArray,
    style,
    tui::{Event, HandleEvent, InteractibleComponent, VisualComponent},
    widget::split,
};

pub trait InteractibleTable: InteractibleComponent {
    fn name(&self) -> &str;

    fn as_widget(&self) -> super::widget::Table;
}

pub struct TablesCollection<const LENGTH: usize> {
    tables: SelectibleArray<Table, LENGTH>,
}

#[derive(derive_more::Deref)]
#[deref(forward)]
struct Table(Box<dyn InteractibleTable>);

impl<const LENGTH: usize> TablesCollection<LENGTH> {
    pub fn new(tables: [Box<dyn InteractibleTable>; LENGTH]) -> Self {
        Self {
            tables: SelectibleArray::new(tables.map(Table)),
        }
    }
}

impl Table {
    pub fn render(&self, area: Rect, buf: &mut Buffer, is_selected: bool) {
        let top = split::Area {
            constraint: Constraint::Length(1),
            render: |area: Rect, buf: &mut Buffer| {
                Paragraph::new(self.name())
                    .centered()
                    .style(style::Selection::from_is_selected(is_selected))
                    .render(area, buf);
            },
        };
        let bottom = split::Area {
            constraint: Constraint::Fill(1),
            render: |area: Rect, buf: &mut Buffer| {
                self.as_widget()
                    .highlight_hovered(is_selected)
                    .render(area, buf);
            },
        };
        split::Horizontal { top, bottom }.render(area, buf);
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

impl HandleEvent for Table {
    fn handle_event(&mut self, event: &Event) {
        let Self(table) = self;
        table.handle_event(event);
    }
}

impl<const N: usize> VisualComponent for TablesCollection<N> {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        self.tables.current().render(area, buf, true);
    }
}
