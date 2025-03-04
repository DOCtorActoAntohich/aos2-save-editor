use std::borrow::Cow;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{self, Widget},
};

use crate::{
    collection::ListSlice,
    style::{self, IndexedColor, Selection, WithColor},
    widget::split,
};

use super::status::Status;

pub struct TogglesTable<'a> {
    pub name: Cow<'a, str>,
    pub content: TogglesContent<'a>,
    pub is_active: bool,
}

#[derive(Default)]
pub struct TogglesContent<'a> {
    items: Vec<Row<'a>>,
    current: usize,
    should_highlight_current: bool,
}

struct Row<'a> {
    name: Cow<'a, str>,
    status: Status,
}

struct RowStyle {
    default_bg: IndexedColor,
    is_selected: bool,
}

impl From<RowStyle> for Style {
    fn from(
        RowStyle {
            default_bg,
            is_selected,
        }: RowStyle,
    ) -> Self {
        if is_selected {
            Style::new().bg(Color::White).fg(Color::Black).bold()
        } else {
            Style::new().with_bg(default_bg).fg(Color::White)
        }
    }
}

impl<'a> TogglesContent<'a> {
    pub fn new(
        items: impl IntoIterator<Item = (impl Into<Cow<'a, str>>, impl Into<Status>)>,
    ) -> Self {
        Self {
            items: items.into_iter().map(Into::into).collect(),
            current: 0,
            should_highlight_current: false,
        }
    }

    pub fn with_current(mut self, current: usize) -> Self {
        self.current = current;
        self
    }

    pub fn highlight_current(mut self, should_highlight: bool) -> Self {
        self.should_highlight_current = should_highlight;
        self
    }

    fn visible_slice(self, window_size: usize) -> Self {
        let Self {
            mut items,
            current,
            should_highlight_current,
        } = self;

        match ListSlice::in_collection(items.len(), current, window_size) {
            Some(slice) => {
                let range = slice.into_range();
                Self {
                    current: current - range.start,
                    items: items.drain(range).collect(),
                    should_highlight_current,
                }
            }
            None => Self {
                should_highlight_current,
                ..Default::default()
            },
        }
    }
}

impl Widget for TogglesTable<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let Self {
            name,
            content,
            is_active,
        } = self;

        let top = split::Area {
            constraint: Constraint::Length(1),
            render: |area: Rect, buf: &mut Buffer| {
                Line::from(name)
                    .centered()
                    .style(style::Selection::from_is_selected(is_active))
                    .render(area, buf);
            },
        };

        let bottom = split::Area {
            constraint: Constraint::Fill(1),
            render: |area: Rect, buf: &mut Buffer| {
                content.highlight_current(is_active).render(area, buf);
            },
        };

        split::Horizontal { top, bottom }.render(area, buf);
    }
}

impl Widget for TogglesContent<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let Self {
            items,
            current,
            should_highlight_current,
        } = self.visible_slice(area.height.into());

        let rows = items
            .into_iter()
            .enumerate()
            .zip(
                [IndexedColor::DarkGray, IndexedColor::Gray]
                    .into_iter()
                    .cycle(),
            )
            .map(|((row_index, Row { name, status }), default_bg)| {
                let row_name = ratatui::widgets::Cell::new(name.to_string());

                let is_selected = should_highlight_current && (row_index == current);
                let style = RowStyle {
                    default_bg,
                    is_selected,
                };
                widgets::Row::new(vec![row_name, status.into()]).style(style)
            });

        let widths = [Constraint::Fill(3), Constraint::Fill(2)];
        ratatui::widgets::Table::new(rows, widths)
            .style(Selection::Unselected)
            .render(area, buf);
    }
}

impl<'a, N, S> From<(N, S)> for Row<'a>
where
    N: Into<Cow<'a, str>>,
    S: Into<Status>,
{
    fn from((name, status): (N, S)) -> Self {
        Row {
            name: name.into(),
            status: status.into(),
        }
    }
}
