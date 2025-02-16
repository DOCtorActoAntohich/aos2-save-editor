use std::borrow::Cow;

use crate::lock::Status;

/// Only stock non-DLC music.
///
/// DLC music is not available in the savefile
/// cos the Steam client is supposed to track it.
#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[brw(little)]
pub struct MusicTracks {
    pub need_for_speed: Status,
    pub black_hole: Status,
    pub distant_thunder: Status,
    pub swordfish: Status,
    pub shine: Status,
    pub expendables: Status,
    pub ribbon: Status,
    pub moving_out: Status,
    pub accelerator: Status,
    pub remember_me: Status,
    pub mgom: Status,
}

impl Default for MusicTracks {
    fn default() -> Self {
        Self {
            need_for_speed: Status::Enabled,
            black_hole: Status::Enabled,
            distant_thunder: Status::Enabled,
            shine: Status::Enabled,
            expendables: Status::Enabled,
            ribbon: Status::Enabled,
            moving_out: Status::Enabled,

            swordfish: Status::Disabled,
            accelerator: Status::Disabled,
            remember_me: Status::Disabled,
            mgom: Status::Disabled,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Display)]
pub enum MusicTrack {
    #[display("Need for Speed")]
    NeedForSpeed,
    #[display("Black Hole")]
    BlackHole,
    #[display("Distant Thunder")]
    DistantThunder,
    Swordfish,
    Shine,
    Expendables,
    Ribbon,
    #[display("Moving Out")]
    MovingOut,
    Accelerator,
    #[display("Remember Me")]
    RememberMe,
    #[display("MGOM")]
    Mgom,
}

impl MusicTracks {
    pub const AMOUNT: usize = 11;

    pub const ALL: Self = Self {
        need_for_speed: Status::Enabled,
        black_hole: Status::Enabled,
        distant_thunder: Status::Enabled,
        swordfish: Status::Enabled,
        shine: Status::Enabled,
        expendables: Status::Enabled,
        ribbon: Status::Enabled,
        moving_out: Status::Enabled,
        accelerator: Status::Enabled,
        remember_me: Status::Enabled,
        mgom: Status::Enabled,
    };

    #[must_use]
    pub fn to_array(&self) -> [Status; Self::AMOUNT] {
        self.clone().into()
    }

    pub fn iter(&self) -> impl Iterator<Item = (MusicTrack, Status)> {
        MusicTrack::list().into_iter().zip(self.to_array())
    }
}

impl From<MusicTracks> for [Status; MusicTracks::AMOUNT] {
    fn from(
        MusicTracks {
            need_for_speed,
            black_hole,
            distant_thunder,
            swordfish,
            shine,
            expendables,
            ribbon,
            moving_out,
            accelerator,
            remember_me,
            mgom,
        }: MusicTracks,
    ) -> Self {
        [
            need_for_speed,
            black_hole,
            distant_thunder,
            swordfish,
            shine,
            expendables,
            ribbon,
            moving_out,
            accelerator,
            remember_me,
            mgom,
        ]
    }
}

impl From<[Status; MusicTracks::AMOUNT]> for MusicTracks {
    fn from(
        [
        need_for_speed,
        black_hole,
        distant_thunder,
        swordfish,
        shine,
        expendables,
        ribbon,
        moving_out,
        accelerator,
        remember_me,
        mgom,
    ]: [Status; MusicTracks::AMOUNT],
    ) -> Self {
        Self {
            need_for_speed,
            black_hole,
            distant_thunder,
            swordfish,
            shine,
            expendables,
            ribbon,
            moving_out,
            accelerator,
            remember_me,
            mgom,
        }
    }
}

impl MusicTrack {
    pub const fn list() -> impl IntoIterator<Item = Self> {
        [
            Self::NeedForSpeed,
            Self::BlackHole,
            Self::DistantThunder,
            Self::Swordfish,
            Self::Shine,
            Self::Expendables,
            Self::Ribbon,
            Self::MovingOut,
            Self::Accelerator,
            Self::RememberMe,
            Self::Mgom,
        ]
    }
}

impl From<MusicTrack> for Cow<'_, str> {
    fn from(value: MusicTrack) -> Self {
        value.to_string().into()
    }
}
