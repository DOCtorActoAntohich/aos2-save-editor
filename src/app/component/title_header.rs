use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Style},
    widgets::{Paragraph, Widget},
};

use crate::tui::VisualComponent;

#[derive(Debug)]
pub struct TitleHeader;

impl TitleHeader {
    pub const CONSTRAINT: Constraint = Constraint::Length(1);
}

impl VisualComponent for TitleHeader {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        pub const DARKEST_BLUE: Color = Color::Indexed(17);

        Paragraph::new("AoS2 Save Editor")
            .style(Style::default().bg(DARKEST_BLUE).fg(Color::White))
            .centered()
            .render(area, buf);
    }
}
