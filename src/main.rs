use aos2_save_editor::EditorApp;
use ratatui::DefaultTerminal;

fn main() -> anyhow::Result<()> {
    let app = EditorApp::new();
    run_tui(app)
}

fn run_tui(app: EditorApp) -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = app.run(terminal);
    ratatui::restore();
    app_result
}
