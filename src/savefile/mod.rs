pub mod profile;
pub mod progress;

mod channel;

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
    #[error(transparent)]
    Env(#[from] aos2_env::Error),
    #[error("Failed to open `{}`", PlayerProgress::FILE_NAME)]
    Progress(#[from] player_progress::Error),
    #[error("Missing player progress file: {}", PlayerProgress::FILE_NAME)]
    MissingProgress,
    #[error("Failed to open `{}`", PlayerOnlineProfile::FILE_NAME)]
    Profile(#[from] online_profile::Error),
    #[error("Missing player online profile: `{}`", PlayerOnlineProfile::FILE_NAME)]
    MissingProfile,
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
