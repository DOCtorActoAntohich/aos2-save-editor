#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

pub mod editor;
pub mod savefile;

mod app;
mod collection;
mod limbo;
mod style;
mod tui;
mod widget;

pub use crate::app::App;
