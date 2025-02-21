mod tabs;

use player_progress::PlayerProgress;
use ratatui::{
    crossterm::event::{Event, KeyCode},
    layout::{Constraint, Layout},
    widgets::Widget,
};
use tokio::sync::watch;

use crate::{
    collection::SelectibleArray,
    progress, statistics,
    tui::{event::GetKeyCode, HandleEvent, InteractibleComponent, VisualComponent},
    widget::black_box::BlackBox,
};

use self::tabs::EvenTabs;

pub trait InteratibleTabComponent: InteractibleComponent {
    fn name(&self) -> &'static str;
}

pub struct ContentWidget {
    tabs: SelectibleArray<Box<dyn InteratibleTabComponent>, 2>,
}

impl ContentWidget {
    pub const CONSTRAINT: Constraint = Constraint::Min(3);

    pub fn new(progress: watch::Sender<PlayerProgress>) -> Self {
        let tabs: [Box<dyn InteratibleTabComponent>; 2] = [
            Box::new(statistics::Tab::new(progress.subscribe())),
            Box::new(progress::Tab::new(progress)),
        ];
        Self {
            tabs: SelectibleArray::new(tabs),
        }
    }
}

impl HandleEvent for ContentWidget {
    fn handle_event(&mut self, event: &Event) {
        match event.key_code() {
            Some(KeyCode::PageUp) => self.tabs.select_previous(),
            Some(KeyCode::PageDown) => self.tabs.select_next(),
            _ => self.tabs.mut_current().handle_event(event),
        }
    }
}

impl VisualComponent for ContentWidget {
    fn render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let constraints = [Constraint::Length(1), Constraint::Min(3)];
        let layout = Layout::vertical(constraints);
        let [tabs_area, content_area] = layout.areas::<2>(area);

        EvenTabs::new(self.tabs.iter().map(|tab| tab.name()))
            .select(self.tabs.current_index())
            .render(tabs_area, buf);

        BlackBox::with_content(|area, buf| {
            self.tabs.current().render(area, buf);
        })
        .render(content_area, buf);
    }
}
