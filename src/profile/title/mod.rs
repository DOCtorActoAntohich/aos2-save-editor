use online_profile::PlayerOnlineProfile;
use ratatui::{buffer::Buffer, crossterm::event::Event, layout::Rect};
use tokio::sync::watch;

use crate::{
    info::content_window::InteratibleTabComponent,
    tui::{HandleEvent, VisualComponent},
};

use super::table::{color, InteractibleTable, TablesCollection};

pub struct Tab {
    tables: TablesCollection<1>,
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
        self.tables.render(area, buf);
    }
}

impl InteratibleTabComponent for Tab {
    fn name(&self) -> &'static str {
        "Online Title"
    }
}
