use ratatui::{
    buffer::Buffer,
    crossterm::event::KeyCode,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{List, Paragraph, Widget},
};

use crate::{
    style::{IndexedColor, WithColor},
    tui::{Event, HandleEvent, InteractibleComponent, VisualComponent},
    widget::{content_box::ContentBox, split},
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

#[derive(Debug, derive_more::Into)]
struct HelpStyle(Style);

impl Mode {
    pub fn toggle(self) -> Self {
        match self {
            Mode::ShowHelp => Mode::ShowContent,
            Mode::ShowContent => Mode::ShowHelp,
        }
    }
}

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
            Mode::ShowHelp => draw_help_window(content_area, buf),
            Mode::ShowContent => self.content.render(content_area, buf),
        }

        draw_footer(footer_area, buf);
    }
}

fn draw_help_window(area: Rect, buf: &mut Buffer) {
    let left = split::Area {
        constraint: Constraint::Fill(1),
        render: draw_controls,
    };

    let right = split::Area {
        constraint: Constraint::Fill(1),
        render: draw_extra_info,
    };

    ContentBox::gray()
        .with_title("[HELP]")
        .with_content(move |area: Rect, buf: &mut Buffer| {
            split::Vertical { left, right }.render(area, buf);
        })
        .render(area, buf);
}

fn draw_controls(area: Rect, buf: &mut Buffer) {
    fn line<'a>(controls: &'a str, description: &'a str) -> Line<'a> {
        Line::from(vec![
            Span::raw(">> "),
            Span::raw(controls).style(Style::new().with_fg(IndexedColor::DarkYellow)),
            Span::raw(" - "),
            Span::raw(description),
        ])
    }

    let lines = vec![
        Line::from("General controls:"),
        Line::from(""),
        line("Arrow Keys", "Navigate tables"),
        line("Enter", "Interact with selected item"),
        line("PgUp / PgDown", "Switch tabs"),
        line("Home / End", "Go to start/end of the list"),
        line("Escape", "Exit"),
    ];
    Paragraph::new(lines).render(area, buf);
}

fn draw_extra_info(area: Rect, buf: &mut Buffer) {
    let lines = [
        Line::from(vec![
            Span::raw("All "),
            Span::raw("changes are saved automatically")
                .style(Style::new().with_fg(IndexedColor::DarkYellow)),
            Span::raw(" when you make them"),
        ]),
        Line::from(""),
        Line::from("Close the game before editing"),
        Line::from("Otherwise, it will ignore your changes"),
        Line::from(""),
        Line::from("If any issues occur, report them on GitHub"),
    ];
    List::new(lines).render(area, buf);
}

fn draw_footer(area: Rect, buf: &mut Buffer) {
    Line::from(vec![
        Span::raw("Press `"),
        Span::raw(FullHelpToggle::<()>::KEY.to_string())
            .style(Style::new().with_fg(IndexedColor::DarkYellow)),
        Span::raw("` to toggle help"),
    ])
    .style(HelpStyle::default())
    .render(area, buf);
}
