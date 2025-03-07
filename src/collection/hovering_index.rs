#[derive(Clone, Copy)]
pub struct HoveringIndex {
    n_items: usize,
    current: Option<usize>,
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

fn clamped(index: usize, n_items: usize) -> Option<usize> {
    match n_items.checked_sub(1) {
        Some(max) => Some(index.clamp(0, max)),
        None => None,
    }
}
