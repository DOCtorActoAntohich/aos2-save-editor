mod collection;

use online_profile::PlayerOnlineProfile;
use ratatui::{
    buffer::Buffer,
    crossterm::event::Event,
    layout::Rect,
    widgets::{Paragraph, Widget},
};
use tokio::sync::watch;

use crate::{
    info::content_window::InteratibleTabComponent,
    tui::{event::GetKeyCode, HandleEvent, VisualComponent},
};

pub struct Tab {
    profile: watch::Sender<PlayerOnlineProfile>,
}

impl Tab {
    pub fn new(profile: watch::Sender<PlayerOnlineProfile>) -> Self {
        Self { profile }
    }
}

impl HandleEvent for Tab {
    fn handle_event(&mut self, event: &Event) {
        match event.key_code() {
            _ => (),
        }
    }
}

impl VisualComponent for Tab {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.profile.borrow().title_color.to_string())
            .centered()
            .render(area, buf);
    }
}

impl InteratibleTabComponent for Tab {
    fn name(&self) -> &'static str {
        "Online"
    }
}
