use crate::tui::InteractibleComponent;

pub mod character;
pub mod empty;

pub trait TabComponent: InteractibleComponent {
    fn name(&self) -> &'static str;
}
