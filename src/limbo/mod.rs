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
        let [title_area, separator_area, all_content_area] =
            Layout::vertical(constraints).areas::<3>(area);

        Line::raw("Error")
            .centered()
            .style(
                Style::new()
                    .with_bg(IndexedColor::DarkRed)
                    .with_fg(Color::White),
            )
            .render(title_area, buf);

        Block::new()
            .borders(Borders::TOP)
            .style(
                Style::new()
                    .with_bg(IndexedColor::DarkGray)
                    .with_fg(Color::White),
            )
            .render(separator_area, buf);

        let padding_bloock = Block::new().borders(Borders::LEFT | Borders::RIGHT);
        let padded_content_area = padding_bloock.inner(all_content_area);

        padding_bloock
            .borders(Borders::empty())
            .style(
                Style::new()
                    .with_bg(IndexedColor::DarkGray)
                    .with_fg(Color::White),
            )
            .render(all_content_area, buf);
        Paragraph::new(self.error.to_string())
            .style(
                Style::new()
                    .with_bg(IndexedColor::DarkGray)
                    .with_fg(Color::White),
            )
            .render(padded_content_area, buf);
    }
}
