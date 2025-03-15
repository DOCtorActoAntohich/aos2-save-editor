use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{List, Widget},
};

use crate::{
    info::content_window::InteratibleTabComponent,
    savefile::Savefile,
    style::{IndexedColor, WithColor},
    tui::{Event, HandleEvent, VisualComponent},
    widget::split,
};

use super::table::{self, Table};

pub struct Tab {
    tables: table::Collection<2>,
}

struct InfoText;

impl InfoText {
    const N_LINES: u16 = 3;
    const CONSTRAINT: Constraint = Constraint::Length(Self::N_LINES);
}

impl Tab {
    pub fn new(savefile: &Savefile) -> Self {
        let character = savefile.profile().write_avatar_character();
        let background = savefile.profile().write_avatar_background();

        let tables: [Box<dyn Table>; 2] = [
            Box::new(table::Generic::new("Character", character)),
            Box::new(table::Generic::new("Background", background)),
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
        "Online Avatar"
    }
}

impl VisualComponent for InfoText {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let lines: [Line<'_>; Self::N_LINES as usize] = [
            Line::from("Character and Background on your profile")
                .style(Style::new().with_bg(Color::Black).with_fg(Color::White))
                .centered(),
            Line::from("Nothing very interesting here, if you ask me...").centered(),
            Line::from("Check out Titles instead")
                .style(
                    Style::new()
                        .with_bg(Color::Black)
                        .with_fg(IndexedColor::DarkYellow),
                )
                .centered(),
        ];
        List::new(lines).render(area, buf);
    }
}
