use ratatui::style::{Color, Style};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Selection {
    Selected,
    Unselected,
}

impl Selection {
    pub fn from_is_selected(is_selected: bool) -> Self {
        if is_selected {
            Self::Selected
        } else {
            Self::Unselected
        }
    }
}

impl From<Selection> for Style {
    fn from(value: Selection) -> Self {
        match value {
            Selection::Selected => Style::new().bg(Color::White).fg(Color::Black).bold(),
            Selection::Unselected => Style::new().bg(Color::Black).fg(Color::White),
        }
    }
}
