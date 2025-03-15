#[derive(Clone, Copy)]
pub struct HoveringIndex {
    n_items: usize,
    current: Option<usize>,
}

#[derive(Clone, Copy)]
pub struct RadioButtonIndex {
    n_items: usize,
    hovered: Option<usize>,
    selected: Option<usize>,
}

impl HoveringIndex {
    pub fn from_collection<T, R>(items: &R) -> Self
    where
        R: AsRef<[T]>,
    {
        Self {
            n_items: items.as_ref().len(),
            current: None,
        }
    }

    pub fn with_current(mut self, current: usize) -> Self {
        self.current = clamped(current, self.n_items);
        self
    }

    pub fn into_index(self) -> Option<usize> {
        let Self {
            n_items: _,
            current,
        } = self;
        current
    }

    pub fn next(mut self) -> Self {
        self.current = self
            .current
            .and_then(|index| clamped(index.saturating_add(1), self.n_items));
        self
    }

    pub fn previous(mut self) -> Self {
        self.current = self
            .current
            .and_then(|index| clamped(index.saturating_sub(1), self.n_items));
        self
    }

    pub fn last(mut self) -> Self {
        self.current = clamped(self.n_items.saturating_sub(1), self.n_items);
        self
    }

    pub fn first(mut self) -> Self {
        self.current = clamped(0, self.n_items);
        self
    }
}

impl RadioButtonIndex {
    pub fn from_collection<T, R>(items: &R) -> Self
    where
        R: AsRef<[T]>,
    {
        Self {
            n_items: items.as_ref().len(),
            hovered: None,
            selected: None,
        }
    }

    pub fn with_hovered(mut self, index: usize) -> Self {
        self.hovered = clamped(index, self.n_items);
        self
    }

    pub fn with_selected(mut self, index: usize) -> Self {
        self.selected = clamped(index, self.n_items);
        self
    }

    pub fn selected(self) -> Option<usize> {
        self.selected
    }

    pub fn hovered(self) -> Option<usize> {
        self.hovered
    }

    pub fn select_hovered(mut self) -> Self {
        self.selected = self.hovered;
        self
    }

    pub fn hover_next(mut self) -> Self {
        self.hovered = self
            .hovered
            .and_then(|index| clamped(index.saturating_add(1), self.n_items));
        self
    }

    pub fn hover_previous(mut self) -> Self {
        self.hovered = self
            .hovered
            .and_then(|index| clamped(index.saturating_sub(1), self.n_items));
        self
    }

    pub fn hover_last(mut self) -> Self {
        self.hovered = clamped(self.n_items.saturating_sub(1), self.n_items);
        self
    }

    pub fn hover_first(mut self) -> Self {
        self.hovered = clamped(0, self.n_items);
        self
    }
}

fn clamped(index: usize, n_items: usize) -> Option<usize> {
    match n_items.checked_sub(1) {
        Some(max) => Some(index.clamp(0, max)),
        None => None,
    }
}
