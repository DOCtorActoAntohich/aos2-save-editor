#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectableArray<T, const LENGTH: usize> {
    items: [T; LENGTH],
    current_index: usize,
}

impl<T, const LENGTH: usize> SelectableArray<T, LENGTH> {
    const MAX_INDEX: usize = LENGTH - 1;

    pub fn new(array: [T; LENGTH]) -> Self {
        const { assert!(LENGTH > 0, "Zero-length array is not allowed") };

        Self {
            items: array,
            current_index: 0,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }

    pub fn current_index(&self) -> usize {
        self.current_index
    }

    pub fn select_next(&mut self) {
        self.current_index = self
            .current_index
            .saturating_add(1)
            .clamp(0, Self::MAX_INDEX);
    }

    pub fn select_previous(&mut self) {
        self.current_index = self
            .current_index
            .saturating_sub(1)
            .clamp(0, Self::MAX_INDEX);
    }

    pub fn mut_current(&mut self) -> &mut T {
        self.items
            .get_mut(self.current_index)
            .expect("Invariant: Index must be constrained to collection size")
    }

    pub fn current(&self) -> &T {
        self.items
            .get(self.current_index)
            .expect("Invariant: Index must be constrained to collection size")
    }
}

#[cfg(test)]
mod tests {
    use super::SelectableArray;

    #[rstest::fixture]
    fn selectible_array<const LENGTH: usize>() -> SelectableArray<usize, LENGTH> {
        let mut array = [0; LENGTH];
        array.iter_mut().enumerate().for_each(|(index, value)| {
            *value = index;
        });
        SelectableArray::new(array)
    }

    #[rstest::rstest]
    #[case(selectible_array::<1>())]
    #[case(selectible_array::<5>())]
    #[case(selectible_array::<100>())]
    fn proper_max_index_constraint<const LENGTH: usize>(
        #[case] mut array: SelectableArray<usize, LENGTH>,
    ) {
        for _ in 0..LENGTH * 2 {
            array.select_next();
        }

        assert_eq!(
            SelectableArray::<usize, LENGTH>::MAX_INDEX,
            array.current_index()
        );
    }
}
