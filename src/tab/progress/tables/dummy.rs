use std::convert::Infallible;

use crate::{tab::progress::widget, tui::HandleEvent};

use super::InteractibleTable;

pub struct Dummy {
    name: String,
}

impl Dummy {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl HandleEvent for Dummy {
    type Error = Infallible;

    fn handle_event(&mut self, _: &ratatui::crossterm::event::Event) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl InteractibleTable for Dummy {
    fn name(&self) -> &str {
        &self.name
    }

    fn as_widget(&self) -> widget::Table {
        widget::Table::new(std::iter::once((self.name(), false)))
            .highlight_current(false)
            .with_current(0)
    }
}
