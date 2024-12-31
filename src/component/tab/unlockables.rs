use ratatui::{
    buffer::Buffer,
    crossterm::event::Event,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Paragraph, Widget},
};

use crate::tui::{HandleEvent, VisualComponent};

use super::TabComponent;

#[derive(Debug)]
pub struct UnlockablesTab {}

impl UnlockablesTab {
    pub fn new() -> Self {
        Self {}
    }
}

impl HandleEvent for UnlockablesTab {
    type Error = anyhow::Error;

    fn handle_event(&mut self, event: &Event) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl VisualComponent for UnlockablesTab {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("stuff")
            .style(Style::new().bg(Color::Black).fg(Color::White))
            .centered()
            .block(Block::bordered())
            .render(area, buf);
    }
}

impl TabComponent for UnlockablesTab {
    fn name(&self) -> &'static str {
        "Unlockables"
    }
}
