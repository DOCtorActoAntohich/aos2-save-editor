use online_profile::PlayerOnlineProfile;
use ratatui::{
    buffer::Buffer,
    crossterm::event::Event,
    layout::{Constraint, Rect},
    text::Line,
    widgets::{List, Widget},
};
use tokio::sync::watch;

use crate::{
    info::content_window::InteratibleTabComponent,
    tui::{HandleEvent, VisualComponent},
    widget::split,
};

use super::table::{color, InteractibleTable, TablesCollection};

pub struct Tab {
    tables: TablesCollection<1>,
}

struct InfoText;

impl InfoText {
    const N_LINES: u16 = 3;
    const CONSTRAINT: Constraint = Constraint::Length(Self::N_LINES);
}

impl Tab {
    pub fn new(profile: watch::Sender<PlayerOnlineProfile>) -> Self {
        let tables: [Box<dyn InteractibleTable>; 1] = [Box::new(color::Table::new(profile))];
        Self {
            tables: TablesCollection::new(tables),
        }
    }
}

impl HandleEvent for Tab {
    fn handle_event(&mut self, event: &Event) {
        self.tables.handle_event(event);
    }
}

impl VisualComponent for Tab {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let top = split::Area {
            constraint: InfoText::CONSTRAINT,
            render: |area: Rect, buf: &mut Buffer| {
                InfoText.render(area, buf);
            },
        };
        let bottom = split::Area {
            constraint: Constraint::Fill(1),
            render: |area: Rect, buf: &mut Buffer| {
                self.tables.render(area, buf);
            },
        };
        split::Horizontal { top, bottom }.render(area, buf);
    }
}

impl InteratibleTabComponent for Tab {
    fn name(&self) -> &'static str {
        "Online Title"
    }
}

impl VisualComponent for InfoText {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let lines: [Line<'_>; Self::N_LINES as usize] = ["a".into(), "b".into(), "c".into()];
        List::new(lines).render(area, buf);
    }
}
