use player_progress::PlayerProgress;
use ratatui::{
    buffer::Buffer,
    crossterm::event::Event,
    layout::{Constraint, Rect},
    text::Text,
    widgets::{List, Paragraph, Widget},
};
use tokio::sync::watch;

use crate::{
    tui::{HandleEvent, VisualComponent},
    widget::split,
};

use super::InteratibleTabComponent;

pub struct Tab {
    progress: watch::Receiver<PlayerProgress>,
}

struct InfoText;

impl Tab {
    pub fn new(progress: watch::Receiver<PlayerProgress>) -> Self {
        Self { progress }
    }
}

impl InfoText {
    const N_LINES: u16 = 3;
    const CONSTRAINT: Constraint = Constraint::Length(Self::N_LINES);
}

impl HandleEvent for Tab {
    fn handle_event(&mut self, _: &Event) {}
}

impl VisualComponent for Tab {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let top = split::Area {
            constraint: InfoText::CONSTRAINT,
            render: |area: Rect, buf: &mut Buffer| {
                InfoText.render(area, buf);
            },
        };

        let bottom = split::Area {
            constraint: Constraint::Fill(1),
            render: |area: Rect, buf: &mut Buffer| {
                Paragraph::new("stats").centered().render(area, buf);
            },
        };

        split::Horizontal { top, bottom }.render(area, buf);
    }
}

impl InteratibleTabComponent for Tab {
    fn name(&self) -> &'static str {
        "Statistics"
    }
}

impl VisualComponent for InfoText {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let lines: [Text; InfoText::N_LINES as usize] = [
            Text::from("Statistics from singleplayer matches").centered(),
            Text::from("").centered(),
            Text::from("Normally, you unlock stuff based on these stats.").centered(),
        ];
        List::new(lines).render(area, buf);
    }
}
