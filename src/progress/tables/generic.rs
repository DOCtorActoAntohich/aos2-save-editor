use player_progress::{Status, StatusSequence};
use ratatui::crossterm::event::KeyCode;
use tokio::sync::watch;

use crate::{
    collection::HoveringIndex,
    progress::widget::{TogglesContent, TogglesTable},
    tui::{Event, HandleEvent},
};

pub trait Item: Send + Sync + AsRef<[Status]> + StatusSequence {}

impl<T> Item for T where T: Send + Sync + AsRef<[Status]> + StatusSequence {}

pub struct Table<T: Item> {
    items: watch::Sender<T>,
    current_index: usize,
    name: String,
}

impl<T: Item> Table<T> {
    pub fn new(name: impl Into<String>, items: watch::Sender<T>) -> Self {
        Self {
            name: name.into(),
            items,
            current_index: 0,
        }
    }
}

impl<T: Item> HandleEvent for Table<T> {
    fn handle_event(&mut self, event: &Event) {
        let hover = HoveringIndex::from_collection(&self.items.borrow().as_ref())
            .with_current(self.current_index);
        match event.key_code() {
            Some(KeyCode::Up) => {
                self.current_index = hover.previous().into_index().unwrap_or_default()
            }
            Some(KeyCode::Down) => {
                self.current_index = hover.next().into_index().unwrap_or_default()
            }
            Some(KeyCode::Home) => {
                self.current_index = hover.first().into_index().unwrap_or_default()
            }
            Some(KeyCode::End) => {
                self.current_index = hover.last().into_index().unwrap_or_default()
            }
            Some(KeyCode::Enter) => {
                self.items
                    .send_modify(|sequence| sequence.toggle_at(self.current_index));
            }
            _ => (),
        }
    }
}

impl<T: Item> super::Table for Table<T> {
    fn as_widget(&self, is_active: bool) -> TogglesTable<'_> {
        TogglesTable {
            name: self.name.as_str().into(),
            content: TogglesContent::new(self.items.borrow().list())
                .with_current(self.current_index),
            is_active,
        }
    }
}
