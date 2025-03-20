#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct AoS2Env {
    pub saves_folder: PathBuf,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to find AoS2 saves directory")]
    DirectoryPath,
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

        Self { saves_folder }
    }
}

impl AoS2Env {
    pub fn from_env() -> Result<Self, Error> {
        let env_vars: EnvVars = envy::from_env().map_err(|_| Error::DirectoryPath)?;
        Ok(env_vars.into())
    }
}
