use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    /// Path to saves folder (ends with `Documents/Fruitbat Factory/AoS2`).
    saves_folder: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let Args { saves_folder } = Args::parse();

    let mut terminal = ratatui::init();
    terminal.clear()?;

    let app = match saves_folder {
        Some(path) => aos2_save_editor::App::from_path(path),
        None => aos2_save_editor::App::from_env(),
    };
    let app_result = app.run(&mut terminal);

    ratatui::restore();
    app_result.context("Critical error in app")
}
