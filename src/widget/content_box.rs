use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{block::Title, Block, Widget},
};

use crate::style::IndexedColor;

type DoNothingFn = fn(Rect, &mut Buffer);

pub struct ContentBox<'a, F> {
    bg: Color,
    fg: Color,
    title: Option<Title<'a>>,
    render_inner_fn: F,
}

impl<'a, F> ContentBox<'a, F> {
    pub fn with_title(mut self, title: impl Into<Title<'a>>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl<'a> ContentBox<'a, DoNothingFn> {
    fn do_nothing(_: Rect, _: &mut Buffer) {}

    pub fn black() -> Self {
        Self {
            bg: Color::Black,
            fg: Color::White,
            title: None,
            render_inner_fn: Self::do_nothing,
        }
    }

    pub fn gray() -> Self {
        Self {
            bg: IndexedColor::DarkGray.into(),
            fg: Color::White,
            title: None,
            render_inner_fn: Self::do_nothing,
        }
    }

    pub fn with_content<F>(self, render_inner_fn: F) -> ContentBox<'a, F>
    where
        F: FnOnce(Rect, &mut Buffer),
    {
        let Self {
            bg,
            fg,
            title,
            render_inner_fn: _,
        } = self;
        ContentBox {
            bg,
            fg,
            title,
            render_inner_fn,
        }
    }
}

impl<F: FnOnce(Rect, &mut Buffer)> Widget for ContentBox<'_, F> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let Self {
            bg,
            fg,
            title,
            render_inner_fn,
        } = self;

        let block = Block::bordered().style(Style::new().bg(bg).fg(fg));
        let block = if let Some(title) = title {
            block.title(title).title_alignment(Alignment::Center)
        } else {
            block
        };
        let content = block.inner(area);
        block.render(area, buf);

        render_inner_fn(content, buf);
    }
}
