use aos2_env::AoS2Env;
use savefile::file::local::{
    background::{image::BackgroundImageSheet, music::BackgroundMusicSheet},
    characters::{full::CharacterSheet, story::CharacterStory1cc},
    PlayerProgress,
};

fn main() -> anyhow::Result<()> {
    let aos2_env = AoS2Env::from_env()?;

    println!(
        "Opening `game.sys` at: {}\n",
        aos2_env.saves_folder.display()
    );

    let mut progress = PlayerProgress::load(&aos2_env)?;

    progress.enabled_character = CharacterSheet::FULLY_UNLOCKED;
    progress.enabled_background_image = BackgroundImageSheet::FULLY_UNLOCKED;
    progress.enabled_background_music = BackgroundMusicSheet::FULLY_UNLOCKED;

    progress.arcade_easy_1cc_by_character = CharacterSheet::FULLY_UNLOCKED;
    progress.arcade_medium_1cc_by_character = CharacterSheet::FULLY_UNLOCKED;
    progress.arcade_hard_1cc_by_character = CharacterSheet::FULLY_UNLOCKED;
    progress.story_1cc_by_character = CharacterStory1cc::FULL;

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
