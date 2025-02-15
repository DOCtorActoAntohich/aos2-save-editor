use std::{borrow::Cow, ops::Range};

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    widgets::{self, Widget},
};

use crate::style::Selection;

use super::status::Status;

pub struct Table<'a> {
    items: Vec<Row<'a>>,
    current: usize,
    should_highlight_current: bool,
}

struct Row<'a> {
    name: Cow<'a, str>,
    value: Status,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TableSlice(Range<usize>);

impl<'a> Table<'a> {
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

    pub fn visible_slice(self, window_size: usize) -> Self {
        let Self {
            mut items,
            current,
            should_highlight_current,
        } = self;

        if let Some(TableSlice(range)) =
            TableSlice::in_collection(items.len(), current, window_size)
        {
            Self {
                current: current - range.start,
                items: items.drain(range).collect(),
                should_highlight_current,
            }
        } else {
            Self {
                items: Vec::new(),
                current: 0,
                should_highlight_current,
            }
        }
    }
}

impl Widget for Table<'_> {
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
            .map(|(row_index, Row { name, value })| {
                let row_name = ratatui::widgets::Cell::new(name.to_string());
                let status_toggle: Status = value.into();

                let is_selected = should_highlight_current && (row_index == current);
                widgets::Row::new(vec![row_name, status_toggle.into_cell()])
                    .style(Selection::from_is_selected(is_selected))
            });

        let widths = [Constraint::Min(12), Constraint::Min(3)];
        ratatui::widgets::Table::new(rows, widths)
            .style(Selection::Unselected)
            .render(area, buf);
    }
}

impl TableSlice {
    fn in_collection(length: usize, current: usize, window_size: usize) -> Option<Self> {
        if length == 0 || window_size == 0 || current >= length {
            None
        } else {
            let half_window = window_size / 2;
            let n_items_after_current = (length - 1) - current;
            let range = if n_items_after_current < half_window {
                let end = length;
                let start = length.saturating_sub(window_size);
                start..end
            } else {
                let start = current.saturating_sub(half_window);
                let end = start.saturating_add(window_size).min(length);
                start..end
            };
            Some(Self(range))
        }
    }
}

impl<'a, R, S> From<(R, S)> for Row<'a>
where
    R: Into<Cow<'a, str>>,
    S: Into<Status>,
{
    fn from((name, value): (R, S)) -> Self {
        Row {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TableSlice;

    #[rstest::rstest]
    #[case::full_collection_exact(15, 0, 15, Some(TableSlice(0..15)))]
    #[case::full_collection_bigger(15, 0, 30, Some(TableSlice(0..15)))]
    #[case::at_start_odd(15, 0, 3, Some(TableSlice(0..3)))]
    #[case::at_start_odd(15, 1, 3, Some(TableSlice(0..3)))]
    #[case::at_start_odd(15, 2, 3, Some(TableSlice(1..4)))]
    #[case::at_start_even(15, 1, 4, Some(TableSlice(0..4)))]
    #[case::at_middle_odd(15, 7, 3, Some(TableSlice(6..9)))]
    #[case::at_middle_odd(15, 7, 5, Some(TableSlice(5..10)))]
    #[case::at_end_odd(15, 12, 3, Some(TableSlice(11..14)))]
    #[case::at_end_odd(15, 13, 3, Some(TableSlice(12..15)))]
    #[case::at_end_odd(15, 14, 3, Some(TableSlice(12..15)))]
    #[case::impossible_current(15, 100, 3, None)]
    #[case::no_items(0, 0, 3, None)]
    #[case::no_window(15, 3, 0, None)]
    fn range_works(
        #[case] length: usize,
        #[case] current_index: usize,
        #[case] window_size: usize,
        #[case] expected_slice: Option<TableSlice>,
    ) {
        let actual_slice = TableSlice::in_collection(length, current_index, window_size);

        assert_eq!(expected_slice, actual_slice);
    }
}
