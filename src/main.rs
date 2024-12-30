use anyhow::Context;
use aos2_env::AoS2Paths;
use aos2_save_editor::{signal::termination_signal, EditorApp};
use savefile::file::game::PlayerProgress;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let paths = AoS2Paths::from_env()?;
    let progress = PlayerProgress::from_file(&paths.game_sys)?;
    let app = EditorApp::new(paths, progress);

    let terminal = ratatui::init();

    // Needs to be a background task because otherwise
    // it cannot catch `CTRL_CLOSE` signal.
    // Why in the world didn't it work? I don't even know man.
    let tui_task = tokio::spawn(app.run(terminal));

    let result = tokio::select! {
        signal = termination_signal() => {
            signal.map(drop)
        }
        result = tui_task => {
            result.context("Failed to await background task")?
        }
    };

    ratatui::restore();

    result
}
