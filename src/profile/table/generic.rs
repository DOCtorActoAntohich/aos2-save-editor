use std::fmt::Display;

use online_profile::PlayerOnlineProfile;
use ratatui::crossterm::event::KeyCode;
use tokio::sync::watch;

use crate::{
    collection::RadioButtonArray,
    profile::widget::{RadioButtonsContent, RadioButtonsTable},
    tui::{Event, HandleEvent},
};

use super::Table;

pub trait OnSelectedFn<T>: Fn(&mut PlayerOnlineProfile, &T) + Send {}
pub trait Item: Send + Display + PartialEq + Clone {}

impl<T, F> OnSelectedFn<T> for F where F: Fn(&mut PlayerOnlineProfile, &T) + Send {}

impl<T> Item for T where T: Send + Display + PartialEq + Clone {}

pub struct Params<T, F, const LENGTH: usize> {
    pub profile: watch::Sender<PlayerOnlineProfile>,
    pub items: [T; LENGTH],
    pub current: T,
    pub on_selected: F,
    pub name: String,
}

/// After creating 5 identical structs with identical behavior
/// but different name for `items` field, I made this silly thing.
///
/// Now this is another useless abstraction,
/// but it's contained and used in 5 (>3) different places,
/// so this shit is justified. Hooray I guess.
pub struct Generic<T, const LENGTH: usize> {
    profile: watch::Sender<PlayerOnlineProfile>,
    items: RadioButtonArray<T, LENGTH>,
    on_selected: Box<dyn OnSelectedFn<T>>,
    name: String,
}

impl<T, const N: usize> Generic<T, N>
where
    T: Item,
{
    pub fn new<F>(
        Params {
            profile,
            items,
            current,
            on_selected,
            name,
        }: Params<T, F, N>,
    ) -> Self
    where
        F: OnSelectedFn<T> + 'static,
    {
        let selected = items
            .iter()
            .enumerate()
            .find_map(
                |(index, value)| {
                    if value == &current {
                        Some(index)
                    } else {
                        None
                    }
                },
            )
            .unwrap_or_default();

        Self {
            profile,
            items: RadioButtonArray::new(items, selected),
            on_selected: Box::new(on_selected),
            name,
        }
    }
}

impl<T, const N: usize> HandleEvent for Generic<T, N>
where
    T: Item,
{
    fn handle_event(&mut self, event: &Event) {
        match event.key_code() {
            Some(KeyCode::Up) => self.items.hover_previous(),
            Some(KeyCode::Down) => self.items.hover_next(),
            Some(KeyCode::Home) => self.items.hover_to_start(),
            Some(KeyCode::End) => self.items.hover_to_end(),
            Some(KeyCode::Enter) => {
                self.items.select_current();
                self.profile.send_modify(|profile| {
                    (self.on_selected)(profile, self.items.current());
                });
            }
            Some(KeyCode::Char(_)) => {
                let jump_to = self
                    .items
                    .find_by_text(event.accumulated_input())
                    .unwrap_or(self.items.hovered_index());
                self.items.hover_at(jump_to);
            }
            _ => (),
        }
    }
}

impl<T, const N: usize> Table for Generic<T, N>
where
    T: Item,
{
    fn as_widget(&self, is_active: bool) -> RadioButtonsTable<'_> {
        RadioButtonsTable {
            name: self.name.as_str().into(),
            content: RadioButtonsContent::new(
                self.items
                    .to_array()
                    .into_iter()
                    .map(|item| item.to_string()),
            )
            .with_hovered(self.items.hovered_index())
            .with_selected(self.items.selected_index()),
            is_active,
        }
    }
}
