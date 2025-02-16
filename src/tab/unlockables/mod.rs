mod backgrounds;
mod music;
mod style;

use player_progress::PlayerProgress;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{Event, KeyCode},
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{List, Row, Table, Widget},
};
use tokio::sync::watch;

use crate::{
    keyboard::GetKeyCode,
    tui::{HandleEvent, VisualComponent},
    widget::{default_text::DefaultText, separator},
};

use super::InteratibleTabComponent;

trait CustomButton: HandleEvent + Send {
    fn as_line(&self) -> Line<'_>;
    fn name(&self) -> &'static str;
}

pub struct Tab {
    table: ButtonsTable,
}

struct HelpText;

struct ButtonsTable {
    buttons: Vec<Box<dyn CustomButton>>,
    current_button: usize,
}

impl Tab {
    pub fn new(progress_tx: watch::Sender<PlayerProgress>) -> Self {
        let buttons: Vec<Box<dyn CustomButton>> = vec![
            Box::new(backgrounds::Button::new(progress_tx.clone())),
            Box::new(music::Button::new(progress_tx)),
        ];
        Self {
            table: ButtonsTable {
                buttons,
                current_button: 0,
            },
        }
    }
}

impl HelpText {
    pub const CONSTRAINT: Constraint = Constraint::Length(5);
}

impl ButtonsTable {
    fn next_button(&mut self) {
        self.current_button = self
            .current_button
            .saturating_add(1)
            .clamp(0, self.buttons.len() - 1);
    }

    fn previous_button(&mut self) {
        self.current_button = self
            .current_button
            .saturating_sub(1)
            .clamp(0, self.buttons.len() - 1);
    }
}

impl HandleEvent for Tab {
    fn handle_event(&mut self, event: &Event) {
        match event.key_code() {
            Some(KeyCode::Up) => self.table.previous_button(),
            Some(KeyCode::Down) => self.table.next_button(),
            _ => self.table.handle_event(event),
        }
    }
}

impl HandleEvent for ButtonsTable {
    fn handle_event(&mut self, event: &Event) {
        self.buttons[self.current_button].handle_event(event);
    }
}

impl VisualComponent for Tab {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let constraints = [
            HelpText::CONSTRAINT,
            separator::Horizontal::CONSTRAINT,
            Constraint::Fill(1),
        ];
        let [text_area, separator_area, table_area] =
            Layout::vertical(constraints).areas::<3>(area);

        HelpText.render(text_area, buf);

        separator::Horizontal::default().render(separator_area, buf);

        self.table.render(table_area, buf);
    }
}

impl VisualComponent for ButtonsTable {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let [area] = Layout::horizontal([Constraint::Ratio(1, 2)])
            .flex(Flex::Center)
            .areas::<1>(area);

        let rows = self.buttons.iter().enumerate().map(|(index, button)| {
            let row = Row::new(vec![button.name().into(), button.as_line().centered()]);
            if index == self.current_button {
                row.style(Style::new().bg(Color::White).fg(Color::Black))
            } else {
                row.style(Style::new().bg(Color::Black).fg(Color::White))
            }
        });

        let widths = [Constraint::Fill(1), Constraint::Fill(1)];
        Table::new(rows, widths).render(area, buf);
    }
}

impl InteratibleTabComponent for Tab {
    fn name(&self) -> &'static str {
        "Unlockables"
    }
}

impl Widget for HelpText {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let lines = [
            DefaultText::new(
                "Every Music track and Arena Background are within your finger's reach",
            ),
            DefaultText::new(""),
            DefaultText::new("(Obviously, this menu can't unlock DLC music)"),
            DefaultText::new(""),
            DefaultText::new("You can also unlock a SUPER SECRET background here :D"),
        ]
        .into_iter()
        .map(|line| Line::from(line).centered());

        List::new(lines).render(area, buf);
    }
}
