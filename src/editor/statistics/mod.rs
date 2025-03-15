mod character_stats;
mod match_stats;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    text::Text,
    widgets::{List, Widget},
};

use crate::{
    editor::info::content_window::InteratibleTabComponent,
    savefile::Savefile,
    tui::{Event, HandleEvent, VisualComponent},
    widget::split,
};

use self::{character_stats::CharacterStats, match_stats::SingleplayerMatchStats};

pub struct Tab {
    character_stats: CharacterStats,
    match_stats: SingleplayerMatchStats,
}

struct InfoText;

impl Tab {
    #[must_use]
    pub fn new(savefile: &Savefile) -> Self {
        let completion_stats = savefile.progress().read_completion_stats();
        let wins = savefile.progress().read_wins();
        Self {
            character_stats: CharacterStats::new(completion_stats),
            match_stats: SingleplayerMatchStats::new(wins),
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
