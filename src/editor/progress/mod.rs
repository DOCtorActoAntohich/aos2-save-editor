mod tables;
mod widget;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{List, Widget},
};

use crate::{
    editor::content_window::InteratibleTabComponent,
    savefile::Savefile,
    style::{IndexedColor, WithColor},
    tui::{Event, HandleEvent, VisualComponent},
    widget::split,
};

use self::tables::TablesCollection;

pub struct Tab {
    tables: TablesCollection,
}

struct InfoText;

impl InfoText {
    pub const N_LINES: u16 = 6;
    pub const CONSTRAINT: Constraint = Constraint::Length(Self::N_LINES);
}

impl Tab {
    #[must_use]
    pub fn new(savefile: &Savefile) -> Self {
        Self {
            tables: TablesCollection::new(savefile),
        }
    }
}

impl HandleEvent for Tab {
    fn handle_event(&mut self, event: &Event) {
        self.tables.handle_event(event);
    }
}

impl VisualComponent for Tab {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let top = split::Area {
            constraint: InfoText::CONSTRAINT,
            render: |area: Rect, buf: &mut Buffer| {
                InfoText.render(area, buf);
            },
        };

        let bottom = split::Area {
            constraint: TablesCollection::CONSTRAINT,
            render: |area: Rect, buf: &mut Buffer| {
                self.tables.render(area, buf);
            },
        };

        split::Horizontal { top, bottom }.render(area, buf);
    }
}

impl InteratibleTabComponent for Tab {
    fn name(&self) -> &'static str {
        "Progress"
    }
}

impl Widget for InfoText {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let lines: [Text<'_>; InfoText::N_LINES as usize] = [
            Text::from("!! Keep at least 2-3 options enabled in each category !!")
                .centered()
                .style(
                    Style::new()
                        .with_bg(Color::Black)
                        .with_fg(IndexedColor::BrightRed),
                ),
            Text::from("Otherwise the game will just crash at character select regularly.")
                .centered(),
            Text::from("").centered(),
            Text::from("Yes, you CAN disable Iru and Sham :trol face:").centered(),
            Text::from("").centered(),
            Text::from("DLC music is not available - Steam controls it, not the savefile.")
                .centered()
                .style(
                    Style::new()
                        .with_bg(Color::Black)
                        .with_fg(IndexedColor::DarkYellow),
                ),
        ];

        List::new(lines).render(area, buf);
    }
}
