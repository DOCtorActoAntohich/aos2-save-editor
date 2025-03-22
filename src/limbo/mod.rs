use ratatui::widgets::{Paragraph, Widget};

use crate::{
    savefile,
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
        Paragraph::new("ded").render(area, buf);
    }
}
