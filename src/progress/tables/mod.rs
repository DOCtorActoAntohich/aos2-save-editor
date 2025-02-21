mod arena;
mod character;
mod music;

use player_progress::PlayerProgress;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{Event, KeyCode},
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::Widget,
};
use tokio::sync::watch;

use crate::{
    collection::SelectibleArray,
    style,
    tui::{event::GetKeyCode, HandleEvent, VisualComponent},
    widget::{separator, split},
};

trait InteractibleTable: HandleEvent + Send {
    fn name(&self) -> &str;

    fn as_widget(&self) -> super::widget::Table;
}

pub struct TablesCollection {
    tables: SelectibleArray<Table, 3>,
}

struct Table(Box<dyn InteractibleTable>);

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
    pub const CONSTRAINT: Constraint = Constraint::Fill(1);

    pub fn render(&self, area: Rect, buf: &mut Buffer, is_selected: bool) {
        let Self(table) = self;

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
                    .as_widget()
                    .highlight_current(is_selected)
                    .render(area, buf);
            },
        };

        split::Horizontal { top, bottom }.render(area, buf);
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
        enum ToDraw<'a> {
            Table(usize, &'a Table),
            Separator(&'a separator::Vertical),
        }

        let vertical_separator = separator::Vertical::default();

        let to_draw: Vec<ToDraw<'_>> = self
            .tables
            .iter()
            .enumerate()
            .flat_map(|(index, table)| {
                [
                    ToDraw::Separator(&vertical_separator),
                    ToDraw::Table(index, table),
                ]
            })
            .skip(1)
            .collect();

        let constraints = to_draw.iter().map(|thing| match thing {
            ToDraw::Table(_, _) => Table::CONSTRAINT,
            ToDraw::Separator(_) => separator::Vertical::CONSTRAINT,
        });

        Layout::horizontal(constraints)
            .split(area)
            .iter()
            .zip(to_draw)
            .for_each(|(&area, thing)| match thing {
                ToDraw::Table(index, table) => {
                    let is_selected = index == self.tables.current_index();
                    table.render(area, buf, is_selected);
                }
                ToDraw::Separator(s) => s.render(area, buf),
            });
    }
}
