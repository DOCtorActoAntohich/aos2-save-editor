mod binary_file;

mod settings;

use self::binary_file::player::{
    sized_section::SizedBinarySection, AvatarBackground, AvatarCharacter, PlayerFile,
    TitleCharacter, TitleColor, TitleText,
};
use settings::Settings;

fn main() -> anyhow::Result<()> {
    let settings = Settings::from_env()?;

    let mut file = PlayerFile::from_file(&settings.player_file_path)?;

    file.avatar_character = AvatarCharacter::Empty;
    file.avatar_background = AvatarBackground::LightGrayBackgroundWithSilhouette;
    file.title_character_in_background = TitleCharacter::OjHimeWinter;
    file.title_color = TitleColor::Blue;
    file.title_text_id = TitleText::LookingForFriends;
    file.nickname = SizedBinarySection {
        bytes: b"DOC".into(),
    };

    file.save(&settings.player_file_path)?;

    println!("Saved file");

    Ok(())
}
