use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style},
    text::Line,
    widgets::Widget,
};

#[derive(Debug)]
pub struct EvenlySpacedTabs<'a> {
    names: Vec<Line<'a>>,
    current: usize,
    divider: &'a str,
    selected_style: Style,
    regular_style: Style,
    divider_style: Style,
}

#[derive(Debug)]
enum TabBlock<'a> {
    Divider(Line<'a>),
    Name(Line<'a>),
}

#[derive(Debug)]
struct TabBlocksSequence<'a> {
    blocks: Vec<TabBlock<'a>>,
}

impl Default for EvenlySpacedTabs<'_> {
    fn default() -> Self {
        Self {
            names: Vec::new(),
            current: 0,
            divider: "|",
            selected_style: Style::new().fg(Color::Black).bg(Color::White),
            regular_style: Style::default(),
            divider_style: Style::default(),
        }
    }
}

impl<'a> EvenlySpacedTabs<'a> {
    pub fn new<I>(iter: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Line<'a>>,
    {
        Self {
            names: iter.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }

    pub fn select(mut self, index: usize) -> Self {
        self.current = index;
        self
    }
}

impl Widget for EvenlySpacedTabs<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let stylized_tabs = self
            .names
            .into_iter()
            .enumerate()
            .map(|(index, line)| {
                if index == self.current {
                    line.style(self.selected_style)
                } else {
                    line.style(self.regular_style)
                }
            })
            .map(|line| line.centered());
        let divider = Line::from(self.divider).style(self.divider_style);

        let tab_blocks = TabBlocksSequence::alternating_from_tabs(stylized_tabs, divider);

        tab_blocks.render(area, buf);
    }
}

impl<'a> TabBlocksSequence<'a> {
    pub fn alternating_from_tabs<I>(tab_names: I, divider: Line<'a>) -> Self
    where
        I: Iterator<Item = Line<'a>>,
    {
        let blocks: Vec<_> = tab_names
            .flat_map(|tab_line| [TabBlock::Divider(divider.clone()), TabBlock::Name(tab_line)])
            .chain(std::iter::once(TabBlock::Divider(divider.clone())))
            .collect();

        Self { blocks }
    }
}

impl TabBlock<'_> {
    pub fn constraint(&self) -> Constraint {
        match self {
            TabBlock::Divider(divider) => Constraint::Length(divider.width() as u16),
            TabBlock::Name(_) => Constraint::Fill(1),
        }
    }
}

impl Widget for TabBlocksSequence<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let constraints = self.blocks.iter().map(TabBlock::constraint);
        let areas = Layout::horizontal(constraints).split(area);

        self.blocks
            .into_iter()
            .zip(areas.iter())
            .for_each(|(block, &block_area)| block.render(block_area, buf));
    }
}

impl Widget for TabBlock<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        match self {
            TabBlock::Divider(divider) => divider.render(area, buf),
            TabBlock::Name(line) => line.render(area, buf),
        }
    }
}

#[cfg(test)]
mod tests {
    use ratatui::{
        buffer::Buffer,
        layout::Rect,
        style::{Color, Style},
        widgets::Widget,
    };

    use super::EvenlySpacedTabs;

    #[rstest::rstest]
    fn proper_spacing() {
        let mut expected = Buffer::with_lines(vec!["|  a  |  b  |  c  |  d  |"]);
        expected.set_style(
            Rect {
                x: 1,
                y: 0,
                width: 5,
                height: 1,
            },
            Style::new().fg(Color::Black).bg(Color::White),
        );

        let area = Rect {
            x: 0,
            y: 0,
            width: 25,
            height: 1,
        };
        let mut actual = Buffer::empty(area);
        let tabs = EvenlySpacedTabs::new(vec!["a", "b", "c", "d"]);
        tabs.render(area, &mut actual);

        assert_eq!(expected, actual);
    }
}
