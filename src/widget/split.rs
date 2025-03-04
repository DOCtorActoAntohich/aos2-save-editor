use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
};

const LINE_STYLE: Style = Style::new().bg(Color::Black).fg(Color::White);

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

pub struct Vertical<F1: RenderFn, F2: RenderFn> {
    pub left: Area<F1>,
    pub right: Area<F2>,
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
            .style(LINE_STYLE)
            .render(separator_area, buf);
        (bottom.render)(bottom_area, buf);
    }
}

impl<F1, F2> Widget for Vertical<F1, F2>
where
    F1: RenderFn,
    F2: RenderFn,
{
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let Self { left, right } = self;

        let constraints = [left.constraint, Constraint::Length(1), right.constraint];
        let [left_area, separator_area, right_area] =
            Layout::horizontal(constraints).areas::<3>(area);

        (left.render)(left_area, buf);
        (right.render)(right_area, buf);
        Block::new()
            .borders(Borders::LEFT)
            .style(LINE_STYLE)
            .render(separator_area, buf);
    }
}
