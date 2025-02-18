mod character_stats;
mod match_stats;

use player_progress::PlayerProgress;
use ratatui::{
    buffer::Buffer,
    crossterm::event::Event,
    layout::{Constraint, Rect},
    text::Text,
    widgets::{List, Widget},
};
use tokio::sync::watch;

use crate::{
    tui::{HandleEvent, VisualComponent},
    widget::split,
};

use self::{character_stats::CharacterStats, match_stats::SingleplayerMatchStats};

use super::InteratibleTabComponent;

pub struct Tab {
    character_stats: CharacterStats,
    match_stats: SingleplayerMatchStats,
}

struct InfoText;

impl Tab {
    pub fn new(progress: watch::Receiver<PlayerProgress>) -> Self {
        Self {
            character_stats: CharacterStats::new(progress.clone()),
            match_stats: SingleplayerMatchStats::new(progress),
        }
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

        let left = split::Area {
            constraint: Constraint::Fill(3),
            render: |area: Rect, buf: &mut Buffer| {
                self.match_stats.render(area, buf);
            },
        };
        let right = split::Area {
            constraint: Constraint::Fill(7),
            render: |area: Rect, buf: &mut Buffer| {
                self.character_stats.render(area, buf);
            },
        };

        let bottom = split::Area {
            constraint: Constraint::Fill(1),
            render: |area: Rect, buf: &mut Buffer| {
                split::Vertical { left, right }.render(area, buf);
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
