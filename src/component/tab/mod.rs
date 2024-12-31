use crate::tui::InteractibleComponent;

pub mod character;
pub mod unlockables;

pub trait InteratibleTab: InteractibleComponent {
    fn name(&self) -> &'static str;
}
