use player_progress::{MusicTracks, PlayerProgress};
use ratatui::{
    crossterm::event::{Event, KeyCode},
    text::Line,
};
use tokio::sync::watch;

use crate::{keyboard::GetKeyCode, tui::HandleEvent};

use super::{style, CustomButton};

pub struct Button {
    progress_tx: watch::Sender<PlayerProgress>,
}

impl Button {
    pub fn new(progress_tx: watch::Sender<PlayerProgress>) -> Self {
        Self { progress_tx }
    }

    pub fn unlock_all(&mut self) {
        self.progress_tx.send_modify(|progress| {
            progress.background_music = MusicTracks::ALL;
        });
    }
}

impl CustomButton for Button {
    fn as_line(&self) -> Line<'_> {
        let all_music_unlocked = self.progress_tx.borrow().background_music == MusicTracks::ALL;

        if all_music_unlocked {
            Line::from("Fully unlocked").style(style::Unlocked::default())
        } else {
            Line::from("[Press Enter to Unlock]").style(style::PossibleUnlocks::default())
        }
    }

    fn name(&self) -> &'static str {
        "Music"
    }
}

impl HandleEvent for Button {
    type Error = anyhow::Error;

    fn handle_event(&mut self, event: &Event) -> Result<(), Self::Error> {
        if let Some(KeyCode::Enter) = event.key_code() {
            self.unlock_all();
        }
        Ok(())
    }
}
