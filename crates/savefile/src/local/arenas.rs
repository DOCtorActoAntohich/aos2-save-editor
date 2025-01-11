use crate::{local::UnknownU8, lock::Status};

/// List of Background Images aka Arena Backgrounds.
///
/// IMPORTANT: ORDER MATTERS. Do not reorder.
#[binrw::binrw]
#[derive(Debug, Clone)]
#[brw(little)]
pub struct Arenas {
    pub before_the_war: Status,
    pub war_10k_years_ago: Status,
    pub canyon_of_wind: Status,
    pub dust_storm: Status,
    pub rain_and_sunset: Status,
    /// Some placeholder that looks like Rain and Sunset but has a buggy icon.
    /// Disabled by default.
    pub equator_doldrums: Status,
    pub big_bridge: Status,
    pub capital_in_flames: Status,
    pub whirlpool_of_malice: Status,

    /// Seems to have no effect when modified, but is needed for proper parsing.
    _0x2d: UnknownU8,

    pub nature_10k: Status,
    pub crashed_spaceship: Status,
    pub guardians_chamber: Status,
    pub moonlight_dance_hall: Status,
    pub sumika_hideout: Status,
}

impl Arenas {
    pub const ALL: Self = Self {
        before_the_war: Status::Enabled,
        war_10k_years_ago: Status::Enabled,
        canyon_of_wind: Status::Enabled,
        dust_storm: Status::Enabled,
        rain_and_sunset: Status::Enabled,
        equator_doldrums: Status::Enabled,
        big_bridge: Status::Enabled,
        capital_in_flames: Status::Enabled,
        whirlpool_of_malice: Status::Enabled,
        _0x2d: UnknownU8(0),
        nature_10k: Status::Enabled,
        crashed_spaceship: Status::Enabled,
        guardians_chamber: Status::Enabled,
        moonlight_dance_hall: Status::Enabled,
        sumika_hideout: Status::Enabled,
    };
}

impl Default for Arenas {
    fn default() -> Self {
        Self {
            before_the_war: Status::Enabled,
            war_10k_years_ago: Status::Enabled,
            canyon_of_wind: Status::Enabled,
            dust_storm: Status::Enabled,
            rain_and_sunset: Status::Enabled,
            big_bridge: Status::Enabled,
            nature_10k: Status::Enabled,

            capital_in_flames: Status::Disabled,
            whirlpool_of_malice: Status::Disabled,
            crashed_spaceship: Status::Disabled,
            guardians_chamber: Status::Disabled,
            moonlight_dance_hall: Status::Disabled,
            sumika_hideout: Status::Disabled,

            equator_doldrums: Status::Disabled,
            _0x2d: UnknownU8(0),
        }
    }
}

/// Skips an unknown field because who knows what it does.
/// Otherwise must be derived.
impl PartialEq for Arenas {
    fn eq(&self, other: &Self) -> bool {
        self.before_the_war == other.before_the_war
            && self.war_10k_years_ago == other.war_10k_years_ago
            && self.canyon_of_wind == other.canyon_of_wind
            && self.dust_storm == other.dust_storm
            && self.rain_and_sunset == other.rain_and_sunset
            && self.equator_doldrums == other.equator_doldrums
            && self.big_bridge == other.big_bridge
            && self.capital_in_flames == other.capital_in_flames
            && self.whirlpool_of_malice == other.whirlpool_of_malice
            && self.nature_10k == other.nature_10k
            && self.crashed_spaceship == other.crashed_spaceship
            && self.guardians_chamber == other.guardians_chamber
            && self.moonlight_dance_hall == other.moonlight_dance_hall
            && self.sumika_hideout == other.sumika_hideout
    }
}

impl Eq for Arenas {}
