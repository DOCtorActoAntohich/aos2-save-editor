use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Settings {
    pub player_file_path: PathBuf,
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
        }
    }
}

impl Settings {
    pub fn from_env() -> anyhow::Result<Self> {
        let env_vars: EnvVars = envy::from_env()?;
        Ok(env_vars.into())
    }
}
