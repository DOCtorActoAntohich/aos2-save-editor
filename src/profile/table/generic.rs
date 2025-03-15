use online_profile::MembersList;
use ratatui::crossterm::event::KeyCode;

use crate::{
    collection::{RadioButtonIndex, TextSearch},
    profile::widget::{RadioButtonsContent, RadioButtonsTable},
    savefile::profile,
    tui::{Event, HandleEvent},
};

use super::Table;

pub trait Item: MembersList {}

impl<T> Item for T where T: MembersList {}

pub struct Generic<T> {
    data: profile::Modify<T>,
    name: String,
    hovered: usize,
}

impl<T: Item> Generic<T> {
    pub fn new(name: impl Into<String>, data: profile::Modify<T>) -> Self {
        Self {
            data,
            name: name.into(),
            hovered: 0,
        }
    }

    fn selected_index(&self) -> Option<usize> {
        let current = self.data.get();
        T::members()
            .into_iter()
            .enumerate()
            .find_map(|(index, item)| (item == current).then_some(index))
    }
}

impl<T: Item> HandleEvent for Generic<T> {
    fn handle_event(&mut self, event: &Event) {
        let items = T::members();
        let index = RadioButtonIndex::from_collection(&items)
            .with_selected(self.selected_index().unwrap_or_default())
            .with_hovered(self.hovered);
        match event.key_code() {
            Some(KeyCode::Up) => {
                self.hovered = index.hover_previous().hovered().unwrap_or_default();
            }
            Some(KeyCode::Down) => {
                self.hovered = index.hover_next().hovered().unwrap_or_default();
            }
            Some(KeyCode::Home) => {
                self.hovered = index.hover_first().hovered().unwrap_or_default();
            }
            Some(KeyCode::End) => {
                self.hovered = index.hover_last().hovered().unwrap_or_default();
            }
            Some(KeyCode::Enter) => {
                let new_selected = index.select_hovered().selected().unwrap_or_default();

                let new = items.get(new_selected).cloned().unwrap_or_default();
                self.data.send(new);
            }
            Some(KeyCode::Char(_)) => {
                self.hovered = TextSearch::in_collection(&items)
                    .with_text(event.accumulated_input())
                    .unwrap_or(self.hovered);
            }
            _ => (),
        }
    }
}

impl<T: Item> Table for Generic<T> {
    fn as_widget(&self, is_active: bool) -> RadioButtonsTable<'_> {
        RadioButtonsTable {
            name: self.name.as_str().into(),
            content: RadioButtonsContent::new(
                T::members().into_iter().map(|item| item.to_string()),
            )
            .with_hovered(self.hovered)
            .with_selected(self.selected_index().unwrap_or(usize::MAX)),
            is_active,
        }
    }
}
