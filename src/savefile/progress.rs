use aos2_env::AoS2Env;
use player_progress::{
    Arenas, MusicTracks, PerfectArcadeMode, PerfectStoryMode, PlayableCharacters, PlayerProgress,
    SingleplayerWins,
};
use tokio::sync::watch;

use super::{channel::Channel, Error};

trait GetFn<T>: Send + Fn(&PlayerProgress) -> T {}
trait ModifyFn<T>: Send + Fn(&mut PlayerProgress, T) {}

impl<A, T> GetFn<T> for A where A: Send + Fn(&PlayerProgress) -> T {}

impl<A, T> ModifyFn<T> for A where A: Send + Fn(&mut PlayerProgress, T) {}

#[derive(Debug, Clone)]
pub struct Progress {
    progress: Channel<PlayerProgress>,
}

#[derive(Debug, Clone)]
pub struct ComplationStats {
    pub arcade_easy: PerfectArcadeMode,
    pub arcade_medium: PerfectArcadeMode,
    pub arcade_hard: PerfectArcadeMode,
    pub story_any: PerfectStoryMode,
}

pub struct Modify<T> {
    progress: watch::Sender<PlayerProgress>,
    modify: Box<dyn ModifyFn<T>>,
    get: Box<dyn GetFn<T>>,
}

pub struct Read<T> {
    progress: watch::Receiver<PlayerProgress>,
    get: Box<dyn Fn(&PlayerProgress) -> T + Send>,
}

impl Progress {
    pub fn load(env: &AoS2Env) -> Result<Self, Error> {
        let progress = PlayerProgress::load(env)?.ok_or(Error::MissingProgress)?;

        Ok(Self {
            progress: Channel::new(progress),
        })
    }

    pub fn save(&mut self, env: &AoS2Env) -> Result<(), Error> {
        if self.progress.has_changed() {
            self.progress
                .borrow_and_update()
                .save(env)
                .map_err(Error::Progress)
        } else {
            Ok(())
        }
    }

    #[must_use]
    pub fn read_completion_stats(&self) -> Read<ComplationStats> {
        Read {
            progress: self.progress.receiver(),
            get: Box::new(|progress: &PlayerProgress| ComplationStats {
                arcade_easy: progress.arcade_easy_1ccs.clone(),
                arcade_medium: progress.arcade_medium_1ccs.clone(),
                arcade_hard: progress.arcade_hard_1ccs.clone(),
                story_any: progress.story_1ccs.clone(),
            }),
        }
    }

    #[must_use]
    pub fn read_wins(&self) -> Read<SingleplayerWins> {
        Read {
            progress: self.progress.receiver(),
            get: Box::new(|progress: &PlayerProgress| progress.wins.clone()),
        }
    }

    #[must_use]
    pub fn modify_playable_characters(&self) -> Modify<PlayableCharacters> {
        Modify {
            progress: self.progress.sender(),
            modify: Box::new(
                |progress: &mut PlayerProgress, characters: PlayableCharacters| {
                    progress.playable_characters = characters;
                },
            ),
            get: Box::new(|progress: &PlayerProgress| progress.playable_characters.clone()),
        }
    }

    #[must_use]
    pub fn modify_arenas(&self) -> Modify<Arenas> {
        Modify {
            progress: self.progress.sender(),
            modify: Box::new(|progress: &mut PlayerProgress, arenas: Arenas| {
                progress.arenas = arenas;
            }),
            get: Box::new(|progress: &PlayerProgress| progress.arenas.clone()),
        }
    }

    #[must_use]
    pub fn modify_music_tracks(&self) -> Modify<MusicTracks> {
        Modify {
            progress: self.progress.sender(),
            modify: Box::new(|progress: &mut PlayerProgress, music: MusicTracks| {
                progress.music_tracks = music;
            }),
            get: Box::new(|progress: &PlayerProgress| progress.music_tracks.clone()),
        }
    }
}

impl<T> Modify<T> {
    pub fn send(&mut self, value: T) {
        self.progress.send_modify(|progress| {
            (self.modify)(progress, value);
        });
    }

    #[must_use]
    pub fn get(&self) -> T {
        (self.get)(&self.progress.borrow())
    }
}

impl<T> Read<T> {
    #[must_use]
    pub fn get(&self) -> T {
        let progress = self.progress.borrow();
        (self.get)(&progress)
    }
}

#[cfg(test)]
mod tests {
    use super::Modify;

    fn ensure_send<T: Send>() {}

    #[rstest::rstest]
    fn modify_progress_is_send() {
        ensure_send::<Modify<()>>();
    }
}
