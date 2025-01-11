use aos2_env::AoS2Env;
use player_progress::{
    Arenas, Music, PerfectArcadeModeRuns, PerfectStoryModeRuns, PlayableCharacters, PlayerProgress,
};

fn main() -> anyhow::Result<()> {
    let aos2_env = AoS2Env::from_env()?;

    println!(
        "Opening `game.sys` at: {}\n",
        aos2_env.saves_folder.display()
    );

    let mut progress = PlayerProgress::load(&aos2_env)?;

    progress.enabled_character = PlayableCharacters::ALL;
    progress.arenas = Arenas::ALL;
    progress.background_music = Music::ALL;

    progress.arcade_easy_1ccs = PerfectArcadeModeRuns::COMPLETED;
    progress.arcade_medium_1ccs = PerfectArcadeModeRuns::COMPLETED;
    progress.arcade_hard_1ccs = PerfectArcadeModeRuns::COMPLETED;
    progress.story_1ccs = PerfectStoryModeRuns::COMPLETED;

    println!(
        r#"
Unlocked:
- All characters.
- All backgrounds.
- All music.
Fully completed (no deaths):
- All arcade modes (easy, medium, hard).
- All story modes
"#
    );

    progress.save(&aos2_env)?;

    println!("Saved `game.sys` at: {}", aos2_env.saves_folder.display());

    Ok(())
}
