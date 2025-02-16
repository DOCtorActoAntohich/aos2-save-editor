use crate::tui::InteractibleComponent;

pub mod progress;
pub mod statistics;

pub trait InteratibleTabComponent: InteractibleComponent {
    fn name(&self) -> &'static str;
}
