use player_progress::{Character, PlayableCharacters, PlayerProgress, Status};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{Event, KeyCode},
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{List, Widget},
};
use tokio::sync::watch;

use crate::{
    collection::SelectibleArray,
    keyboard::GetKeyCode,
    tui::{HandleEvent, VisualComponent},
    widget::{default_text::DefaultText, separator, toggles_table::TogglesTable},
};

use super::InteratibleTabComponent;

#[derive(Debug)]
pub struct Tab {
    progress: watch::Sender<PlayerProgress>,
    characters: SelectibleArray<Status, { PlayableCharacters::AMOUNT }>,
}

struct HelpText;

struct CharacterTabWidget {
    table: TogglesTable<Character, Status>,
}

impl Tab {
    pub fn new(progress: watch::Sender<PlayerProgress>) -> Self {
        let characters = progress.borrow().playable_characters.to_array();
        Self {
            progress,
            characters: SelectibleArray::new(characters),
        }
    }

    pub fn toggle_current_character(&mut self) {
        self.characters.modify_current(|status| *status = !*status);
        self.progress.send_modify(|progress| {
            progress.playable_characters = self.characters.to_array().into();
        });
    }

    fn as_widget(&self) -> CharacterTabWidget {
        CharacterTabWidget {
            table: TogglesTable {
                items: self
                    .progress
                    .borrow()
                    .playable_characters
                    .iter()
                    .map(Into::into)
                    .collect(),
                current: self.characters.current_index(),
            },
        }
    }
}

impl HelpText {
    pub const CONSTRAINT: Constraint = Constraint::Length(3);
}

impl HandleEvent for Tab {
    fn handle_event(&mut self, event: &Event) {
        match event.key_code() {
            Some(KeyCode::Up) => self.characters.select_previous(),
            Some(KeyCode::Down) => self.characters.select_next(),
            Some(KeyCode::Enter) => self.toggle_current_character(),
            _ => (),
        }
    }
}

impl VisualComponent for Tab {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        self.as_widget().render(area, buf);
    }
}

impl InteratibleTabComponent for Tab {
    fn name(&self) -> &'static str {
        "Characters"
    }
}

impl Widget for CharacterTabWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let Self { table } = self;

        let constraints = [
            HelpText::CONSTRAINT,
            separator::Horizontal::CONSTRAINT,
            Constraint::Fill(1),
        ];
        let [text_area, separator_area, table_area] =
            Layout::vertical(constraints).areas::<3>(area);

        HelpText.render(text_area, buf);

        separator::Horizontal::default().render(separator_area, buf);

        let [table_area] = Layout::horizontal([Constraint::Ratio(1, 3)])
            .flex(Flex::Center)
            .areas::<1>(table_area);

        table.render(table_area, buf);
    }
}

impl Widget for HelpText {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let lines = [
            DefaultText::new("!! Keep at least 3-5 characters enabled !!").red(),
            DefaultText::new(""),
            DefaultText::new("Otherwise the game will crash regularly"),
        ]
        .into_iter()
        .map(|line| Line::from(line).centered());

        List::new(lines)
            .style(Style::new().bg(Color::Black))
            .render(area, buf);
    }
}
