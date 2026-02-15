use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph, Widget, Wrap};

use crate::savefile;
use crate::style::{IndexedColor, WithColor};
use crate::tui::{HandleEvent, VisualComponent};

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
    fn render(&self, full_area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let constraints = [
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ];
        let [title_area, separator_area, all_content_area] =
            Layout::vertical(constraints).areas::<3>(full_area);

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

        match &self.error {
            savefile::Error::Env(error) => draw_env(error, padded_content_area, buf),
            savefile::Error::Progress(error) => {
                draw_progress_error(error, padded_content_area, buf);
            }
            savefile::Error::Profile(error) => draw_profile_error(error, padded_content_area, buf),
        }
    }
}

fn draw_env(error: &aos2_env::Error, area: Rect, buf: &mut Buffer) {
    match error {
        aos2_env::Error::Home => {
            const REASON: &str = "This usually happens due to poor system configuration.";
            const TIP: &str = "You can try manually specifying save folder location:";

            let example = aos2_env::saves_location(aos2_env::EXAMPLE_HOME);

            let text = format!(
                "{error}\n\n{REASON}\n\n{TIP}\n\n{example}",
                example = example.display()
            );

            draw_error_paragraph(text, area, buf);
        }
    }
}

fn draw_progress_error(error: &binary_file::Error, area: Rect, buf: &mut Buffer) {
    let text = format!(
        "Error handling the progress file, aka `{filename}`\n\n{error}",
        filename = player_progress::PlayerProgress::FILE_NAME
    );
    draw_error_paragraph(text, area, buf);
}

fn draw_profile_error(error: &binary_file::Error, area: Rect, buf: &mut Buffer) {
    let text = format!(
        "Error handling the online profile file, aka `{filename}`\n\n{error}",
        filename = online_profile::PlayerOnlineProfile::FILE_NAME
    );
    draw_error_paragraph(text, area, buf);
}

fn draw_error_paragraph(text: String, area: Rect, buf: &mut Buffer) {
    Paragraph::new(text)
        .style(
            Style::new()
                .with_bg(IndexedColor::DarkGray)
                .with_fg(Color::White),
        )
        .wrap(Wrap { trim: false })
        .render(area, buf);
}
