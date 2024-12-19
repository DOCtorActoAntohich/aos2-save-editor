use ratatui::{
    buffer::Buffer,
    crossterm::event::{Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Paragraph, Widget},
};

use crate::tui::{HandleEvent, InteractibleComponent, VisualComponent};

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

impl<C> FullHelpToggle<C>
where
    C: InteractibleComponent,
{
    pub const CONSTRAINT: Constraint = Constraint::Length(1);

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
    type Error = anyhow::Error;

    fn handle_event(&mut self, event: Event) -> Result<Event, Self::Error> {
        match event {
            Event::Key(key_event)
                if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::F(1) =>
            {
                self.mode = self.mode.toggle();
                Ok(event)
            }
            other => {
                if self.mode == Mode::ShowContent {
                    self.content.handle_event(other)
                } else {
                    Ok(other)
                }
            }
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
        Paragraph::new("TODO: write this help lol")
            .centered()
            .render(area, buf);
    }
}

impl VisualComponent for Footer {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        const DARK_GRAY: Color = Color::Indexed(236);

        Paragraph::new("Press F1 to toggle help")
            .style(Style::default().bg(DARK_GRAY).fg(Color::White))
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
