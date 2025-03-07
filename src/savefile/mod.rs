use aos2_env::AoS2Env;
use online_profile::PlayerOnlineProfile;
use player_progress::{PlayableCharacters, PlayerProgress};
use tokio::sync::watch;

#[derive(Debug, Clone)]
pub struct Savefile {
    progress: Progress,
    profile: Profile,
}

#[derive(Debug, Clone)]
pub struct Progress {
    env: AoS2Env,
    progress: PlayerProgress,
    playable_characters: Channel<PlayableCharacters>,
}

#[derive(Debug, Clone)]
pub struct Profile {
    env: AoS2Env,
    profile: PlayerOnlineProfile,
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

#[derive(Debug, Clone)]
struct Channel<T> {
    sender: watch::Sender<T>,
    receiver: watch::Receiver<T>,
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
        let has_changed = self.progress.update();
        if has_changed {
            self.progress.save()?;
        }

        let has_changed = self.profile.update();
        if has_changed {
            self.profile.save()?;
        }

        Ok(())
    }
}

impl<T> Channel<T> {
    pub fn new(value: T) -> Self {
        let (sender, receiver) = watch::channel(value);
        Channel { sender, receiver }
    }

    pub fn has_changed(&self) -> bool {
        self.receiver
            .has_changed()
            .expect("Invariant: channel can't close: 1+ instance of sender/receiver always exists")
    }

    pub fn borrow_and_update(&mut self) -> watch::Ref<'_, T> {
        self.receiver.borrow_and_update()
    }
}

impl Progress {
    pub fn load(env: AoS2Env) -> Result<Self, Error> {
        let progress = PlayerProgress::load(&env)?.ok_or(Error::MissingProgress)?;
        let playable_characters = Channel::new(progress.playable_characters.clone());
        Ok(Self {
            env,
            progress,
            playable_characters,
        })
    }

    pub fn save(&mut self) -> Result<(), Error> {
        self.progress.save(&self.env).map_err(Error::Progress)
    }

    pub fn update(&mut self) -> bool {
        let mut has_changed = false;
        if self.playable_characters.has_changed() {
            let value = self.playable_characters.borrow_and_update().clone();
            self.progress.playable_characters = value;
            has_changed = true;
        }

        has_changed
    }

    pub fn write_playable_characters(&self) -> watch::Sender<PlayableCharacters> {
        self.playable_characters.sender.clone()
    }
}

impl Profile {
    pub fn load(env: AoS2Env) -> Result<Self, Error> {
        let profile = PlayerOnlineProfile::load(&env)?.ok_or(Error::MissingProfile)?;
        Ok(Self { env, profile })
    }

    pub fn save(&mut self) -> Result<(), Error> {
        self.profile.save(&self.env).map_err(Error::Profile)
    }

    pub fn update(&mut self) -> bool {
        let mut has_changed = false;
        has_changed
    }
}
