use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Text,
    widgets::{Cell, Widget},
};

use crate::style::{IndexedColor, WithColor};

pub struct StatusCell<'a> {
    text: Text<'a>,
    is_enabled: bool,
}

impl<'a> StatusCell<'a> {
    pub fn new(text: impl Into<Text<'a>>) -> Self {
        Self {
            text: text.into(),
            is_enabled: false,
        }
    }

    pub fn enabled(mut self, is_enabled: bool) -> Self {
        self.is_enabled = is_enabled;
        self
    }

    pub fn style(&self) -> Style {
        if self.is_enabled {
            Style::new()
                .with_bg(IndexedColor::DarkGreen)
                .with_fg(Color::White)
        } else {
            Style::new()
                .with_bg(IndexedColor::DarkRed)
                .with_fg(Color::White)
        }
    }
}

impl<'a> From<StatusCell<'a>> for Cell<'a> {
    fn from(cell: StatusCell<'a>) -> Self {
        let style = cell.style();
        let StatusCell {
            text,
            is_enabled: _,
        } = cell;

        Cell::new(text).style(style)
    }
}

impl Widget for StatusCell<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let style = self.style();
        self.text.style(style).render(area, buf);
    }
}
