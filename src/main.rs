mod player;
mod settings;

use player::{AvatarBackground, AvatarCharacter, PlayerFile, TitleCharacter, TitleColor};
use settings::Settings;

fn main() -> anyhow::Result<()> {
    let settings = Settings::from_env()?;

    let mut file = PlayerFile::from_file(&settings.player_file_path)?;

    file.avatar_character = AvatarCharacter::Empty;
    file.avatar_background = AvatarBackground::LightGrayBackgroundWithSilhouette;
    file.title_character_in_background = TitleCharacter::None;
    file.title_color = TitleColor::Blue;

    file.save(&settings.player_file_path)?;

    println!("Saved file");

    Ok(())
}
