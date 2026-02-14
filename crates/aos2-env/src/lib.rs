#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

use std::path::Path;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
pub const EXAMPLE_HOME: &str = "C:/Users/<user>";
#[cfg(target_os = "linux")]
pub const EXAMPLE_HOME: &str = "/home/<user>";
#[cfg(target_os = "macos")]
pub const EXAMPLE_HOME: &str = "/Users/<user>";

#[derive(Debug, Clone)]
pub struct AoS2Env {
    pub saves_folder: PathBuf,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Home directory is not defined")]
    Home,
}

impl AoS2Env {
    pub fn from_home_dir() -> Result<Self, Error> {
        std::env::home_dir()
            .map(saves_location)
            .map(|saves_folder| Self { saves_folder })
            .ok_or(Error::Home)
    }

    pub fn from_path(path: impl Into<PathBuf>) -> Self {
        Self {
            saves_folder: path.into(),
        }
    }
}

#[cfg(target_os = "windows")]
pub fn saves_location(home: impl AsRef<Path>) -> PathBuf {
    home.as_ref()
        .join("Documents")
        .join("Fruitbat Factory")
        .join("AoS2")
}

#[cfg(target_os = "linux")]
pub fn saves_location(home: impl AsRef<Path>) -> PathBuf {
    // This is the cringe location where I had it.
    // Not sure if it's universal enough but "it works on my machine". xd.
    home.as_ref()
        .join(".local")
        .join("share")
        .join("Steam")
        .join("steamapps")
        .join("compatdata")
        .join("390710")
        .join("pfx")
        .join("drive_c")
        .join("users")
        .join("steamuser")
        .join("Documents")
        .join("Fruitbat Factory")
        .join("AoS2")
}

/// Note: This is a crutch to just make it compile for `MacOS`.
/// Akshually, `AoS2` doesn't run on `MacOS`,
/// but this location is where 100% OJ stores its savedata, so
/// this is where theoretical `AoS2` for Mac would store its data too.
#[cfg(target_os = "macos")]
pub fn saves_location(home: impl AsRef<Path>) -> PathBuf {
    home.as_ref()
        .join("Library")
        .join("Application Support")
        .join("FBF")
        .join("AoS2")
}
