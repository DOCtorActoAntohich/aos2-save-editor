use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
};

pub trait RenderFn: FnOnce(Rect, &mut Buffer) {}

impl<F> RenderFn for F where F: FnOnce(Rect, &mut Buffer) {}

pub struct Area<F: RenderFn> {
    pub constraint: Constraint,
    pub render: F,
}

pub struct Horizontal<F1: RenderFn, F2: RenderFn> {
    pub top: Area<F1>,
    pub bottom: Area<F2>,
}

#[derive(Debug, Clone, Copy, derive_more::Into)]
struct LineStyle(Style);

impl Default for LineStyle {
    fn default() -> Self {
        Self(Style::new().bg(Color::Black).fg(Color::White))
    }
}

impl<F1, F2> Widget for Horizontal<F1, F2>
where
    F1: RenderFn,
    F2: RenderFn,
{
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let Self { top, bottom } = self;

        let constraints = [top.constraint, Constraint::Length(1), bottom.constraint];
        let [top_area, separator_area, bottom_area] =
            Layout::vertical(constraints).areas::<3>(area);

        (top.render)(top_area, buf);
        Block::new()
            .borders(Borders::TOP)
            .style(LineStyle::default())
            .render(separator_area, buf);
        (bottom.render)(bottom_area, buf);
    }
}
