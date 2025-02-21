use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};

pub trait GetKeyCode {
    fn key_code(&self) -> Option<KeyCode>;
}

impl GetKeyCode for Event {
    fn key_code(&self) -> Option<KeyCode> {
        match self {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => Some(key_event.code),
            Event::Key(_)
            | Event::FocusGained
            | Event::FocusLost
            | Event::Mouse(_)
            | Event::Paste(_)
            | Event::Resize(_, _) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use ratatui::crossterm::event::{
        Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers,
    };

    use super::GetKeyCode;

    #[rstest::rstest]
    #[case::none_release(
        Event::Key(KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Release,
            state: KeyEventState::empty(),
        }),
        None
    )]
    #[case::none_repeat(
        Event::Key(KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Repeat,
            state: KeyEventState::empty(),
        }),
        None
    )]
    #[case::ok_enter(
        Event::Key(KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        }),
        Some(KeyCode::Enter)
    )]
    #[case::ok_enter_all_modifiers(
        Event::Key(KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::all(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        }),
        Some(KeyCode::Enter)
    )]
    #[case::ok_enter_all_states(
        Event::Key(KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::all(),
        }),
        Some(KeyCode::Enter)
    )]
    #[case::ok_up_all_modifiers_and_states(
        Event::Key(KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::all(),
            kind: KeyEventKind::Press,
            state: KeyEventState::all(),
        }),
        Some(KeyCode::Up)
    )]
    fn event_key_code_is_correct(#[case] event: Event, #[case] expected: Option<KeyCode>) {
        let actual = event.key_code();

        assert_eq!(expected, actual);
    }
}
