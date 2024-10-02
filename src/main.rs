mod binary_file;

mod settings;

use self::binary_file::player::PlayerFile;
use settings::Settings;

fn main() -> anyhow::Result<()> {
    let settings = Settings::from_env()?;

    let file = PlayerFile::from_file(&settings.player_file_path)?;
    file.save(&settings.player_file_path)?;

    println!("Saved file");

    Ok(())
}
