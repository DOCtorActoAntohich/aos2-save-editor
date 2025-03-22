use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;

    let error = aos2_save_editor::savefile::Error::Progress(player_progress::Error::NotFound);
    let app_result = aos2_save_editor::App::new_limbo(error).run(&mut terminal);

    ratatui::restore();
    app_result.context("Critical error")
}
