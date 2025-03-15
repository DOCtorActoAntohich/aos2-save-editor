use aos2_env::AoS2Env;
use aos2_save_editor::{editor::EditorApp, savefile::Savefile};

fn main() -> anyhow::Result<()> {
    let aos2_env = AoS2Env::from_env()?;
    let savefile = Savefile::load(aos2_env)?;
    let app = EditorApp::new(savefile);
    run_tui(app)
}

fn run_tui(app: EditorApp) -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = app.run(terminal);
    ratatui::restore();
    app_result
}
