use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Widget},
};

pub struct BlackBox<F> {
    render_inner_fn: F,
}

impl<F: FnOnce(Rect, &mut Buffer)> BlackBox<F> {
    pub fn with_content(render_inner_fn: F) -> Self {
        Self { render_inner_fn }
    }
}

impl<F: FnOnce(Rect, &mut Buffer)> Widget for BlackBox<F> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let Self { render_inner_fn } = self;

        let block = Block::bordered().style(Style::new().bg(Color::Black).fg(Color::White));
        let content = block.inner(area);
        block.render(area, buf);

        render_inner_fn(content, buf);
    }
}
