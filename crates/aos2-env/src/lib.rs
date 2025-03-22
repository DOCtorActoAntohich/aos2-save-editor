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
    #[error("Home directory is not defined")]
    Home,
}

#[derive(Debug, Deserialize)]
struct EnvVars {
    home: PathBuf,
}

impl From<EnvVars> for AoS2Env {
    fn from(EnvVars { home }: EnvVars) -> Self {
        let saves_folder = home.join("Documents").join("Fruitbat Factory").join("AoS2");

        Self { saves_folder }
    }
}

impl AoS2Env {
    pub fn from_env() -> Result<Self, Error> {
        let env_vars: EnvVars = envy::from_env().map_err(|_| Error::Home)?;
        Ok(env_vars.into())
    }
}
