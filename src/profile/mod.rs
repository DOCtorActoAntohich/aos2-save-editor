mod table;
mod widget;

use online_profile::PlayerOnlineProfile;
use ratatui::{buffer::Buffer, crossterm::event::Event, layout::Rect};
use tokio::sync::watch;

use crate::{
    info::content_window::InteratibleTabComponent,
    tui::{HandleEvent, VisualComponent},
};

use self::table::TablesCollection;

pub struct Tab {
    colors: TablesCollection,
}

impl Tab {
    pub fn new(profile: watch::Sender<PlayerOnlineProfile>) -> Self {
        Self {
            colors: TablesCollection::new(profile),
        }
    }
}

impl HandleEvent for Tab {
    fn handle_event(&mut self, event: &Event) {
        self.colors.handle_event(event);
    }
}

impl VisualComponent for Tab {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        self.colors.render(area, buf);
    }
}

impl InteratibleTabComponent for Tab {
    fn name(&self) -> &'static str {
        "Online"
    }
}
