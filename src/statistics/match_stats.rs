use player_progress::PlayerProgress;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Text,
    widgets::{List, Widget},
};
use tokio::sync::watch;

use crate::tui::VisualComponent;

pub struct SingleplayerMatchStats {
    progress: watch::Receiver<PlayerProgress>,
}

impl SingleplayerMatchStats {
    pub fn new(progress: watch::Receiver<PlayerProgress>) -> Self {
        Self { progress }
    }
}

impl VisualComponent for SingleplayerMatchStats {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let progress = self.progress.borrow();

        let items = [
            Text::from("Easy arcade 1CCs:"),
            Text::from(format!("    {}", progress.n_arcade_easy_1ccs)),
            Text::from(""),
            Text::from("Medium arcade 1CCs:"),
            Text::from(format!("    {}", progress.n_arcade_medium_1ccs)),
            Text::from(""),
            Text::from("Hard arcade 1CCs:"),
            Text::from(format!("    {}", progress.n_arcade_hard_1ccs)),
            Text::from(""),
            Text::from("Story 1CCs:"),
            Text::from(format!("    {}", progress.n_story_1ccs)),
            Text::from(""),
            Text::from("Total matches won:"),
            Text::from(format!("    {}", progress.n_singleplayer_match_wins)),
        ];
        List::new(items).render(area, buf);
    }
}
