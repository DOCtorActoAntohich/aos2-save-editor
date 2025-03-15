use player_progress::SingleplayerWins;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Text,
    widgets::{List, Widget},
};

use crate::{savefile::progress, tui::VisualComponent};

pub struct SingleplayerMatchStats {
    wins: progress::Read<SingleplayerWins>,
}

impl SingleplayerMatchStats {
    pub fn new(wins: progress::Read<SingleplayerWins>) -> Self {
        Self { wins }
    }
}

impl VisualComponent for SingleplayerMatchStats {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let wins = self.wins.get();

        let items = [
            Text::from("Easy arcade 1CCs:"),
            Text::from(format!("    {}", wins.n_arcade_easy_1ccs)),
            Text::from(""),
            Text::from("Medium arcade 1CCs:"),
            Text::from(format!("    {}", wins.n_arcade_medium_1ccs)),
            Text::from(""),
            Text::from("Hard arcade 1CCs:"),
            Text::from(format!("    {}", wins.n_arcade_hard_1ccs)),
            Text::from(""),
            Text::from("Story 1CCs:"),
            Text::from(format!("    {}", wins.n_story_1ccs)),
            Text::from(""),
            Text::from("Total matches won:"),
            Text::from(format!("    {}", wins.total)),
        ];
        List::new(items).render(area, buf);
    }
}
