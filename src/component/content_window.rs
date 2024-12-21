use ratatui::{
    crossterm::event::{Event, KeyCode},
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    widgets::Widget,
};

use crate::{
    keyboard::GetKeyCode,
    tui::{HandleEvent, VisualComponent},
    widget::evenly_spaced_tabs::EvenlySpacedTabs,
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
        match event.key_code() {
            Some(KeyCode::PageDown) => self.next_tab(),
            Some(KeyCode::PageUp) => self.previous_tab(),
            _ => self.tabs[self.current_tab].handle_event(event)?,
        }

        Ok(())
    }
}

impl VisualComponent for ContentWidget {
    fn render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let constraints = [Constraint::Length(1), Constraint::Min(3)];
        let layout = Layout::vertical(constraints);
        let [tabs_area, content_area] = layout.areas::<2>(area);

        let unselected_style = Style::new().fg(Color::White).bg(Color::Black);
        EvenlySpacedTabs::new(self.tabs.iter().map(|tab| tab.name()))
            .selected_style(Style::new().fg(Color::Black).bg(Color::White).bold())
            .regular_style(unselected_style)
            .divider_style(unselected_style)
            .select(self.current_tab)
            .render(tabs_area, buf);

        self.tabs[self.current_tab].render(content_area, buf);
    }
}
