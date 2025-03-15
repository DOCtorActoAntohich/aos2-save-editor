use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{List, Widget},
};

use crate::{
    editor::info::content_window::InteratibleTabComponent,
    savefile::Savefile,
    style::{IndexedColor, WithColor},
    tui::{Event, HandleEvent, VisualComponent},
    widget::split,
};

use super::table::{self, Table};

pub struct Tab {
    tables: table::Collection<3>,
}

struct InfoText;

impl InfoText {
    const N_LINES: u16 = 6;
    const CONSTRAINT: Constraint = Constraint::Length(Self::N_LINES);
}

impl Tab {
    #[must_use]
    pub fn new(savefile: &Savefile) -> Self {
        let title_color = savefile.profile().modify_title_color();
        let character = savefile.profile().modify_title_character();
        let title_text = savefile.profile().modify_title_text();

        let tables: [Box<dyn Table>; 3] = [
            Box::new(table::Generic::new("Color", title_color)),
            Box::new(table::Generic::new("Background Character", character)),
            Box::new(table::Generic::new("Title Text", title_text)),
        ];

        Self {
            tables: table::Collection::new(tables),
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
            constraint: Constraint::Fill(1),
            render: |area: Rect, buf: &mut Buffer| {
                self.tables.render(area, buf);
            },
        };
        split::Horizontal { top, bottom }.render(area, buf);
    }
}

impl InteratibleTabComponent for Tab {
    fn name(&self) -> &'static str {
        "Online Title"
    }
}

impl VisualComponent for InfoText {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let lines: [Line<'_>; Self::N_LINES as usize] = [
            Line::from("Choose any multiplayer title - free of charge")
                .style(Style::new().with_bg(Color::Black).with_fg(Color::White))
                .centered(),
            Line::from("").centered(),
            Line::from("Start typing with your keyboard for easy search (lists are long)")
                .style(
                    Style::new()
                        .with_bg(Color::Black)
                        .with_fg(IndexedColor::DarkYellow),
                )
                .centered(),
            Line::from("").centered(),
            Line::from("\"Background character\" changes character eyes in the title background")
                .centered(),
            Line::from("For some reason, this setting can turn Titles On/Off...").centered(),
        ];
        List::new(lines).render(area, buf);
    }
}
