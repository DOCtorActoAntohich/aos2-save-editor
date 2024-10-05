use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct AoS2Paths {
    pub saves_folder: PathBuf,
    pub player_rkg: PathBuf,
    pub game_sys: PathBuf,
}

#[derive(Debug, Deserialize)]
struct EnvVars {
    home: PathBuf,
}

impl From<EnvVars> for AoS2Paths {
    fn from(value: EnvVars) -> Self {
        let saves_folder = value
            .home
            .join("Documents")
            .join("Fruitbat Factory")
            .join("AoS2");

        Self {
            player_rkg: saves_folder.join("player.rkg"),
            game_sys: saves_folder.join("game.sys"),
            saves_folder,
        }
    }
}

impl AoS2Paths {
    pub fn from_env() -> anyhow::Result<Self> {
        let env_vars: EnvVars = envy::from_env()?;
        Ok(env_vars.into())
    }
}
