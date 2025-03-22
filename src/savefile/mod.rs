pub mod profile;
pub mod progress;

mod channel;

use std::fmt::Display;

use aos2_env::AoS2Env;
use online_profile::PlayerOnlineProfile;
use player_progress::PlayerProgress;

use self::{profile::Profile, progress::Progress};

#[derive(Debug, Clone)]
pub struct Savefile {
    aos2_env: AoS2Env,
    progress: Progress,
    profile: Profile,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    Env(#[from] aos2_env::Error),
    Progress(#[from] player_progress::Error),
    Profile(#[from] online_profile::Error),
}

impl Savefile {
    pub fn from_env() -> Result<Self, Error> {
        let env = AoS2Env::from_env()?;
        Self::load(env)
    }

    pub fn load(aos2_env: AoS2Env) -> Result<Self, Error> {
        let progress = Progress::load(&aos2_env)?;
        let profile = Profile::load(&aos2_env)?;

        Ok(Self {
            aos2_env,
            progress,
            profile,
        })
    }

    #[must_use]
    pub fn progress(&self) -> &Progress {
        &self.progress
    }

    #[must_use]
    pub fn profile(&self) -> &Profile {
        &self.profile
    }

    pub fn save_all(&mut self) -> Result<(), Error> {
        self.progress.save(&self.aos2_env)?;
        self.profile.save(&self.aos2_env)?;

        Ok(())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Env(error) => Display::fmt(error, f),
            Error::Progress(error) => {
                writeln!(f, "Failed to open `{}`:", PlayerProgress::FILE_NAME)?;
                writeln!(f, "- {}", error)
            }
            Error::Profile(error) => {
                writeln!(f, "Failed to open `{}`:", PlayerOnlineProfile::FILE_NAME)?;
                writeln!(f, "- {}", error)
            }
        }
    }
}
