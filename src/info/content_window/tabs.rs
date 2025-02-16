use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::Text,
    widgets::Widget,
};

use crate::{style, widget::separator};

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
        enum Thing<'a> {
            Line(separator::Vertical),
            Name(usize, Text<'a>),
        }

        let Self {
            tabs: widgets,
            selected_index,
        } = self;

        let to_draw: Vec<Thing<'_>> = std::iter::once(Thing::Line(separator::Vertical))
            .chain(widgets.into_iter().enumerate().flat_map(|(index, widget)| {
                [Thing::Name(index, widget), Thing::Line(separator::Vertical)]
            }))
            .collect();

        let constraints = to_draw.iter().map(|to_draw| match to_draw {
            Thing::Line(_) => separator::Vertical::CONSTRAINT,
            Thing::Name(_, _) => Constraint::Fill(1),
        });
        Layout::horizontal(constraints)
            .split(area)
            .into_iter()
            .zip(to_draw)
            .for_each(|(&area, to_draw)| match to_draw {
                Thing::Line(line) => line.render(area, buf),
                Thing::Name(index, name) => {
                    let is_selected = Some(index) == selected_index;
                    name.centered()
                        .style(style::Selection::from_is_selected(is_selected))
                        .render(area, buf);
                }
            });
    }
}
