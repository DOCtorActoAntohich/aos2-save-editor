pub mod content_window;
pub mod info;
pub mod profile;
pub mod progress;
pub mod statistics;

use ratatui::{buffer::Buffer, layout::Rect};

use crate::{
    savefile::{self, Savefile},
    tui::{Event, HandleEvent, VisualComponent},
};

use self::{content_window::ContentWidget, info::FullHelpToggle};

#[must_use]
pub struct App {
    content: FullHelpToggle<ContentWidget>,
    savefile: Savefile,
}

impl App {
    pub fn new(savefile: Savefile) -> Self {
        Self {
            content: FullHelpToggle::new(ContentWidget::new(&savefile)),
            savefile,
        }
    }

    pub fn handle_savefile_updates(&mut self) -> Result<(), savefile::Error> {
        self.savefile.save_all()?;

        Ok(())
    }
}

impl HandleEvent for App {
    fn handle_event(&mut self, event: &Event) {
        self.content.handle_event(event);
    }
}

impl VisualComponent for App {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        self.content.render(area, buf);
    }
}
