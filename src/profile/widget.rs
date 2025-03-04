use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{self, Cell, Row, Widget},
};

use crate::{
    collection::ListSlice,
    style::{self, IndexedColor, WithColor as _},
};

#[derive(Default)]
pub struct TableContent<'a> {
    items: Vec<Line<'a>>,
    selected: Option<usize>,
    hovered: usize,
    should_highlight_hovered: bool,
}

impl<'a> TableContent<'a> {
    pub fn new(items: impl IntoIterator<Item = impl Into<Line<'a>>>) -> Self {
        Self {
            items: items.into_iter().map(Into::into).collect(),
            selected: None,
            hovered: 0,
            should_highlight_hovered: false,
        }
    }

    pub fn with_selected(mut self, selected: usize) -> Self {
        self.selected = Some(selected);
        self
    }

    pub fn with_hovered(mut self, hovered: usize) -> Self {
        self.hovered = hovered;
        self
    }

    pub fn highlight_hovered(mut self, highlight: bool) -> Self {
        self.should_highlight_hovered = highlight;
        self
    }

    fn visible_slice(self, window_size: usize) -> Self {
        let Self {
            mut items,
            selected,
            hovered,
            should_highlight_hovered,
        } = self;

        match ListSlice::in_collection(items.len(), hovered, window_size) {
            Some(slice) => {
                let range = slice.into_range();
                Self {
                    selected: selected.and_then(|selected| selected.checked_sub(range.start)),
                    hovered: hovered - range.start,
                    items: items.drain(range).collect(),
                    should_highlight_hovered,
                }
            }
            None => Self {
                should_highlight_hovered,
                selected,
                ..Default::default()
            },
        }
    }
}

impl Widget for TableContent<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let Self {
            items,
            selected,
            hovered,
            should_highlight_hovered,
        } = self.visible_slice(area.height.into());

        let rows = items
            .into_iter()
            .enumerate()
            .zip(
                [IndexedColor::DarkGray, IndexedColor::Gray]
                    .into_iter()
                    .cycle(),
            )
            .map(|((row_index, row_line), background_color)| {
                let is_selected = Some(row_index) == selected;
                let is_hovered = row_index == hovered;

                let selection_text = if is_selected { "[X]" } else { "[ ]" };

                let cells = [
                    Cell::new(Line::from(selection_text).centered()),
                    Cell::new(row_line),
                ];
                let style = if should_highlight_hovered && is_hovered {
                    style::Selection::from_is_selected(is_hovered).into()
                } else {
                    Style::new().with_bg(background_color).fg(Color::White)
                };
                Row::new(cells).style(style)
            });

        let widths = [Constraint::Length(3), Constraint::Fill(1)];
        widgets::Table::new(rows, widths).render(area, buf);
    }
}
