use aos2_env::AoS2Env;
use online_profile::{avatar, title, PlayerOnlineProfile};
use player_progress::{Arenas, MusicTracks, PlayableCharacters, PlayerProgress, SingleplayerWins};
use tokio::sync::watch;

#[derive(Debug, Clone)]
pub struct Savefile {
    progress: Progress,
    profile: Profile,
}

#[derive(Debug, Clone)]
pub struct Progress {
    env: AoS2Env,
    progress: Channel<PlayerProgress>,
    wins: Channel<SingleplayerWins>,
    playable_characters: Channel<PlayableCharacters>,
    arenas: Channel<Arenas>,
    music_tracks: Channel<MusicTracks>,
}

#[derive(Debug, Clone)]
pub struct Profile {
    env: AoS2Env,
    profile: Channel<PlayerOnlineProfile>,
}

pub struct ModifyProfile<T> {
    profile: watch::Sender<PlayerOnlineProfile>,
    modify: Box<dyn Fn(&mut PlayerOnlineProfile, T) + Send>,
    get: Box<dyn Fn(&PlayerOnlineProfile) -> T + Send>,
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

        self.profile.save()?;

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

        let wins = Channel::new(progress.wins.clone());

        let playable_characters = Channel::new(progress.playable_characters.clone());
        let arenas = Channel::new(progress.arenas.clone());
        let music_tracks = Channel::new(progress.music_tracks.clone());

        Ok(Self {
            env,
            progress: Channel::new(progress),
            wins,
            playable_characters,
            arenas,
            music_tracks,
        })
    }

    pub fn save(&mut self) -> Result<(), Error> {
        self.progress
            .receiver
            .borrow()
            .save(&self.env)
            .map_err(Error::Progress)
    }

    pub fn update(&mut self) -> bool {
        let mut has_changed = false;

        if self.playable_characters.has_changed() {
            let value = self.playable_characters.borrow_and_update().clone();
            self.progress
                .sender
                .send_modify(|progress| progress.playable_characters = value);
            has_changed = true;
        }

        if self.arenas.has_changed() {
            let value = self.arenas.borrow_and_update().clone();
            self.progress
                .sender
                .send_modify(|progress| progress.arenas = value);
            has_changed = true
        }

        if self.music_tracks.has_changed() {
            let value = self.music_tracks.borrow_and_update().clone();
            self.progress
                .sender
                .send_modify(|progress| progress.music_tracks = value);
            has_changed = true
        }

        has_changed
    }

    pub fn read_all(&self) -> watch::Receiver<PlayerProgress> {
        self.progress.receiver.clone()
    }

    pub fn read_wins(&self) -> watch::Receiver<SingleplayerWins> {
        self.wins.receiver.clone()
    }

    pub fn write_playable_characters(&self) -> watch::Sender<PlayableCharacters> {
        self.playable_characters.sender.clone()
    }

    pub fn write_arenas(&self) -> watch::Sender<Arenas> {
        self.arenas.sender.clone()
    }

    pub fn write_music_tracks(&self) -> watch::Sender<MusicTracks> {
        self.music_tracks.sender.clone()
    }
}

impl Profile {
    pub fn load(env: AoS2Env) -> Result<Self, Error> {
        let profile = PlayerOnlineProfile::load(&env)?.ok_or(Error::MissingProfile)?;
        Ok(Self {
            env,
            profile: Channel::new(profile),
        })
    }

    pub fn save(&mut self) -> Result<(), Error> {
        if self.profile.has_changed() {
            self.profile
                .borrow_and_update()
                .save(&self.env)
                .map_err(Error::Profile)
        } else {
            Ok(())
        }
    }

    pub fn write_title_character(&self) -> ModifyProfile<title::Character> {
        ModifyProfile {
            profile: self.profile.sender.clone(),
            modify: Box::new(
                |profile: &mut PlayerOnlineProfile, character: title::Character| {
                    profile.title_character_in_background = character;
                },
            ),
            get: Box::new(|profile: &PlayerOnlineProfile| profile.title_character_in_background),
        }
    }

    pub fn write_title_color(&self) -> ModifyProfile<title::Color> {
        ModifyProfile {
            profile: self.profile.sender.clone(),
            modify: Box::new(|profile: &mut PlayerOnlineProfile, color: title::Color| {
                profile.title_color = color;
            }),
            get: Box::new(|profile: &PlayerOnlineProfile| profile.title_color),
        }
    }

    pub fn write_title_text(&self) -> ModifyProfile<title::Text> {
        ModifyProfile {
            profile: self.profile.sender.clone(),
            modify: Box::new(|profile: &mut PlayerOnlineProfile, text: title::Text| {
                profile.title_text_id = text;
            }),
            get: Box::new(|profile: &PlayerOnlineProfile| profile.title_text_id),
        }
    }

    pub fn write_avatar_character(&self) -> ModifyProfile<avatar::Character> {
        ModifyProfile {
            profile: self.profile.sender.clone(),
            modify: Box::new(
                |profile: &mut PlayerOnlineProfile, character: avatar::Character| {
                    profile.avatar_character = character;
                },
            ),
            get: Box::new(|profile: &PlayerOnlineProfile| profile.avatar_character),
        }
    }

    pub fn write_avatar_background(&self) -> ModifyProfile<avatar::Background> {
        ModifyProfile {
            profile: self.profile.sender.clone(),
            modify: Box::new(
                |profile: &mut PlayerOnlineProfile, background: avatar::Background| {
                    profile.avatar_background = background;
                },
            ),
            get: Box::new(|profile: &PlayerOnlineProfile| profile.avatar_background),
        }
    }
}

impl<T> ModifyProfile<T> {
    pub fn get(&self) -> T {
        let profile = self.profile.borrow();
        (self.get)(&profile)
    }

    pub fn send(&mut self, value: T) {
        self.profile.send_modify(|profile| {
            (self.modify)(profile, value);
        });
    }
}

#[cfg(test)]
mod tests {

    use super::ModifyProfile;

    fn ensure_send<T: Send>() {}

    #[rstest::rstest]
    fn modify_profile_is_send() {
        ensure_send::<ModifyProfile<()>>();
    }
}
