use ratatui::{
    crossterm::event::{Event, KeyCode},
    layout::Constraint,
    widgets::{Block, Row, Table, Widget},
};
use savefile::file::game::{characters::full::FullCharacterValueGrid, PlayerProgress};
use tokio::sync::watch;

use crate::{
    keyboard::GetKeyCode,
    tui::{HandleEvent, VisualComponent},
};

use super::TabComponent;

#[derive(Debug)]
pub struct CharacterTab {
    progress: watch::Sender<PlayerProgress>,
    grid: FullCharacterValueGrid<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum TabItemEvent {
    #[default]
    None,
    Previous,
    Next,
    Toggle,
}

impl CharacterTab {
    pub fn new(progress: watch::Sender<PlayerProgress>) -> Self {
        let grid = progress.borrow().enabled_character.into();
        Self { progress, grid }
    }
}

impl HandleEvent for CharacterTab {
    type Error = anyhow::Error;

    fn handle_event(&mut self, event: Event) -> Result<Event, Self::Error> {
        match TabItemEvent::from(&event) {
            TabItemEvent::None => (),
            TabItemEvent::Previous => self.grid.switch_previous(),
            TabItemEvent::Next => self.grid.switch_next(),
            TabItemEvent::Toggle => {
                self.grid.toggle_current();
                self.progress
                    .send_modify(|progress| progress.enabled_character = self.grid.clone().into());
            }
        }

        Ok(event)
    }
}

impl From<&Event> for TabItemEvent {
    fn from(event: &Event) -> Self {
        event
            .key_code()
            .map(|code| match code {
                KeyCode::Up => Self::Previous,
                KeyCode::Down => Self::Next,
                KeyCode::Enter => Self::Toggle,
                _ => Self::default(),
            })
            .unwrap_or_default()
    }
}

impl VisualComponent for CharacterTab {
    fn render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let rows: Vec<_> = self
            .grid
            .characters()
            .map(|(name, &is_enabled)| {
                Row::new(vec![
                    name.to_string(),
                    if is_enabled {
                        "[+]".to_owned()
                    } else {
                        "[ ]".to_owned()
                    },
                ])
            })
            .collect();
        let widths = (0..rows.len()).into_iter().map(|_| Constraint::Max(32));
        Table::new(rows, widths)
            .block(Block::bordered())
            .render(area, buf);
    }
}

impl TabComponent for CharacterTab {
    fn name(&self) -> &'static str {
        "Characters"
    }
}

#[cfg(test)]
mod tests {
    use ratatui::crossterm::event::{
        Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers,
    };

    use crate::component::tab::character::TabItemEvent;

    #[rstest::rstest]
    #[case(
        Event::Key(KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Release,
            state: KeyEventState::empty(),
        }),
        TabItemEvent::None
    )]
    #[case(
        Event::Key(KeyEvent {
            code: KeyCode::PageUp,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        }),
        TabItemEvent::None
    )]
    #[case(
        Event::Key(KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        }),
        TabItemEvent::Toggle
    )]
    #[case(
        Event::Key(KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        }),
        TabItemEvent::Previous
    )]
    #[case(
        Event::Key(KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        }),
        TabItemEvent::Next
    )]
    fn proper_tab_event(#[case] event: Event, #[case] expected: TabItemEvent) {
        let actual = TabItemEvent::from(&event);

        assert_eq!(expected, actual);
    }
}
