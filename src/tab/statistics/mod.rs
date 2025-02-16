use player_progress::PlayerProgress;
use ratatui::{
    buffer::Buffer,
    crossterm::event::Event,
    layout::Rect,
    widgets::{Paragraph, Widget},
};
use tokio::sync::watch;

use crate::tui::{HandleEvent, VisualComponent};

use super::InteratibleTabComponent;

pub struct Tab {
    progress: watch::Receiver<PlayerProgress>,
}

impl Tab {
    pub fn new(progress: watch::Receiver<PlayerProgress>) -> Self {
        Self { progress }
    }
}

impl HandleEvent for Tab {
    fn handle_event(&mut self, _: &Event) {}
}

impl VisualComponent for Tab {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("stats").centered().render(area, buf);
    }
}

impl InteratibleTabComponent for Tab {
    fn name(&self) -> &'static str {
        "Statistics"
    }
}
