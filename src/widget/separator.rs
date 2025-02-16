use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::Color,
    widgets::{Block, Borders, Widget},
};

#[derive(Debug, Clone, Copy, derive_more::Into)]
pub struct Style(ratatui::style::Style);

#[derive(Debug, Clone, Copy, Default)]
pub struct Vertical(Style);

impl Default for Style {
    fn default() -> Self {
        Self(
            ratatui::style::Style::new()
                .bg(Color::Black)
                .fg(Color::White),
        )
    }
}

impl Vertical {
    pub const CONSTRAINT: Constraint = Constraint::Length(1);
}

impl Widget for Vertical {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let Self(style) = self;
        Block::new()
            .borders(Borders::LEFT)
            .style(style)
            .render(area, buf);
    }
}
