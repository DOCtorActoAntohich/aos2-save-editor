use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};

use super::separator;

pub struct VerticallySeparated<I, W>
where
    I: Iterator<Item = W>,
    W: Widget,
{
    pub widgets: I,
}

enum ToDraw<W> {
    Widget(W),
    Separator,
}

impl<I, W> Widget for VerticallySeparated<I, W>
where
    I: Iterator<Item = W>,
    W: Widget,
{
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let Self { widgets } = self;

        let to_draw: Vec<ToDraw<W>> = widgets
            .flat_map(|widget| [ToDraw::Separator, ToDraw::Widget(widget)])
            .skip(1)
            .collect();

        let constraints = to_draw.iter().map(|to_draw| match to_draw {
            ToDraw::Widget(_) => Constraint::Fill(1),
            ToDraw::Separator => separator::Vertical::CONSTRAINT,
        });

        Layout::horizontal(constraints)
            .split(area)
            .iter()
            .zip(to_draw)
            .for_each(|(&area, to_draw)| match to_draw {
                ToDraw::Widget(widget) => {
                    widget.render(area, buf);
                }
                ToDraw::Separator => separator::Vertical.render(area, buf),
            });
    }
}
