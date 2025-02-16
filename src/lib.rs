#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

mod collection;
mod info;
mod keyboard;
mod style;
mod tab;
mod tui;
mod widget;

use anyhow::Context;
use aos2_env::AoS2Env;
use player_progress::PlayerProgress;
use ratatui::{
    buffer::Buffer,
    crossterm::{
        self,
        event::{Event, KeyCode},
    },
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
    DefaultTerminal, Frame,
};
use tokio::sync::watch;

use crate::{
    info::full_help_toggle::FullHelpToggle,
    keyboard::GetKeyCode,
    tui::{HandleEvent, VisualComponent},
};

use self::info::{content_window::ContentWidget, title_header::TitleHeader};

#[must_use]
pub struct EditorApp {
    should_run: bool,
    content: FullHelpToggle<ContentWidget>,
    aos2_env: AoS2Env,
    progress_rx: watch::Receiver<PlayerProgress>,
}

impl EditorApp {
    pub fn new(aos2_env: AoS2Env, player_progress: PlayerProgress) -> Self {
        let (progress_tx, progress_rx) = watch::channel(player_progress);

        Self {
            should_run: true,
            content: FullHelpToggle::new(ContentWidget::new(progress_tx)),
            aos2_env,
            progress_rx,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> anyhow::Result<()> {
        while self.should_run {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> anyhow::Result<()> {
        let event = crossterm::event::read()?;
        self.handle_event(&event);

        self.handle_savefile_updates()?;

        Ok(())
    }

    fn handle_savefile_updates(&mut self) -> anyhow::Result<()> {
        if self
            .progress_rx
            .has_changed()
            .context("Invariant Broken: Save tracking channel closed")?
        {
            let progress = self.progress_rx.borrow_and_update();
            progress
                .save(&self.aos2_env)
                .context("Failed to save player progress file")?;
        }

        Ok(())
    }

    fn exit(&mut self) {
        self.should_run = false;
    }
}

impl HandleEvent for EditorApp {
    fn handle_event(&mut self, event: &Event) {
        match event.key_code() {
            Some(KeyCode::Char('q')) => self.exit(),
            _ => self.content.handle_event(event),
        }
    }
}

impl Widget for &EditorApp {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        const CONSTRAINTS: [Constraint; 2] = [TitleHeader::CONSTRAINT, ContentWidget::CONSTRAINT];

        let [header_area, central_area] = Layout::vertical(CONSTRAINTS).areas(area);

        TitleHeader.render(header_area, buf);
        self.content.render(central_area, buf);
    }
}
