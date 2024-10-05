use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Settings {
    pub saves_folder: PathBuf,
    pub player_file_path: PathBuf,
    pub game_sys_path: PathBuf,
}

#[derive(Debug, Deserialize)]
struct EnvVars {
    home: PathBuf,
}

impl From<EnvVars> for Settings {
    fn from(value: EnvVars) -> Self {
        let saves_folder = value
            .home
            .join("Documents")
            .join("Fruitbat Factory")
            .join("AoS2");

        Self {
            player_file_path: saves_folder.join("player.rkg"),
            game_sys_path: saves_folder.join("game.sys"),
            saves_folder,
        }
    }
}

impl Settings {
    pub fn from_env() -> anyhow::Result<Self> {
        let env_vars: EnvVars = envy::from_env()?;
        Ok(env_vars.into())
    }
}
