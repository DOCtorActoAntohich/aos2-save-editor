use ratatui::{
    crossterm::event::{Event, KeyCode},
    text::Line,
};
use savefile::file::game::{background::image::BackgroundImageSheet, PlayerProgress};
use tokio::sync::watch;

use crate::{keyboard::GetKeyCode, tui::HandleEvent};

use super::{
    style::{PossibleUnlocksStyle, UnlockedStyle},
    CustomButton,
};

pub struct UnlockBackgroundsButton {
    progress_tx: watch::Sender<PlayerProgress>,
}

impl UnlockBackgroundsButton {
    pub fn new(progress_tx: watch::Sender<PlayerProgress>) -> Self {
        Self { progress_tx }
    }

    pub fn unlock_all(&mut self) {
        self.progress_tx.send_modify(|progress| {
            progress.enabled_background_image = BackgroundImageSheet::FULLY_UNLOCKED;
        });
    }
}

impl CustomButton for UnlockBackgroundsButton {
    fn as_line(&self) -> Line<'_> {
        let all_backgrounds_unlocked = self.progress_tx.borrow().enabled_background_image
            == BackgroundImageSheet::FULLY_UNLOCKED;

        if all_backgrounds_unlocked {
            Line::from("Fully unlocked").style(UnlockedStyle::default())
        } else {
            Line::from("[Press Enter to Unlock]").style(PossibleUnlocksStyle::default())
        }
    }

    fn name(&self) -> &'static str {
        "Backgrounds"
    }
}

impl HandleEvent for UnlockBackgroundsButton {
    type Error = anyhow::Error;

    fn handle_event(&mut self, event: &Event) -> Result<(), Self::Error> {
        if let Some(KeyCode::Enter) = event.key_code() {
            self.unlock_all();
        }
        Ok(())
    }
}
