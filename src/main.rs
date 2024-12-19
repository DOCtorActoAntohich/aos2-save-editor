use aos2_save_editor::EditorApp;
use ratatui::DefaultTerminal;

fn main() -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = run(terminal);
    ratatui::restore();
    app_result
}

fn run(terminal: DefaultTerminal) -> anyhow::Result<()> {
    EditorApp::new().run(terminal)?;

    Ok(())
}
