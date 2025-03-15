pub mod profile;
pub mod progress;

mod channel;

use aos2_env::AoS2Env;

use self::{profile::Profile, progress::Progress};

#[derive(Debug, Clone)]
pub struct Savefile {
    progress: Progress,
    profile: Profile,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Progress(#[from] player_progress::Error),
    #[error("Missing player progress file")]
    MissingProgress,
    #[error(transparent)]
    Profile(#[from] online_profile::Error),
    #[error("Missing player online profile")]
    MissingProfile,
}

impl Savefile {
    pub fn load(env: AoS2Env) -> Result<Self, Error> {
        let progress = Progress::load(env.clone())?;
        let profile = Profile::load(env)?;

        Ok(Self { progress, profile })
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
        self.progress.save()?;
        self.profile.save()?;

        Ok(())
    }
}
