use std::path::PathBuf;

use anyhow::Context;
use binary_file::ErroneousAction;
use clap::Parser;

#[derive(clap::Parser)]
struct Args {
    #[arg(long, default_value_t = ErroneousAction::Reading)]
    action: ErroneousAction,
    #[arg(long)]
    path: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let Args { action, path } = Args::parse();
    let path = path.unwrap_or_else(|| {
        aos2_env::saves_location(aos2_env::EXAMPLE_HOME)
            .join(player_progress::PlayerProgress::FILE_NAME)
    });

    let mut terminal = ratatui::init();
    terminal.clear()?;

    let error = aos2_save_editor::savefile::Error::Progress(binary_file::Error {
        path,
        action,
        detail: binary_file::ErrorDetail::NotFound,
    });
    let app_result = aos2_save_editor::App::new_limbo(error).run(&mut terminal);

    ratatui::restore();
    app_result.context("Critical error")
}
