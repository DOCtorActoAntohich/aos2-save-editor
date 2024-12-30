pub mod signal;

mod component;
mod keyboard;
mod tui;
mod widget;

use anyhow::Context;
use aos2_env::AoS2Paths;
use component::tab::{character::CharacterTab, empty::EmptyTab, TabComponent};
use crossterm::event::EventStream;
use futures::StreamExt;
use keyboard::GetKeyCode;
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
use savefile::file::game::PlayerProgress;
use tokio::sync::watch;

use crate::{
    component::{
        content_window::ContentWidget, full_help_toggle::FullHelpToggle, title_header::TitleHeader,
    },
    tui::{HandleEvent, VisualComponent},
};

pub struct EditorApp {
    should_run: bool,
    content: FullHelpToggle<ContentWidget>,
    paths: AoS2Paths,
    progress_rx: watch::Receiver<PlayerProgress>,
    event_stream: EventStream,
}

impl EditorApp {
    pub fn new(paths: AoS2Paths, progress: PlayerProgress) -> Self {
        let (progress_tx, progress_rx) = watch::channel(progress);
        let tabs: [Box<dyn TabComponent>; 2] =
            [Box::new(EmptyTab), Box::new(CharacterTab::new(progress_tx))];
        Self {
            should_run: true,
            content: FullHelpToggle::new(ContentWidget::new(tabs)),
            paths,
            progress_rx,
            event_stream: EventStream::new(),
        }
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> anyhow::Result<()> {
        terminal.clear()?;

        while self.should_run {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events().await?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    async fn handle_events(&mut self) -> anyhow::Result<()> {
        let event = self
            .event_stream
            .next()
            .await
            .context("No more events can be received")?
            .context("Failed to receive event")?;

        self.handle_event(&event)?;

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
                .save_to_file(&self.paths.game_sys)
                .context("Failed to save player progress file")?;
        }

        Ok(())
    }

    fn exit(&mut self) {
        self.should_run = false
    }
}

impl HandleEvent for EditorApp {
    type Error = anyhow::Error;

    fn handle_event(&mut self, event: &Event) -> Result<(), Self::Error> {
        match event.key_code() {
            Some(KeyCode::Char('q')) => self.exit(),
            _ => self.content.handle_event(event)?,
        }

        Ok(())
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
