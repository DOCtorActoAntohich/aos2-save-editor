use std::{fmt::Display, ops::Range};

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Style, Stylize},
    widgets::{self, Cell, Table, Widget},
};

use super::status_toggle::StatusToggle;

pub struct TogglesTable<R, S> {
    pub items: Vec<Row<R, S>>,
    pub current: usize,
}

pub struct Row<R, S> {
    pub name: R,
    pub value: S,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TableSlice(Range<usize>);

#[derive(derive_more::Into)]
struct SelectedStyle(Style);

#[derive(derive_more::Into)]
struct UnselectedStyle(Style);

impl<R, S> TogglesTable<R, S> {
    pub fn visible_slice(self, window_size: usize) -> Self {
        let Self { mut items, current } = self;

        if let Some(TableSlice(range)) =
            TableSlice::in_collection(items.len(), current, window_size)
        {
            Self {
                current: current - range.start,
                items: items.drain(range).collect(),
            }
        } else {
            Self {
                items: Vec::new(),
                current: 0,
            }
        }
    }
}

impl<R, S> Widget for TogglesTable<R, S>
where
    R: Display,
    S: Into<StatusToggle>,
{
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let Self { items, current } = self.visible_slice(area.height.into());

        let rows = items
            .into_iter()
            .enumerate()
            .map(|(row_index, Row { name, value })| {
                let row_name = Cell::new(name.to_string());
                let status_toggle: StatusToggle = value.into();
                let row = widgets::Row::new(vec![row_name, status_toggle.into_cell()]);

                let is_selected = row_index == current;
                if is_selected {
                    row.style(SelectedStyle::default())
                } else {
                    row.style(UnselectedStyle::default())
                }
            });
        let widths = [Constraint::Min(12), Constraint::Min(3)];
        Table::new(rows, widths)
            .style(UnselectedStyle::default())
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

impl Default for SelectedStyle {
    fn default() -> Self {
        Self(Style::new().bg(Color::White).fg(Color::Black).bold())
    }
}

impl Default for UnselectedStyle {
    fn default() -> Self {
        Self(Style::new().bg(Color::Black).fg(Color::White))
    }
}

impl<R, S> From<(R, S)> for Row<R, S> {
    fn from((name, value): (R, S)) -> Self {
        Row { name, value }
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
