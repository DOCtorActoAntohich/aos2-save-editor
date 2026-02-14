use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;

    let error = aos2_save_editor::savefile::Error::Progress(binary_file::Error {
        path: aos2_env::saves_location(aos2_env::EXAMPLE_HOME)
            .join(player_progress::PlayerProgress::FILE_NAME),
        action: binary_file::ErroneousAction::Reading,
        detail: binary_file::ErrorDetail::NotFound,
    });
    let app_result = aos2_save_editor::App::new_limbo(error).run(&mut terminal);

    ratatui::restore();
    app_result.context("Critical error")
}
