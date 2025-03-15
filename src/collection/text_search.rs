use std::fmt::Display;

pub struct TextSearch<'a, T> {
    items: &'a [T],
}

impl<'a, T: Display> TextSearch<'a, T> {
    pub fn in_collection(items: &'a [T]) -> Self {
        Self { items }
    }

    pub fn with_text(self, text: &str) -> Option<usize> {
        let Self { items } = self;

        let text = text.to_ascii_lowercase();

        items.iter().enumerate().find_map(|(index, item)| {
            let mut string = item.to_string();
            string.make_ascii_lowercase();
            string.contains(&text).then_some(index)
        })
    }
}
