use ratatui::{
    style::{Color, Style},
    widgets::{Block, Widget},
};

#[derive(Debug, derive_more::Deref)]
pub struct BlackBox<'a>(Block<'a>);

impl Default for BlackBox<'_> {
    fn default() -> Self {
        let block = Block::bordered().style(Style::new().bg(Color::Black).fg(Color::White));
        Self(block)
    }
}

impl<'a> From<BlackBox<'a>> for Block<'a> {
    fn from(BlackBox(block): BlackBox<'a>) -> Self {
        block
    }
}

impl Widget for BlackBox<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Block::from(self).render(area, buf);
    }
}
