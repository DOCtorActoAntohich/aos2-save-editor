use std::ops::{Index, IndexMut};

use crate::{lock::Status, StatusSequence, UnknownU8};

/// List of Background Images aka Arena Backgrounds.
///

#[binrw::binrw]
#[derive(Debug, Clone, derive_more::Deref, derive_more::AsRef)]
#[brw(little)]
#[br(map = From::<RawArenas>::from)]
#[bw(map = RawArenas::from)]
pub struct Arenas {
    #[deref]
    #[as_ref(forward)]
    arenas: [Status; Arenas::AMOUNT],
    unused_0x2d: UnknownU8,
}

/// IMPORTANT: ORDER MATTERS. Do not reorder.
#[binrw::binrw]
#[derive(Debug, Clone)]
#[brw(little)]
struct RawArenas {
    pub before_the_war: Status,
    pub war_10k_years_ago: Status,
    pub canyon_of_wind: Status,
    pub dust_storm: Status,
    pub rain_and_sunset: Status,
    pub equator_doldrums: Status,
    pub big_bridge: Status,
    pub capital_in_flames: Status,
    pub whirlpool_of_malice: Status,

    /// Seems to have no effect when modified, but is needed for proper parsing.
    unused_0x2d: UnknownU8,

    pub nature_10k: Status,
    pub crashed_spaceship: Status,
    pub guardians_chamber: Status,
    pub moonlight_dance_hall: Status,
    pub sumika_hideout: Status,
}

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
pub enum Arena {
    #[display("Before the War")]
    BeforeTheWar = 0,
    #[display("War 10k years ago")]
    War10kYearsAgo = 1,
    #[display("Canyon of Wind")]
    CanyonOfWind = 2,
    #[display("Dust Storm")]
    DustStorm = 3,
    #[display("Rain and Sunset")]
    RainAndSunset = 4,
    /// Some placeholder that looks like Rain and Sunset but has a buggy icon.
    /// Disabled by default.
    #[display("Equator Doldrums")]
    EquatorDoldrums = 5,
    #[display("Big Bridge")]
    BigBridge = 6,
    #[display("Capital in Flames")]
    CapitalInFlames = 7,
    #[display("Whirlpool of Malice")]
    WhirlpoolOfMalice = 8,
    #[display("Nature 10k")]
    Nature10k = 9,
    #[display("Crashed Spaceship")]
    CrashedSpaceship = 10,
    #[display("Guardian's Chamber")]
    GuardiansChamber = 11,
    #[display("Moonlight Dance Hall")]
    MoonlightDanceHall = 12,
    #[display("Sumika's Hideout")]
    SumikaHideout = 13,
}

impl Arenas {
    pub const AMOUNT: usize = 14;

    pub const ALL: Self = Self {
        arenas: [Status::Enabled; Self::AMOUNT],
        unused_0x2d: UnknownU8(0),
    };

    pub fn toggle(&mut self, arena: Arena) {
        self[arena] = !self[arena];
    }
}

impl StatusSequence for Arenas {
    fn toggle_at(&mut self, index: usize) {
        if let Ok(arena) = Arena::try_from(index) {
            self.toggle(arena);
        }
    }

    fn list(&self) -> Vec<(String, Status)> {
        let Self {
            arenas,
            unused_0x2d: _,
        } = self;
        Arena::members()
            .into_iter()
            .zip(arenas.iter().copied())
            .map(|(name, status)| (name.to_string(), status))
            .collect()
    }
}

impl Index<Arena> for Arenas {
    type Output = Status;

    fn index(&self, index: Arena) -> &Self::Output {
        let Self {
            arenas,
            unused_0x2d: _,
        } = self;
        &arenas[index as usize]
    }
}

impl IndexMut<Arena> for Arenas {
    fn index_mut(&mut self, index: Arena) -> &mut Self::Output {
        let Self {
            arenas,
            unused_0x2d: _,
        } = self;
        &mut arenas[index as usize]
    }
}

impl Default for Arenas {
    fn default() -> Self {
        let mut arenas = Self::ALL;

        arenas[Arena::CapitalInFlames] = Status::Disabled;
        arenas[Arena::WhirlpoolOfMalice] = Status::Disabled;
        arenas[Arena::CrashedSpaceship] = Status::Disabled;
        arenas[Arena::GuardiansChamber] = Status::Disabled;
        arenas[Arena::MoonlightDanceHall] = Status::Disabled;
        arenas[Arena::SumikaHideout] = Status::Disabled;

        arenas[Arena::EquatorDoldrums] = Status::Disabled;

        arenas
    }
}

/// Skips an unknown field because who knows what it does.
/// Otherwise must be derived.
impl PartialEq for Arenas {
    fn eq(&self, other: &Self) -> bool {
        self.arenas == other.arenas
    }
}

impl Eq for Arenas {}

impl From<RawArenas> for Arenas {
    fn from(
        RawArenas {
            before_the_war,
            war_10k_years_ago,
            canyon_of_wind,
            dust_storm,
            rain_and_sunset,
            equator_doldrums,
            big_bridge,
            capital_in_flames,
            whirlpool_of_malice,
            unused_0x2d,
            nature_10k,
            crashed_spaceship,
            guardians_chamber,
            moonlight_dance_hall,
            sumika_hideout,
        }: RawArenas,
    ) -> Self {
        let arenas = [
            before_the_war,
            war_10k_years_ago,
            canyon_of_wind,
            dust_storm,
            rain_and_sunset,
            equator_doldrums,
            big_bridge,
            capital_in_flames,
            whirlpool_of_malice,
            nature_10k,
            crashed_spaceship,
            guardians_chamber,
            moonlight_dance_hall,
            sumika_hideout,
        ];
        Self {
            arenas,
            unused_0x2d,
        }
    }
}

impl From<&Arenas> for RawArenas {
    fn from(arenas: &Arenas) -> Self {
        let Arenas {
            arenas,
            unused_0x2d,
        } = arenas.clone();
        let [before_the_war, war_10k_years_ago, canyon_of_wind, dust_storm, rain_and_sunset, equator_doldrums, big_bridge, capital_in_flames, whirlpool_of_malice, nature_10k, crashed_spaceship, guardians_chamber, moonlight_dance_hall, sumika_hideout] =
            arenas;
        Self {
            before_the_war,
            war_10k_years_ago,
            canyon_of_wind,
            dust_storm,
            rain_and_sunset,
            equator_doldrums,
            big_bridge,
            capital_in_flames,
            whirlpool_of_malice,
            unused_0x2d,
            nature_10k,
            crashed_spaceship,
            guardians_chamber,
            moonlight_dance_hall,
            sumika_hideout,
        }
    }
}
