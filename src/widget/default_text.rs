use ratatui::{
    style::{Color, Style},
    text::Line,
    widgets::Widget,
};

#[derive(Debug)]
pub struct DefaultText<'a> {
    line: Line<'a>,
    style: Style,
}

impl<'a> DefaultText<'a> {
    pub fn new(text: impl Into<Line<'a>>) -> Self {
        Self {
            line: text.into(),
            style: Style::new().fg(Color::White).bg(Color::Black),
        }
    }

    pub fn red(mut self) -> Self {
        self.style = self.style.fg(Color::Red);
        self
    }
}

impl<'a> From<DefaultText<'a>> for Line<'a> {
    fn from(DefaultText { line, style }: DefaultText<'a>) -> Self {
        line.style(style)
    }
}

impl<'a> Widget for DefaultText<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Line::from(self).render(area, buf);
    }
}
