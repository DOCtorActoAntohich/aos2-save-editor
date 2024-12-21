use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::Style,
    widgets::{Block, Borders, Widget},
};

#[derive(Default)]
pub struct HorizontalSeparator {
    style: Style,
}

impl HorizontalSeparator {
    pub const CONSTRAINT: Constraint = Constraint::Length(1);

    pub fn new() -> Self {
        Self::default()
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
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
