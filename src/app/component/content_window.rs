use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout},
    widgets::{Block, Paragraph, Widget},
};

use crate::{
    tui::{HandleEvent, VisualComponent},
    widget::EvenlySpacedTabs,
};

#[derive(Debug)]
pub struct ContentWidget {
    current_tab: usize,
    tab_names: Vec<&'static str>,
}

impl ContentWidget {
    pub const CONSTRAINT: Constraint = Constraint::Min(3);

    pub fn new() -> Self {
        Self {
            current_tab: 0,
            tab_names: vec!["amog", "imposter", "sus", "beniz", "comk"],
        }
    }

    pub fn next_tab(&mut self) {
        self.current_tab = self
            .current_tab
            .saturating_add(1)
            .clamp(0, self.tab_names.len() - 1);
    }

    pub fn previous_tab(&mut self) {
        self.current_tab = self
            .current_tab
            .saturating_sub(1)
            .clamp(0, self.tab_names.len() - 1);
    }
}

impl HandleEvent for ContentWidget {
    type Error = anyhow::Error;

    fn handle_event(&mut self, event: Event) -> Result<Event, Self::Error> {
        match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                if key_event.code == KeyCode::PageDown {
                    self.next_tab();
                } else if key_event.code == KeyCode::PageUp {
                    self.previous_tab();
                }
            }

            _ => (),
        }

        Ok(event)
    }
}

impl VisualComponent for ContentWidget {
    fn render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let constraints = [Constraint::Length(1), Constraint::Min(3)];
        let layout = Layout::vertical(constraints);
        let [tabs_area, content_area] = layout.areas::<2>(area);

        EvenlySpacedTabs::new(self.tab_names.clone())
            .select(self.current_tab)
            .render(tabs_area, buf);

        let content = Paragraph::new("ur mom gay")
            .centered()
            .block(Block::bordered());
        content.render(content_area, buf);
    }
}
