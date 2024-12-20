use ratatui::{
    crossterm::event::{Event, KeyCode},
    layout::{Constraint, Layout},
    widgets::Widget,
};

use crate::{
    keyboard::GetKeyCode,
    tui::{HandleEvent, VisualComponent},
    widget::EvenlySpacedTabs,
};

use super::tab::TabComponent;

pub struct ContentWidget {
    current_tab: usize,
    tabs: Vec<Box<dyn TabComponent>>,
}

impl ContentWidget {
    pub const CONSTRAINT: Constraint = Constraint::Min(3);

    pub fn new(tabs: impl IntoIterator<Item = Box<dyn TabComponent>>) -> Self {
        Self {
            current_tab: 0,
            tabs: tabs.into_iter().collect(),
        }
    }

    pub fn next_tab(&mut self) {
        self.current_tab = self
            .current_tab
            .saturating_add(1)
            .clamp(0, self.tabs.len() - 1);
    }

    pub fn previous_tab(&mut self) {
        self.current_tab = self
            .current_tab
            .saturating_sub(1)
            .clamp(0, self.tabs.len() - 1);
    }
}

impl HandleEvent for ContentWidget {
    type Error = anyhow::Error;

    fn handle_event(&mut self, event: &Event) -> Result<(), Self::Error> {
        let Some(key_code) = event.key_code() else {
            return Ok(());
        };

        match key_code {
            KeyCode::PageDown => {
                self.next_tab();
                Ok(())
            }
            KeyCode::PageUp => {
                self.previous_tab();
                Ok(())
            }
            _ => self.tabs[self.current_tab].handle_event(event),
        }
    }
}

impl VisualComponent for ContentWidget {
    fn render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let constraints = [Constraint::Length(1), Constraint::Min(3)];
        let layout = Layout::vertical(constraints);
        let [tabs_area, content_area] = layout.areas::<2>(area);

        EvenlySpacedTabs::new(self.tabs.iter().map(|tab| tab.name()))
            .select(self.current_tab)
            .render(tabs_area, buf);

        self.tabs[self.current_tab].render(content_area, buf);
    }
}
