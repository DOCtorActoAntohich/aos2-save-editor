mod player;
mod settings;

use player::PlayerFile;
use settings::Settings;

fn main() -> anyhow::Result<()> {
    let settings = Settings::from_env()?;

    let file = PlayerFile::from_file(settings.player_file_path)?;
    let lobby = String::from_utf8(file.lobby_name.bytes)?;
    println!("Lobby name: {}", lobby);

    Ok(())
}
