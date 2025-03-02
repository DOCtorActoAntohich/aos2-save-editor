use online_profile::{title, PlayerOnlineProfile};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{Event, KeyCode},
    layout::{Constraint, Rect},
    widgets::{Cell, Row, Table, Widget},
};
use tokio::sync::watch;

use crate::{
    profile::collection::RadioButtonArray,
    style,
    tui::{event::GetKeyCode, HandleEvent, VisualComponent},
};

pub struct TitleColor {
    profile: watch::Sender<PlayerOnlineProfile>,
    colors: RadioButtonArray<title::Color, { title::Color::MEMBERS_COUNT }>,
}

impl TitleColor {
    pub fn new(profile: watch::Sender<PlayerOnlineProfile>) -> Self {
        let colors = title::Color::members();
        let current_color = profile.borrow().title_color;
        let selected = colors
            .iter()
            .enumerate()
            .find_map(|(index, &value)| {
                if value == current_color {
                    Some(index)
                } else {
                    None
                }
            })
            .unwrap_or_default();

        Self {
            profile,
            colors: RadioButtonArray::new(colors, selected),
        }
    }
}

impl HandleEvent for TitleColor {
    fn handle_event(&mut self, event: &Event) {
        match event.key_code() {
            Some(KeyCode::Up) => self.colors.hover_previous(),
            Some(KeyCode::Down) => self.colors.hover_next(),
            Some(KeyCode::Enter) => {
                self.colors.select_current();
                self.profile
                    .send_modify(|profile| profile.title_color = *self.colors.current());
            }
            _ => (),
        }
    }
}

impl VisualComponent for TitleColor {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let rows = self.colors.iter().enumerate().map(|(index, color)| {
            let is_selected = index == self.colors.selected_index();
            let is_hovered = index == self.colors.hovered_index();
            let selection = if is_selected { "[X]" } else { "[ ]" };
            let cells = [Cell::new(selection), Cell::new(color.to_string())];

            Row::new(cells).style(style::Selection::from_is_selected(is_hovered))
        });
        let widths = [Constraint::Fill(1), Constraint::Fill(1)];
        Table::new(rows, widths).render(area, buf);
    }
}
