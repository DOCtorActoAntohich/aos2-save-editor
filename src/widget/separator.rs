use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vertical;

impl Vertical {
    pub const CONSTRAINT: Constraint = Constraint::Length(1);
}

impl Widget for Vertical {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Block::new()
            .borders(Borders::LEFT)
            .style(Style::new().bg(Color::Black).fg(Color::White))
            .render(area, buf);
    }
}
