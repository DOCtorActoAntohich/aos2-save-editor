mod tables;
mod widget;

use player_progress::PlayerProgress;
use ratatui::{
    layout::{Constraint, Layout},
    text::Text,
    widgets::{List, Widget},
};
use tokio::sync::watch;

use crate::{
    tui::{HandleEvent, VisualComponent},
    widget::separator,
};

use self::tables::TablesCollection;

use super::InteratibleTabComponent;

pub struct Tab {
    tables: TablesCollection,
}

impl Tab {
    pub fn new(progress: watch::Sender<PlayerProgress>) -> Self {
        Self {
            tables: TablesCollection::new(progress),
        }
    }
}

impl HandleEvent for Tab {
    type Error = anyhow::Error;

    fn handle_event(
        &mut self,
        event: &ratatui::crossterm::event::Event,
    ) -> Result<(), Self::Error> {
        match self.tables.handle_event(event) {
            Ok(()) => Ok(()),
            Err(infallible) => match infallible {},
        }
    }
}

impl VisualComponent for Tab {
    fn render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let constraints = [
            Constraint::Length(3),
            separator::Horizontal::CONSTRAINT,
            Constraint::Fill(1),
        ];
        let [text_area, separator_area, tables_area] =
            Layout::vertical(constraints).areas::<3>(area);

        List::new([
            Text::from("amogus").centered(),
            Text::from("imposter").centered(),
            Text::from("sus").centered(),
        ])
        .render(text_area, buf);

        separator::Horizontal::default().render(separator_area, buf);

        self.tables.render(tables_area, buf);
    }
}

impl InteratibleTabComponent for Tab {
    fn name(&self) -> &'static str {
        "Progress"
    }
}
