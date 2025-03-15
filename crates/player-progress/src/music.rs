use std::ops::{Index, IndexMut};

use crate::{lock::Status, StatusSequence};

/// Only stock non-DLC music.
///
/// DLC music is not available in the savefile
/// cos the Steam client is supposed to track it.
#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq, Hash, derive_more::Deref, derive_more::AsRef)]
#[brw(little)]
#[as_ref(forward)]
pub struct MusicTracks([Status; MusicTracks::AMOUNT]);

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_more::TryFrom,
    derive_more::Display,
    enum_array::EnumMembersArray,
)]
#[try_from(repr)]
#[repr(usize)]
pub enum MusicTrack {
    #[display("Need for Speed")]
    NeedForSpeed = 0,
    #[display("Black Hole")]
    BlackHole = 1,
    #[display("Distant Thunder")]
    DistantThunder = 2,
    Swordfish = 3,
    Shine = 4,
    Expendables = 5,
    Ribbon = 6,
    #[display("Moving Out")]
    MovingOut = 7,
    Accelerator = 8,
    #[display("Remember Me")]
    RememberMe = 9,
    #[display("MGOM")]
    Mgom = 10,
}

impl MusicTracks {
    pub const AMOUNT: usize = 11;

    pub const ALL: Self = Self([Status::Enabled; Self::AMOUNT]);

    pub fn toggle(&mut self, music: MusicTrack) {
        self[music] = !self[music];
    }
}

impl Default for MusicTracks {
    fn default() -> Self {
        let mut music = Self::ALL;

        music[MusicTrack::Swordfish] = Status::Disabled;
        music[MusicTrack::Accelerator] = Status::Disabled;
        music[MusicTrack::RememberMe] = Status::Disabled;
        music[MusicTrack::Mgom] = Status::Disabled;

        music
    }
}

impl StatusSequence for MusicTracks {
    fn toggle_at(&mut self, index: usize) {
        if let Ok(music) = MusicTrack::try_from(index) {
            self.toggle(music);
        }
    }

    fn list(&self) -> Vec<(String, Status)> {
        let Self(music) = self;
        MusicTrack::members()
            .into_iter()
            .zip(music.iter().copied())
            .map(|(name, status)| (name.to_string(), status))
            .collect()
    }
}

impl Index<MusicTrack> for MusicTracks {
    type Output = Status;

    fn index(&self, index: MusicTrack) -> &Self::Output {
        let Self(music) = self;
        &music[index as usize]
    }
}

impl IndexMut<MusicTrack> for MusicTracks {
    fn index_mut(&mut self, index: MusicTrack) -> &mut Self::Output {
        let Self(music) = self;
        &mut music[index as usize]
    }
}
