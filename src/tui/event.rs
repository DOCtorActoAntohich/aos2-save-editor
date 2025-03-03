use std::time::{Duration, Instant};

use ratatui::crossterm::event::{KeyCode, KeyEventKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    key_code: Option<KeyCode>,
    ascii_input: AsciiInputBuffer,
    received_at: Instant,
}

#[derive(Debug, Clone, PartialEq, Eq, derive_more::AsRef)]
#[as_ref(forward)]
struct AsciiInputBuffer(String);

impl Event {
    pub const MAX_TEXT_RETAIN_DELTA: Duration = Duration::from_millis(250);

    pub fn empty() -> Self {
        let long_ago = Instant::now() - 2 * Self::MAX_TEXT_RETAIN_DELTA;
        Self {
            key_code: None,
            ascii_input: AsciiInputBuffer::empty(),
            received_at: long_ago,
        }
    }

    pub fn follow_with(self, event: ratatui::crossterm::event::Event, now: Instant) -> Self {
        let Self {
            key_code: _,
            mut ascii_input,
            received_at,
        } = self;

        if now.duration_since(received_at) > Self::MAX_TEXT_RETAIN_DELTA {
            ascii_input.clear();
        }

        let key_code = match event.key_code() {
            code @ Some(KeyCode::Char(c)) => {
                ascii_input.try_push(c);
                code
            }
            code => {
                ascii_input.clear();
                code
            }
        };

        Self {
            key_code,
            ascii_input,
            received_at: now,
        }
    }

    pub fn key_code(&self) -> Option<KeyCode> {
        self.key_code
    }

    pub fn accumulated_input(&self) -> &str {
        self.ascii_input.as_ref()
    }
}

trait GetKeyCode {
    fn key_code(&self) -> Option<KeyCode>;
}

impl GetKeyCode for ratatui::crossterm::event::Event {
    fn key_code(&self) -> Option<KeyCode> {
        match self {
            Self::Key(key_event) if key_event.kind == KeyEventKind::Press => Some(key_event.code),
            Self::Key(_)
            | Self::FocusGained
            | Self::FocusLost
            | Self::Mouse(_)
            | Self::Paste(_)
            | Self::Resize(_, _) => None,
        }
    }
}

impl AsciiInputBuffer {
    pub const MAX_SIZE: usize = 32;

    pub fn empty() -> Self {
        Self(String::with_capacity(Self::MAX_SIZE))
    }

    pub fn try_push(&mut self, character: char) -> bool {
        let Self(buffer) = self;

        let should_push = character.is_ascii() && buffer.len() < Self::MAX_SIZE;
        if should_push {
            buffer.push(character);
        }
        should_push
    }

    pub fn clear(&mut self) {
        let Self(buffer) = self;
        buffer.clear();
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use ratatui::crossterm::event::Event as RatatuiEvent;
    use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

    use super::{AsciiInputBuffer, Event};

    fn event_from_key(code: KeyCode) -> RatatuiEvent {
        RatatuiEvent::Key(KeyEvent {
            code,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        })
    }

    #[rstest::rstest]
    #[case::short("123456789aBcDeF", "123456789aBcDeF")]
    #[case::too_long(
        &std::iter::repeat_n("1234567890", 4).collect::<String>(),
        "12345678901234567890123456789012"
    )]
    fn ascii_input_buffer_outputs(
        #[case] input: &str,
        #[case] expected: &str,
        #[values(Duration::from_millis(200))] step: Duration,
    ) {
        assert!(step <= Event::MAX_TEXT_RETAIN_DELTA, "Test Precondition");

        let start_time = Instant::now();
        let instants = (0..input.len())
            .into_iter()
            .map(|i| start_time + i as u32 * step);

        let mut current = Event::empty();
        for (instant, character) in instants.zip(input.chars()) {
            let ratatui_event = event_from_key(KeyCode::Char(character));
            current = current.follow_with(ratatui_event, instant);
        }

        assert_eq!(expected, current.accumulated_input());
    }

    #[rstest::rstest]
    #[case::too_old(KeyCode::Char('s'), Duration::from_secs(5), "s")]
    #[case::not_a_char(KeyCode::Enter, Duration::from_millis(10), "")]
    fn ascii_input_buffer_resets(
        #[case] code: KeyCode,
        #[case] age: Duration,
        #[case] expected: &str,
    ) {
        let now = Instant::now();
        let received_at = now - age;

        let event = Event {
            key_code: None,
            ascii_input: AsciiInputBuffer("any contents".to_owned()),
            received_at,
        };

        let event = event.follow_with(event_from_key(code), now);

        assert_eq!(expected, event.accumulated_input())
    }
}
