mod arena;
mod character;
mod music;

use player_progress::PlayerProgress;
use ratatui::{
    buffer::Buffer,
    crossterm::event::KeyCode,
    layout::{Constraint, Rect},
    text::Line,
    widgets::Widget,
};
use tokio::sync::watch;

use crate::{
    collection::SelectibleArray,
    style,
    tui::{Event, HandleEvent, VisualComponent},
    widget::{sequence, split},
};

trait InteractibleTable: HandleEvent + Send {
    fn name(&self) -> &str;

    fn content_widget(&self) -> super::widget::Table;
}

pub struct TablesCollection {
    tables: SelectibleArray<Table, 3>,
}

struct Table(Box<dyn InteractibleTable>);

struct TableWidget<'a> {
    table: &'a dyn InteractibleTable,
    is_selected: bool,
}

impl TablesCollection {
    pub const CONSTRAINT: Constraint = Constraint::Fill(1);

    pub fn new(progress: watch::Sender<PlayerProgress>) -> Self {
        let tables: [Table; 3] = [
            Table(Box::new(self::character::Table::new(progress.clone()))),
            Table(Box::new(self::arena::Table::new(progress.clone()))),
            Table(Box::new(self::music::Table::new(progress))),
        ];
        Self {
            tables: SelectibleArray::new(tables),
        }
    }
}

impl Table {
    pub fn as_widget(&self, is_selected: bool) -> TableWidget<'_> {
        let Self(table) = self;
        TableWidget {
            table: table.as_ref(),
            is_selected,
        }
    }
}

impl HandleEvent for Table {
    fn handle_event(&mut self, event: &Event) {
        let Self(table) = self;
        table.handle_event(event);
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

impl Widget for TableWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let Self { table, is_selected } = self;

        let top = split::Area {
            constraint: Constraint::Length(1),
            render: |area: Rect, buf: &mut Buffer| {
                Line::from(table.name())
                    .centered()
                    .style(style::Selection::from_is_selected(is_selected))
                    .render(area, buf);
            },
        };

        let bottom = split::Area {
            constraint: Constraint::Fill(1),
            render: |area: Rect, buf: &mut Buffer| {
                table
                    .content_widget()
                    .highlight_current(is_selected)
                    .render(area, buf);
            },
        };

        split::Horizontal { top, bottom }.render(area, buf);
    }
}
