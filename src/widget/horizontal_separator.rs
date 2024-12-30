use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
};

#[derive(Debug)]
pub struct HorizontalSeparator {
    style: Style,
}

impl Default for HorizontalSeparator {
    fn default() -> Self {
        Self {
            style: Style::new().bg(Color::Black).fg(Color::White),
        }
    }
}

impl HorizontalSeparator {
    pub const CONSTRAINT: Constraint = Constraint::Length(1);
}

impl Widget for HorizontalSeparator {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Block::new()
            .borders(Borders::TOP)
            .style(self.style)
            .render(area, buf);
    }
}
