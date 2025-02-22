use ratatui::style::{Color, Style};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IndexedColor {
    DarkGreen,
    DarkRed,
    DarkYellow,
    DarkGray,
    DarkBlue,
    BrightRed,
    Gray,
}

pub trait WithColor {
    fn with_bg(self, color: impl Into<Color>) -> Self;
    fn with_fg(self, color: impl Into<Color>) -> Self;
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
            IndexedColor::BrightRed => Color::Indexed(196),
            IndexedColor::Gray => Color::Indexed(237),
        }
    }
}

impl WithColor for Style {
    fn with_bg(self, color: impl Into<Color>) -> Self {
        self.bg(color.into())
    }

    fn with_fg(self, color: impl Into<Color>) -> Self {
        self.fg(color.into())
    }
}
