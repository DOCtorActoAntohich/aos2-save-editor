use aos2_env::AoS2Env;
use aos2_save_editor::{savefile::Savefile, EditorApp};
use online_profile::PlayerOnlineProfile;
use player_progress::PlayerProgress;

fn main() -> anyhow::Result<()> {
    let aos2_env = AoS2Env::from_env()?;
    let progress = PlayerProgress::load(&aos2_env)?.unwrap_or_default();

    let profile = PlayerOnlineProfile::load(&aos2_env)?.unwrap_or_default();
    let savefile = Savefile::load(aos2_env.clone())?;
    let app = EditorApp::new(aos2_env, progress, profile, savefile);
    run_tui(app)
}

fn run_tui(app: EditorApp) -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = app.run(terminal);
    ratatui::restore();
    app_result
}
