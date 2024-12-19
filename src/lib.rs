mod component;
mod tui;
mod widget;

use ratatui::{
    buffer::Buffer,
    crossterm::{
        self,
        event::{Event, KeyCode, KeyEventKind},
    },
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
    DefaultTerminal, Frame,
};

use crate::{
    component::{
        content_window::ContentWidget, full_help_toggle::FullHelpToggle, title_header::TitleHeader,
    },
    tui::{HandleEvent, VisualComponent},
};

#[derive(Debug)]
pub struct EditorApp {
    should_run: bool,
    content: FullHelpToggle<ContentWidget>,
}

impl EditorApp {
    pub fn new() -> Self {
        Self {
            should_run: true,
            content: FullHelpToggle::new(ContentWidget::new()),
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

        self.handle_event(event)
            .and_then(|event| self.content.handle_event(event))?;

        Ok(())
    }

    fn exit(&mut self) {
        self.should_run = false
    }
}

impl HandleEvent for EditorApp {
    type Error = anyhow::Error;

    fn handle_event(&mut self, event: Event) -> Result<Event, Self::Error> {
        match event {
            Event::Key(key_event)
                if key_event.kind == KeyEventKind::Press
                    && key_event.code == KeyCode::Char('q') =>
            {
                self.exit();
            }
            _ => (),
        }

        Ok(event)
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
