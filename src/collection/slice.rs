use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Into)]
pub struct ListSlice(Range<usize>);

impl ListSlice {
    pub fn into_range(self) -> Range<usize> {
        let Self(range) = self;
        range
    }

    /// Collection slice around a currently selected item.
    /// Slice fits inside a window size (i.e. number of elements).
    ///
    /// Intended for scrolling and rendering in TUI.
    pub fn in_collection(length: usize, current: usize, window_size: usize) -> Option<Self> {
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

#[cfg(test)]
mod tests {
    use super::ListSlice;

    #[rstest::rstest]
    #[case::full_collection_exact(15, 0, 15, Some(ListSlice(0..15)))]
    #[case::full_collection_bigger(15, 0, 30, Some(ListSlice(0..15)))]
    #[case::at_start_odd(15, 0, 3, Some(ListSlice(0..3)))]
    #[case::at_start_odd(15, 1, 3, Some(ListSlice(0..3)))]
    #[case::at_start_odd(15, 2, 3, Some(ListSlice(1..4)))]
    #[case::at_start_even(15, 1, 4, Some(ListSlice(0..4)))]
    #[case::at_middle_odd(15, 7, 3, Some(ListSlice(6..9)))]
    #[case::at_middle_odd(15, 7, 5, Some(ListSlice(5..10)))]
    #[case::at_end_odd(15, 12, 3, Some(ListSlice(11..14)))]
    #[case::at_end_odd(15, 13, 3, Some(ListSlice(12..15)))]
    #[case::at_end_odd(15, 14, 3, Some(ListSlice(12..15)))]
    #[case::impossible_current(15, 100, 3, None)]
    #[case::no_items(0, 0, 3, None)]
    #[case::no_window(15, 3, 0, None)]
    fn range_works(
        #[case] length: usize,
        #[case] current_index: usize,
        #[case] window_size: usize,
        #[case] expected_slice: Option<ListSlice>,
    ) {
        let actual_slice = ListSlice::in_collection(length, current_index, window_size);

        assert_eq!(expected_slice, actual_slice);
    }
}
