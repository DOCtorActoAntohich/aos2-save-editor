use ratatui::{
    buffer::Buffer,
    crossterm::event::{Event, KeyCode},
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Cell, List, Row, Table, Widget},
};
use savefile::file::game::{
    characters::{full::FullCharacterSheet, Character},
    PlayerProgress,
};
use tokio::sync::watch;

use crate::{
    keyboard::GetKeyCode,
    tui::{HandleEvent, VisualComponent},
    widget::{
        black_box::BlackBox, default_text::DefaultText, horizontal_separator::HorizontalSeparator,
        status_toggle::StatusToggle,
    },
};

use super::TabComponent;

#[derive(Debug)]
pub struct CharacterTab {
    progress: watch::Sender<PlayerProgress>,
    selected_character: usize,
}

struct HelpText;

struct CharacterTabWidget<I: Iterator<Item = (Character, bool)>> {
    table: CharacterTable<I>,
}

struct CharacterTable<I: Iterator<Item = (Character, bool)>> {
    rows: I,
    selected_character: usize,
}

impl CharacterTab {
    pub fn new(progress: watch::Sender<PlayerProgress>) -> Self {
        Self {
            progress,
            selected_character: 0,
        }
    }

    pub fn next_character(&mut self) {
        self.selected_character = self
            .selected_character
            .saturating_add(1)
            .clamp(0, FullCharacterSheet::N_CHARACTERS - 1);
    }

    pub fn previous_character(&mut self) {
        self.selected_character = self
            .selected_character
            .saturating_sub(1)
            .clamp(0, FullCharacterSheet::N_CHARACTERS - 1);
    }

    pub fn toggle_current_character(&mut self) {
        self.progress.send_modify(|progress| {
            let mut characters = progress.enabled_character.as_array();
            characters[self.selected_character] = !characters[self.selected_character];
            progress.enabled_character = characters.into();
        });
    }

    fn as_widget(&self) -> CharacterTabWidget<impl Iterator<Item = (Character, bool)>> {
        CharacterTabWidget {
            table: CharacterTable {
                rows: self.progress.borrow().enabled_character.iter(),
                selected_character: self.selected_character,
            },
        }
    }
}

impl HelpText {
    pub const CONSTRAINT: Constraint = Constraint::Length(3);
}

impl HandleEvent for CharacterTab {
    type Error = anyhow::Error;

    fn handle_event(&mut self, event: &Event) -> Result<(), Self::Error> {
        match event.key_code() {
            Some(KeyCode::Up) => self.previous_character(),
            Some(KeyCode::Down) => self.next_character(),
            Some(KeyCode::Enter) => self.toggle_current_character(),
            _ => (),
        }

        Ok(())
    }
}

impl VisualComponent for CharacterTab {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let borders = BlackBox::default();
        let inner_area = borders.inner(area);

        borders.render(area, buf);

        self.as_widget().render(inner_area, buf);
    }
}

impl TabComponent for CharacterTab {
    fn name(&self) -> &'static str {
        "Characters"
    }
}

impl<I> Widget for CharacterTabWidget<I>
where
    I: Iterator<Item = (Character, bool)>,
{
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let constraints = [
            HelpText::CONSTRAINT,
            HorizontalSeparator::CONSTRAINT,
            Constraint::Fill(1),
        ];
        let [text_area, separator_area, table_area] =
            Layout::vertical(constraints).areas::<3>(area);

        HelpText.render(text_area, buf);

        HorizontalSeparator::default().render(separator_area, buf);

        self.table.render(table_area, buf);
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

impl<I> Widget for CharacterTable<I>
where
    I: Iterator<Item = (Character, bool)>,
{
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let [area] = Layout::horizontal([Constraint::Ratio(1, 3)])
            .flex(Flex::Center)
            .areas::<1>(area);

        let CharacterTable {
            rows,
            selected_character,
        } = self;

        let rows = rows
            .enumerate()
            .map(|(row_index, (character, is_enabled))| {
                let character_name = Cell::new(character.to_string());
                let status_cell = StatusToggle::from(is_enabled).into_cell();
                let row = Row::new(vec![character_name, status_cell]);

                let is_selected = row_index == selected_character;
                if is_selected {
                    row.style(Style::new().bg(Color::White).fg(Color::Black))
                } else {
                    row.style(Style::new().bg(Color::Black).fg(Color::White))
                }
            });

        let widths = [Constraint::Min(12), Constraint::Min(3)];
        Table::new(rows, widths)
            .style(Style::new().bg(Color::Black).fg(Color::White))
            .render(area, buf);
    }
}
