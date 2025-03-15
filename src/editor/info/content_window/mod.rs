mod tabs;

use ratatui::{
    crossterm::event::KeyCode,
    layout::{Constraint, Layout},
    widgets::Widget,
};

use crate::{
    collection::SelectableArray,
    editor,
    savefile::Savefile,
    tui::{Event, HandleEvent, InteractibleComponent, VisualComponent},
    widget::content_box::ContentBox,
};

use self::tabs::EvenTabs;

pub trait InteratibleTabComponent: InteractibleComponent {
    fn name(&self) -> &'static str;
}

pub struct ContentWidget {
    tabs: SelectableArray<Box<dyn InteratibleTabComponent>, 4>,
}

impl ContentWidget {
    #[must_use]
    pub fn new(savefile: &Savefile) -> Self {
        let tabs: [Box<dyn InteratibleTabComponent>; 4] = [
            Box::new(editor::statistics::Tab::new(savefile)),
            Box::new(editor::progress::Tab::new(savefile)),
            Box::new(editor::profile::avatar::Tab::new(savefile)),
            Box::new(editor::profile::title::Tab::new(savefile)),
        ];
        Self {
            tabs: SelectableArray::new(tabs),
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

        ContentBox::black()
            .with_content(|area, buf| {
                self.tabs.current().render(area, buf);
            })
            .render(content_area, buf);
    }
}
