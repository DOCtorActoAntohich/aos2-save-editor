use aos2_env::AoS2Env;
use player_progress::{
    Arenas, MusicTracks, PerfectArcadeMode, PerfectStoryMode, PlayableCharacters, PlayerProgress,
};

fn main() -> anyhow::Result<()> {
    let aos2_env = AoS2Env::from_env()?;

    println!(
        "Opening `game.sys` at: {}\n",
        aos2_env.saves_folder.display()
    );

    let mut progress = PlayerProgress::load(&aos2_env)?;

    progress.playable_characters = PlayableCharacters::ALL;
    progress.arenas = Arenas::ALL;
    progress.background_music = MusicTracks::ALL;

    progress.arcade_easy_1ccs = PerfectArcadeMode::COMPLETED;
    progress.arcade_medium_1ccs = PerfectArcadeMode::COMPLETED;
    progress.arcade_hard_1ccs = PerfectArcadeMode::COMPLETED;
    progress.story_1ccs = PerfectStoryMode::COMPLETED;

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
