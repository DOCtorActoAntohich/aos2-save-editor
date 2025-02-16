use ratatui::style::Color;

pub enum IndexedColor {
    DarkGreen,
    DarkRed,
    DarkYellow,
    DarkGray,
    DarkBlue,
}

impl From<IndexedColor> for Color {
    fn from(value: IndexedColor) -> Self {
        // i hate numbers
        match value {
            IndexedColor::DarkGreen => Color::Indexed(22),
            IndexedColor::DarkRed => Color::Indexed(52),
            IndexedColor::DarkYellow => Color::Indexed(220),
            IndexedColor::DarkGray => Color::Indexed(236),
            IndexedColor::DarkBlue => Color::Indexed(17),
        }
    }
}
