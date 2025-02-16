use crate::tui::InteractibleComponent;

pub mod progress;

pub trait InteratibleTabComponent: InteractibleComponent {
    fn name(&self) -> &'static str;
}
