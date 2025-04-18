use std::fmt::Display;

use ratatui::crossterm::event::KeyCode;

use crate::{
    collection::{RadioButtonIndex, TextSearch},
    editor::profile::widget::{RadioButtonsContent, RadioButtonsTable},
    savefile::profile,
    tui::{Event, HandleEvent},
};

use super::Table;

pub trait Item: Sized + Clone + Copy + PartialEq + Eq + Display + Default + Send {
    fn members() -> Vec<Self>;
}

pub struct Generic<T> {
    data: profile::Modify<T>,
    name: String,
    hovered: usize,
}

impl<T: Item> Generic<T> {
    pub fn new(name: impl Into<String>, data: profile::Modify<T>) -> Self {
        let hovered = T::members()
            .into_iter()
            .enumerate()
            .find_map(|(index, value)| (value == data.get()).then_some(index))
            .unwrap_or_default();
        Self {
            data,
            name: name.into(),
            hovered,
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

                let new = items.get(new_selected).copied().unwrap_or_default();
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

impl Item for online_profile::title::Character {
    fn members() -> Vec<Self> {
        Self::members().to_vec()
    }
}

impl Item for online_profile::title::Color {
    fn members() -> Vec<Self> {
        Self::members().into()
    }
}

impl Item for online_profile::title::Text {
    fn members() -> Vec<Self> {
        Self::members().into()
    }
}

impl Item for online_profile::avatar::Character {
    fn members() -> Vec<Self> {
        Self::members().to_vec()
    }
}

impl Item for online_profile::avatar::Background {
    fn members() -> Vec<Self> {
        Self::members().to_vec()
    }
}
