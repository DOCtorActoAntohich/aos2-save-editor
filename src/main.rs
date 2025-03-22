use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;

    let app_result = aos2_save_editor::App::new().run(&mut terminal);

    ratatui::restore();
    app_result.context("Critical error in app")
}
