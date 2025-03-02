#[derive(Debug, Clone)]
struct RadioButtonArray<T, const LENGTH: usize> {
    items: [T; LENGTH],
    hover_index: usize,
    selected_index: usize,
}

impl<T, const LENGTH: usize> RadioButtonArray<T, LENGTH> {
    const MAX_INDEX: usize = LENGTH - 1;

    pub fn new(items: [T; LENGTH], selected_index: usize) -> Option<Self> {
        const { assert!(LENGTH > 0, "Zero-length array is not allowed") };

        if selected_index < LENGTH {
            Some(Self {
                items,
                hover_index: 0,
                selected_index,
            })
        } else {
            None
        }
    }

    pub fn hover_index(&self) -> usize {
        self.hover_index
    }

    pub fn selected_index(&self) -> usize {
        self.selected_index
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
}

impl<T: Clone, const LENGTH: usize> RadioButtonArray<T, LENGTH> {
    pub fn to_array(&self) -> [T; LENGTH] {
        self.items.clone()
    }
}
