use crate::tui::InteractibleComponent;

pub mod character;
pub mod unlockables;

pub trait Tab: InteractibleComponent {
    fn name(&self) -> &'static str;
}
