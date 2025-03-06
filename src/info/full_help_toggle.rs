use ratatui::{
    buffer::Buffer,
    crossterm::event::KeyCode,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Paragraph, Widget},
};

use crate::{
    style::{IndexedColor, WithColor},
    tui::{Event, HandleEvent, InteractibleComponent, VisualComponent},
    widget::content_box::ContentBox,
};

#[derive(Debug)]
pub struct FullHelpToggle<C> {
    content: C,
    mode: Mode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum Mode {
    ShowHelp,
    #[default]
    ShowContent,
}

#[derive(Debug)]
struct HelpTextWindow;

#[derive(Debug)]
struct Footer;

#[derive(Debug, derive_more::Into)]
struct HelpStyle(Style);

impl<C> FullHelpToggle<C> {
    pub const KEY: KeyCode = KeyCode::F(12);
}

impl Default for HelpStyle {
    fn default() -> Self {
        Self(
            Style::new()
                .with_bg(IndexedColor::DarkGray)
                .with_fg(Color::White),
        )
    }
}

impl<C> FullHelpToggle<C>
where
    C: InteractibleComponent,
{
    pub fn new(content: C) -> Self {
        Self {
            content,
            mode: Mode::default(),
        }
    }
}

impl<C> HandleEvent for FullHelpToggle<C>
where
    C: InteractibleComponent,
{
    fn handle_event(&mut self, event: &Event) {
        match event.key_code() {
            Some(Self::KEY) => {
                self.mode = self.mode.toggle();
            }
            _other if self.mode == Mode::ShowContent => self.content.handle_event(event),
            _ => (),
        }
    }
}

impl<C> VisualComponent for FullHelpToggle<C>
where
    C: InteractibleComponent,
{
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let constraints = [Constraint::Fill(1), Constraint::Length(1)];
        let [content_area, footer_area] = Layout::vertical(constraints).areas::<2>(area);

        match self.mode {
            Mode::ShowHelp => HelpTextWindow.render(content_area, buf),
            Mode::ShowContent => self.content.render(content_area, buf),
        }

        Footer.render(footer_area, buf);
    }
}

impl VisualComponent for HelpTextWindow {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        ContentBox::gray()
            .with_title("[HELP]")
            .with_content(|area: Rect, buf: &mut Buffer| {
                Paragraph::new("TODO: write this help lol")
                    .centered()
                    .style(HelpStyle::default())
                    .render(area, buf);
            })
            .render(area, buf);
    }
}

impl VisualComponent for Footer {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let text = format!("Press `{}` to toggle help", FullHelpToggle::<()>::KEY);

        Paragraph::new(text)
            .style(HelpStyle::default())
            .render(area, buf);
    }
}

impl Mode {
    pub fn toggle(self) -> Self {
        match self {
            Mode::ShowHelp => Mode::ShowContent,
            Mode::ShowContent => Mode::ShowHelp,
        }
    }
}
