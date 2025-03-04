use player_progress::{MusicTracks, PlayerProgress, Status};
use ratatui::crossterm::event::KeyCode;
use tokio::sync::watch;

use crate::{
    collection::SelectibleArray,
    progress::widget::{TogglesContent, TogglesTable},
    tui::{Event, HandleEvent},
};

use super::InteractibleTable;

pub struct Table {
    progress: watch::Sender<PlayerProgress>,
    music: SelectibleArray<Status, { MusicTracks::AMOUNT }>,
}

impl Table {
    pub fn new(progress: watch::Sender<PlayerProgress>) -> Self {
        let music = progress.borrow().background_music.to_array();
        Self {
            progress,
            music: SelectibleArray::new(music),
        }
    }
}

impl HandleEvent for Table {
    fn handle_event(&mut self, event: &Event) {
        match event.key_code() {
            Some(KeyCode::Up) => self.music.select_previous(),
            Some(KeyCode::Down) => self.music.select_next(),
            Some(KeyCode::Home) => self.music.select_first(),
            Some(KeyCode::End) => self.music.select_last(),
            Some(KeyCode::Enter) => {
                self.music.modify_current(|status| *status = !*status);
                self.progress.send_modify(|progress| {
                    progress.background_music = self.music.to_array().into();
                });
            }
            _ => (),
        }
    }
}

impl InteractibleTable for Table {
    fn as_widget(&self, is_active: bool) -> TogglesTable<'_> {
        TogglesTable {
            name: "Music".into(),
            content: TogglesContent::new(self.progress.borrow().background_music.iter())
                .with_current(self.music.current_index()),
            is_active,
        }
    }
}
