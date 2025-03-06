use player_progress::{Character, PlayerProgress, Run};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Style, Stylize},
    text::Text,
    widgets::{Cell, Row, Table, Widget},
};
use tokio::sync::watch;

use crate::{
    style::{IndexedColor, WithColor},
    tui::VisualComponent,
};

pub struct CharacterStats {
    progress: watch::Receiver<PlayerProgress>,
}

struct ContentRow {
    character: Character,
    arcade_easy: Run,
    arcade_medium: Run,
    arcade_hard: Run,
    story: Option<Run>,
}

struct RawRow<'a>([Cell<'a>; const { ContentRow::COLUMN_HEADERS.len() }]);

#[derive(Debug, Clone, Copy, derive_more::From)]
struct CompletionStatus(Option<Run>);

impl CharacterStats {
    pub fn new(progress: watch::Receiver<PlayerProgress>) -> Self {
        Self { progress }
    }
}

impl ContentRow {
    const SEPARATOR: &'static str = "│";
    const SEPARATOR_CONSTRAINT: Constraint = Constraint::Length(1);

    const COLUMN_HEADERS: [&'static str; 5] = [
        "Character 1CC",
        "Arcade Easy",
        "Arcade Medium",
        "Arcade Hard",
        "Story (Any)",
    ];
    const COLUMN_CONSTRAINT: Constraint = Constraint::Fill(1);

    pub fn separator() -> Cell<'static> {
        Cell::from(Self::SEPARATOR)
    }

    pub fn title() -> Row<'static> {
        let cells = Self::COLUMN_HEADERS.map(|name| Cell::from(Text::raw(name).centered().bold()));
        RawRow(cells).into()
    }

    pub fn widths() -> impl Iterator<Item = Constraint> {
        [Self::COLUMN_CONSTRAINT; Self::COLUMN_HEADERS.len()]
            .into_iter()
            .flat_map(|constraint| [Self::SEPARATOR_CONSTRAINT, constraint])
            .skip(1)
    }
}

impl From<ContentRow> for RawRow<'_> {
    fn from(
        ContentRow {
            character,
            arcade_easy,
            arcade_medium,
            arcade_hard,
            story,
        }: ContentRow,
    ) -> Self {
        let cells = [
            Cell::from(Text::raw(character)),
            Cell::from(CompletionStatus::from(arcade_easy)),
            Cell::from(CompletionStatus::from(arcade_medium)),
            Cell::from(CompletionStatus::from(arcade_hard)),
            Cell::from(CompletionStatus::from(story)),
        ];
        Self(cells)
    }
}

impl From<ContentRow> for Row<'_> {
    fn from(value: ContentRow) -> Self {
        RawRow::from(value).into()
    }
}

impl<'a> From<RawRow<'a>> for Row<'a> {
    fn from(RawRow(cells): RawRow<'a>) -> Self {
        Row::new(
            cells
                .into_iter()
                .flat_map(|cell| [ContentRow::separator(), cell])
                .skip(1),
        )
    }
}

impl VisualComponent for CharacterStats {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let progress = self.progress.borrow();
        let arcade_easy = progress.arcade_easy_1ccs.to_array();
        let arcade_medium = progress.arcade_medium_1ccs.to_array();
        let arcade_hard = progress.arcade_hard_1ccs.to_array();
        let story = progress
            .story_1ccs
            .to_array()
            .into_iter()
            .map(Some)
            .chain(std::iter::once(None));

        let content = Character::members()
            .into_iter()
            .zip(arcade_easy)
            .zip(arcade_medium)
            .zip(arcade_hard)
            .zip(story)
            .map(
                |((((character, arcade_easy), arcade_medium), arcade_hard), story)| ContentRow {
                    character,
                    arcade_easy,
                    arcade_medium,
                    arcade_hard,
                    story,
                },
            )
            .map(Row::from);

        let rows = std::iter::once(ContentRow::title())
            .chain(content)
            .zip(
                [IndexedColor::DarkGray, IndexedColor::Gray]
                    .into_iter()
                    .cycle(),
            )
            .map(|(row, bg_color)| row.style(Style::new().with_bg(bg_color)));

        Table::new(rows, ContentRow::widths()).render(area, buf);
    }
}

impl From<Run> for CompletionStatus {
    fn from(value: Run) -> Self {
        Self(Some(value))
    }
}

impl From<CompletionStatus> for Cell<'static> {
    fn from(CompletionStatus(run): CompletionStatus) -> Self {
        let (text, color) = match run {
            Some(run) if run.is_completed() => ("Done", IndexedColor::DarkGreen),
            Some(_) => ("Not done", IndexedColor::BrightRed),
            None => ("Cannnot", IndexedColor::DarkYellow),
        };

        Cell::from(Text::from(text).centered()).style(Style::new().with_fg(color))
    }
}
