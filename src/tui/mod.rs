mod event;

pub use self::event::Event;

use ratatui::{buffer::Buffer, layout::Rect};

pub trait HandleEvent {
    fn handle_event(&mut self, event: &Event);
}

/// Similar to [`ratatui::widgets::Widget`] but takes `self` by reference.
///
/// Since [`ratatui::widgets::Widget`] consumes `self`,
/// it prevents me from doing cool shit with traits.
/// In my opinion, that design makes no sense, because it steals flexibility.
/// The user should decide when to drop their owned types (yes i'm angy about it).
///
/// This trait is intended for use when consuming the type for rendering makes no sense.
/// Usually good for stateful and interactible components.
pub trait VisualComponent {
    fn render(&self, area: Rect, buf: &mut Buffer);
}

pub trait InteractibleComponent: HandleEvent + VisualComponent + Send {}

impl<C> InteractibleComponent for C where C: HandleEvent + VisualComponent + Send {}
