use crate::tui::InteractibleComponent;

pub mod character;
pub mod unlockables;

pub trait TabComponent: InteractibleComponent {
    fn name(&self) -> &'static str;
}
