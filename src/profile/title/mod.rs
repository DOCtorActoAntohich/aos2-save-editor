use online_profile::{title, PlayerOnlineProfile};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{List, Widget},
};
use tokio::sync::watch;

use crate::{
    info::content_window::InteratibleTabComponent,
    style::{IndexedColor, WithColor},
    tui::{Event, HandleEvent, VisualComponent},
    widget::split,
};

use super::table::{generic, InteractibleTable, TablesCollection};

pub struct Tab {
    tables: TablesCollection<3>,
}

struct InfoText;

impl InfoText {
    const N_LINES: u16 = 3;
    const CONSTRAINT: Constraint = Constraint::Length(Self::N_LINES);
}

impl Tab {
    pub fn new(profile: watch::Sender<PlayerOnlineProfile>) -> Self {
        let color_params = generic::TableParams {
            profile: profile.clone(),
            items: title::Color::members(),
            current: profile.borrow().title_color,
            on_selected: |profile: &mut PlayerOnlineProfile, color: &title::Color| {
                profile.title_color = *color;
            },
            name: "Color".to_owned(),
        };
        let character_params = generic::TableParams {
            profile: profile.clone(),
            items: title::Character::members(),
            current: profile.borrow().title_character_in_background,
            on_selected: |profile: &mut PlayerOnlineProfile, character: &title::Character| {
                profile.title_character_in_background = *character;
            },
            name: "Background Character".to_owned(),
        };
        let text_params = generic::TableParams {
            profile: profile.clone(),
            items: title::Text::members(),
            current: profile.borrow().title_text_id,
            on_selected: |profile: &mut PlayerOnlineProfile, text: &title::Text| {
                profile.title_text_id = *text;
            },
            name: "Title Text".to_owned(),
        };

        let tables: [Box<dyn InteractibleTable>; 3] = [
            Box::new(generic::Table::new(color_params)),
            Box::new(generic::Table::new(character_params)),
            Box::new(generic::Table::new(text_params)),
        ];

        Self {
            tables: TablesCollection::new(tables),
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
        ];
        List::new(lines).render(area, buf);
    }
}
