use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct RadioButtonArray<T, const LENGTH: usize> {
    items: [T; LENGTH],
    hover_index: usize,
    selected_index: usize,
}

impl<T, const LENGTH: usize> RadioButtonArray<T, LENGTH> {
    const MAX_INDEX: usize = LENGTH - 1;

    pub fn new(items: [T; LENGTH], selected_index: usize) -> Self {
        const { assert!(LENGTH > 0, "Zero-length array is not allowed") };

        let selected_index = selected_index.clamp(0, Self::MAX_INDEX);
        Self {
            items,
            hover_index: selected_index,
            selected_index,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }

    pub fn hovered_index(&self) -> usize {
        self.hover_index
    }

    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    pub fn current(&self) -> &T {
        self.items
            .get(self.selected_index)
            .expect("Invariant: Index is constrained to array size")
    }

    pub fn select_current(&mut self) {
        self.selected_index = self.hover_index;
    }

    pub fn hover_next(&mut self) {
        self.hover_index = self.hover_index.saturating_add(1).clamp(0, Self::MAX_INDEX);
    }

    pub fn hover_previous(&mut self) {
        self.hover_index = self.hover_index.saturating_sub(1).clamp(0, Self::MAX_INDEX);
    }

    pub fn hover_at(&mut self, index: usize) {
        if (0..LENGTH).contains(&index) {
            self.hover_index = index;
        }
    }
}

impl<T: Clone, const LENGTH: usize> RadioButtonArray<T, LENGTH> {
    pub fn to_array(&self) -> [T; LENGTH] {
        self.items.clone()
    }
}

impl<T: Display, const N: usize> RadioButtonArray<T, N> {
    pub fn find_by_text(&self, text: &str) -> Option<usize> {
        let text = text.to_ascii_lowercase();
        self.iter().enumerate().find_map(|(index, value)| {
            let mut value = value.to_string();
            value.make_ascii_lowercase();
            value.contains(&text).then_some(index)
        })
    }
}
