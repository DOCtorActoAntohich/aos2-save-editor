use ratatui::{
    crossterm::event::Event,
    style::{Color, Style},
    widgets::{Block, Paragraph, Widget},
};

use crate::tui::{HandleEvent, VisualComponent};

use super::TabComponent;

pub struct EmptyTab;

impl HandleEvent for EmptyTab {
    type Error = anyhow::Error;

    fn handle_event(&mut self, _: &Event) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl VisualComponent for EmptyTab {
    fn render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        Paragraph::new("ur mom gay")
            .style(Style::new().bg(Color::Black).fg(Color::White))
            .centered()
            .block(Block::bordered())
            .render(area, buf);
    }
}

impl TabComponent for EmptyTab {
    fn name(&self) -> &'static str {
        "[trolleing]"
    }
}
