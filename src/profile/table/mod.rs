mod color;

use online_profile::PlayerOnlineProfile;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{Event, KeyCode},
    layout::Rect,
};
use tokio::sync::watch;

use crate::{
    collection::SelectibleArray,
    tui::{event::GetKeyCode, HandleEvent, InteractibleComponent, VisualComponent},
};

trait InteractibleTable: InteractibleComponent {
    fn name(&self) -> &str;
}

pub struct TablesCollection {
    tables: SelectibleArray<Box<dyn InteractibleTable>, 1>,
}

impl TablesCollection {
    pub fn new(profile: watch::Sender<PlayerOnlineProfile>) -> Self {
        let tables: [Box<dyn InteractibleTable>; 1] = [Box::new(self::color::Table::new(profile))];
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
            _other => self.tables.mut_current().handle_event(event),
        }
    }
}

impl VisualComponent for TablesCollection {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        self.tables.current().render(area, buf);
    }
}
