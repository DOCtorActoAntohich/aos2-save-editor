use std::path::PathBuf;

use anyhow::Context;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct AoS2Env {
    pub saves_folder: PathBuf,
    pub player_rkg: PathBuf,
}

#[derive(Debug, Deserialize)]
struct EnvVars {
    home: PathBuf,
}

impl From<EnvVars> for AoS2Env {
    fn from(value: EnvVars) -> Self {
        let saves_folder = value
            .home
            .join("Documents")
            .join("Fruitbat Factory")
            .join("AoS2");

        Self {
            player_rkg: saves_folder.join("player.rkg"),
            saves_folder,
        }
    }
}

impl AoS2Env {
    pub fn from_env() -> anyhow::Result<Self> {
        let env_vars: EnvVars =
            envy::from_env().context("Failed to get AoS2 savefile directory")?;
        Ok(env_vars.into())
    }
}
