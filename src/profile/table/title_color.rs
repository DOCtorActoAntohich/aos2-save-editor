use online_profile::{title, PlayerOnlineProfile};
use ratatui::crossterm::event::KeyCode;
use tokio::sync::watch;

use crate::{
    collection::RadioButtonArray,
    profile::widget::TableContent,
    tui::{Event, HandleEvent},
};

use super::InteractibleTable;

pub struct Table {
    profile: watch::Sender<PlayerOnlineProfile>,
    colors: RadioButtonArray<title::Color, { title::Color::MEMBERS_COUNT }>,
}

impl Table {
    pub fn new(profile: watch::Sender<PlayerOnlineProfile>) -> Self {
        let colors = title::Color::members();
        let current_color = profile.borrow().title_color;
        let selected = colors
            .iter()
            .enumerate()
            .find_map(|(index, &value)| {
                if value == current_color {
                    Some(index)
                } else {
                    None
                }
            })
            .unwrap_or_default();

        Self {
            profile,
            colors: RadioButtonArray::new(colors, selected),
        }
    }
}

impl HandleEvent for Table {
    fn handle_event(&mut self, event: &Event) {
        match event.key_code() {
            Some(KeyCode::Up) => self.colors.hover_previous(),
            Some(KeyCode::Down) => self.colors.hover_next(),
            Some(KeyCode::Enter) => {
                self.colors.select_current();
                self.profile
                    .send_modify(|profile| profile.title_color = *self.colors.current());
            }
            Some(KeyCode::Char(_)) => {
                let jump_to = self
                    .colors
                    .find_by_text(event.accumulated_input())
                    .unwrap_or(self.colors.hovered_index());
                self.colors.hover_at(jump_to);
            }
            _ => (),
        }
    }
}

impl InteractibleTable for Table {
    fn name(&self) -> &'static str {
        "Color"
    }

    fn content_widget(&self) -> TableContent {
        TableContent::new(
            self.colors
                .to_array()
                .into_iter()
                .map(|color| color.to_string()),
        )
        .with_hovered(self.colors.hovered_index())
        .with_selected(self.colors.selected_index())
    }
}
