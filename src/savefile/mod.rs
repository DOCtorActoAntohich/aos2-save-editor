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

    pub fn progress(&self) -> &Progress {
        &self.progress
    }

    pub fn mut_progress(&mut self) -> &mut Progress {
        &mut self.progress
    }

    pub fn profile(&self) -> &Profile {
        &self.profile
    }

    pub fn mut_profile(&mut self) -> &mut Profile {
        &mut self.profile
    }

    pub fn update_and_save(&mut self) -> Result<(), Error> {
        self.progress.save()?;
        self.profile.save()?;

        Ok(())
    }
}
