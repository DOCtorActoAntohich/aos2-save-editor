pub mod info;
pub mod profile;
pub mod progress;
pub mod statistics;

use std::time::Instant;

use ratatui::{
    buffer::Buffer,
    crossterm::{self, event::KeyCode},
    layout::Rect,
    widgets::Widget,
    DefaultTerminal, Frame,
};

use crate::{
    savefile::Savefile,
    tui::{Event, HandleEvent, VisualComponent},
};

use self::info::{content_window::ContentWidget, info_toggle::FullHelpToggle};

#[must_use]
pub struct EditorApp {
    should_run: bool,
    content: FullHelpToggle<ContentWidget>,
    previous_event: Event,
    savefile: Savefile,
}

impl EditorApp {
    pub fn new(savefile: Savefile) -> Self {
        Self {
            should_run: true,
            content: FullHelpToggle::new(ContentWidget::new(&savefile)),
            previous_event: Event::empty(Instant::now()),
            savefile,
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
        let event = self
            .previous_event
            .clone()
            .follow_with(&crossterm::event::read()?, Instant::now());

        self.handle_event(&event);

        self.handle_savefile_updates()?;

        self.previous_event = event;

        Ok(())
    }

    fn handle_savefile_updates(&mut self) -> anyhow::Result<()> {
        self.savefile.save_all()?;

        Ok(())
    }

    fn exit(&mut self) {
        self.should_run = false;
    }
}

impl HandleEvent for EditorApp {
    fn handle_event(&mut self, event: &Event) {
        match event.key_code() {
            Some(KeyCode::Esc) => self.exit(),
            _ => self.content.handle_event(event),
        }
    }
}

impl Widget for &EditorApp {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        self.content.render(area, buf);
    }
}
