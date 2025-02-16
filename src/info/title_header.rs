use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Style},
    widgets::{Paragraph, Widget},
};

use crate::{
    style::{IndexedColor, WithColor},
    tui::VisualComponent,
};

#[derive(Debug)]
pub struct TitleHeader;

impl TitleHeader {
    pub const CONSTRAINT: Constraint = Constraint::Length(1);
}

impl VisualComponent for TitleHeader {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("AoS2 Save Editor")
            .style(
                Style::default()
                    .with_bg(IndexedColor::DarkBlue)
                    .with_fg(Color::White),
            )
            .centered()
            .render(area, buf);
    }
}
