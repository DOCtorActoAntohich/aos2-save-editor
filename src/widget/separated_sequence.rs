use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};

use super::separator;

pub struct VerticallySeparatedSequence<I, W>
where
    I: Iterator<Item = W>,
    W: Widget,
{
    pub items: I,
}

enum ToDraw<W> {
    Item(W),
    Separator,
}

impl<I, W> Widget for VerticallySeparatedSequence<I, W>
where
    I: Iterator<Item = W>,
    W: Widget,
{
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let Self { items } = self;

        let to_draw: Vec<ToDraw<W>> = items
            .flat_map(|item| [ToDraw::Separator, ToDraw::Item(item)])
            .skip(1)
            .collect();

        let constraints = to_draw.iter().map(|thing| match thing {
            ToDraw::Item(_) => Constraint::Fill(1),
            ToDraw::Separator => separator::Vertical::CONSTRAINT,
        });

        Layout::horizontal(constraints)
            .split(area)
            .iter()
            .zip(to_draw)
            .for_each(|(&area, thing)| match thing {
                ToDraw::Item(item) => {
                    item.render(area, buf);
                }
                ToDraw::Separator => separator::Vertical.render(area, buf),
            });
    }
}
