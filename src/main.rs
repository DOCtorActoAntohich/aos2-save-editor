mod binary_file;

mod settings;

use self::binary_file::{player::PlayerFile, savefile::GameSaveFile};
use binary_file::game::GameSysFile;
use settings::Settings;

fn main() -> anyhow::Result<()> {
    let settings = Settings::from_env()?;

    let player_file = PlayerFile::from_file(&settings.player_file_path)?;
    player_file.save(&settings.player_file_path)?;

    let mut game_sys = GameSysFile::from_file(&settings.game_sys_path)?;

    let suguri_story_something_location = 0x4c - 8;
    println!("len is {}", game_sys.content.bytes.len());
    println!(
        "byte at 75 / 0x4c is: {:#x}",
        game_sys.content.bytes[suguri_story_something_location]
    );

    // game_sys.content.bytes[suguri_story_something_location] = 0x6f;
    game_sys.save(&settings.game_sys_path)?;

    println!("Saved file");

    Ok(())
}
