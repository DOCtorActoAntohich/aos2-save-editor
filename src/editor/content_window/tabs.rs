use ratatui::{buffer::Buffer, layout::Rect, text::Text, widgets::Widget};

use crate::{style, widget::sequence};

pub struct EvenTabs<'a> {
    tabs: Vec<Text<'a>>,
    selected_index: Option<usize>,
}

impl<'a> EvenTabs<'a> {
    pub fn new(tabs: impl IntoIterator<Item = impl Into<Text<'a>>>) -> Self {
        Self {
            tabs: tabs.into_iter().map(Into::into).collect(),
            selected_index: None,
        }
    }

    pub fn select(mut self, selected: usize) -> Self {
        self.selected_index = Some(selected);
        self
    }
}

impl Widget for EvenTabs<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let Self {
            tabs,
            selected_index,
        } = self;

        sequence::VerticallySeparated {
            widgets: tabs.into_iter().enumerate().map(|(index, tab_name)| {
                let is_selected = Some(index) == selected_index;
                tab_name
                    .centered()
                    .style(style::Selection::from_is_selected(is_selected))
            }),
        }
        .render(area, buf);
    }
}
