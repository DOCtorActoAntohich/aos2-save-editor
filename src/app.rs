use std::{path::PathBuf, time::Instant};

use ratatui::{
    DefaultTerminal,
    crossterm::{self, event::KeyCode},
    widgets::Widget,
};

use crate::{
    editor, limbo,
    savefile::{self, Savefile},
    tui::{Event, HandleEvent, VisualComponent},
};

pub struct App {
    should_run: bool,
    screen: Screen,
    previous_event: Event,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to render to terminal")]
    Render(#[source] std::io::Error),
    #[error("Failed to reveive crossterm Event")]
    Event(#[source] std::io::Error),
}

enum Screen {
    Editor(editor::App),
    Limbo(limbo::Screen),
}

impl App {
    #[must_use]
    pub fn from_env() -> Self {
        match Savefile::from_env() {
            Ok(savefile) => Self::new_editor(savefile),
            Err(error) => Self::new_limbo(error),
        }
    }

    #[must_use]
    pub fn from_path(path: impl Into<PathBuf>) -> Self {
        match Savefile::from_path(path) {
            Ok(savefile) => Self::new_editor(savefile),
            Err(error) => Self::new_limbo(error),
        }
    }

    #[must_use]
    pub fn new_editor(savefile: Savefile) -> Self {
        Self {
            should_run: true,
            screen: Screen::Editor(editor::App::new(savefile)),
            previous_event: Event::empty(Instant::now()),
        }
    }

    #[must_use]
    pub fn new_limbo(error: savefile::Error) -> Self {
        Self {
            should_run: true,
            screen: Screen::Limbo(limbo::Screen::new(error)),
            previous_event: Event::empty(Instant::now()),
        }
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<(), Error> {
        while self.should_run {
            terminal
                .draw(|frame| frame.render_widget(&self, frame.area()))
                .map_err(Error::Render)?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<(), Error> {
        let event = self.previous_event.clone().follow_with(
            &crossterm::event::read().map_err(Error::Event)?,
            Instant::now(),
        );

        self.handle_event(&event);

        self.previous_event = event;

        if let Screen::Editor(editor) = &mut self.screen {
            if let Err(error) = editor.handle_savefile_updates() {
                self.screen = Screen::Limbo(limbo::Screen::new(error));
            }
        }

        Ok(())
    }
}

impl HandleEvent for App {
    fn handle_event(&mut self, event: &Event) {
        match (event.key_code(), &mut self.screen) {
            (Some(KeyCode::Esc), _) => {
                self.should_run = false;
            }
            (_, Screen::Editor(editor)) => editor.handle_event(event),
            (_, Screen::Limbo(screen)) => screen.handle_event(event),
        }
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        match &self.screen {
            Screen::Editor(app) => app.render(area, buf),
            Screen::Limbo(screen) => screen.render(area, buf),
        }
    }
}
