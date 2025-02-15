use crate::tui::InteractibleComponent;

pub mod character;
pub mod progress;
pub mod unlockables;

pub trait InteratibleTabComponent: InteractibleComponent {
    fn name(&self) -> &'static str;
}
