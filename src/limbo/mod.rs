use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{
    savefile,
    style::{IndexedColor, WithColor},
    tui::{HandleEvent, VisualComponent},
};

pub struct Screen {
    error: savefile::Error,
}

impl Screen {
    pub fn new(error: savefile::Error) -> Self {
        Self { error }
    }
}

impl HandleEvent for Screen {
    fn handle_event(&mut self, _: &crate::tui::Event) {}
}

impl VisualComponent for Screen {
    fn render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let constraints = [
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ];
        let [title, empty, content] = Layout::vertical(constraints).areas::<3>(area);

        Line::raw("Error")
            .centered()
            .style(
                Style::new()
                    .with_bg(IndexedColor::DarkRed)
                    .with_fg(Color::White),
            )
            .render(title, buf);

        Block::new()
            .borders(Borders::TOP)
            .style(
                Style::new()
                    .with_bg(IndexedColor::DarkGray)
                    .with_fg(Color::White),
            )
            .render(empty, buf);

        Paragraph::new(self.error.to_string())
            .style(
                Style::new()
                    .with_bg(IndexedColor::DarkGray)
                    .with_fg(Color::White),
            )
            .centered()
            .render(content, buf);
    }
}
