use aos2_env::AoS2Paths;
use aos2_save_editor::EditorApp;
use savefile::file::game::PlayerProgress;

fn main() -> anyhow::Result<()> {
    let paths = AoS2Paths::from_env()?;
    let progress = PlayerProgress::from_file(&paths.game_sys)?;
    let app = EditorApp::new(paths, progress);
    run_tui(app)
}

fn run_tui(app: EditorApp) -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = app.run(terminal);
    ratatui::restore();
    app_result
}
