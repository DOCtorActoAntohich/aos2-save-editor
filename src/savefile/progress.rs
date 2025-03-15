use aos2_env::AoS2Env;
use player_progress::{
    Arenas, MusicTracks, PerfectArcadeMode, PerfectStoryMode, PlayableCharacters, PlayerProgress,
    SingleplayerWins,
};
use tokio::sync::watch;

use super::{channel::Channel, Error};

#[derive(Debug, Clone)]
pub struct Progress {
    env: AoS2Env,
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
    modify: Box<dyn Fn(&mut PlayerProgress, T) + Send>,
    get: Box<dyn Fn(&PlayerProgress) -> T + Send>,
}

pub struct Read<T> {
    progress: watch::Receiver<PlayerProgress>,
    get: Box<dyn Fn(&PlayerProgress) -> T + Send>,
}

impl Progress {
    pub fn load(env: AoS2Env) -> Result<Self, Error> {
        let progress = PlayerProgress::load(&env)?.ok_or(Error::MissingProgress)?;

        Ok(Self {
            env,
            progress: Channel::new(progress),
        })
    }

    pub fn save(&mut self) -> Result<(), Error> {
        if self.progress.has_changed() {
            self.progress
                .borrow_and_update()
                .save(&self.env)
                .map_err(Error::Progress)
        } else {
            Ok(())
        }
    }

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

    pub fn read_wins(&self) -> Read<SingleplayerWins> {
        Read {
            progress: self.progress.receiver(),
            get: Box::new(|progress: &PlayerProgress| progress.wins.clone()),
        }
    }

    pub fn write_playable_characters(&self) -> Modify<PlayableCharacters> {
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

    pub fn write_arenas(&self) -> Modify<Arenas> {
        Modify {
            progress: self.progress.sender(),
            modify: Box::new(|progress: &mut PlayerProgress, arenas: Arenas| {
                progress.arenas = arenas;
            }),
            get: Box::new(|progress: &PlayerProgress| progress.arenas.clone()),
        }
    }

    pub fn write_music_tracks(&self) -> Modify<MusicTracks> {
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

    pub fn get(&self) -> T {
        (self.get)(&self.progress.borrow())
    }
}

impl<T> Read<T> {
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
