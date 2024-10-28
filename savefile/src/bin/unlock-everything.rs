use aos2_env::AoS2Paths;
use savefile::file::game::{
    BackgroundImageSheet, BackgroundMusicSheet, FullCharacterSheet, PlayerProgress,
    StoryCharacterSheet,
};

fn main() -> anyhow::Result<()> {
    let paths = AoS2Paths::from_env()?;

    println!("Opening `game.sys` at: {}\n", paths.game_sys.display());

    let mut progress = PlayerProgress::from_file(&paths.game_sys)?;

    progress.enabled_character = FullCharacterSheet::FULLY_UNLOCKED;
    progress.enabled_background_image = BackgroundImageSheet::FULLY_UNLOCKED;
    progress.enabled_background_music = BackgroundMusicSheet::FULLY_UNLOCKED;

    progress.arcade_easy_1cc_by_character = FullCharacterSheet::FULLY_UNLOCKED;
    progress.arcade_medium_1cc_by_character = FullCharacterSheet::FULLY_UNLOCKED;
    progress.arcade_hard_1cc_by_character = FullCharacterSheet::FULLY_UNLOCKED;
    progress.story_1cc_by_character = StoryCharacterSheet::FULLY_UNLOCKED;

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

    progress.save_to_file(&paths.game_sys)?;

    println!("Saved `game.sys` to: {}", paths.game_sys.display());

    Ok(())
}
