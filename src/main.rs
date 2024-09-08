mod player;
mod settings;

use player::{AvatarBackground, AvatarCharacter, PlayerFile, TitleCharacter};
use settings::Settings;

fn main() -> anyhow::Result<()> {
    let settings = Settings::from_env()?;

    let mut file = PlayerFile::from_file(&settings.player_file_path)?;
    let lobby = String::from_utf8(file.lobby_name.bytes.clone())?;
    println!("Lobby name: {}", lobby);

    file.avatar_character = AvatarCharacter::Empty;
    file.avatar_background = AvatarBackground::LightGrayBackgroundWithSilhouette;
    file.title_character_in_background = TitleCharacter::DisableTitle;

    file.save(&settings.player_file_path)?;

    println!("Saved file");

    Ok(())
}
