use crate::{bin_bool::BinBool, file::local::UnknownU8};

#[binrw::binrw]
#[derive(Debug, Clone)]
#[brw(little)]
pub struct BackgroundImageSheet {
    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub before_the_war: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub war_10k_years_ago: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub canyon_of_wind: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub dust_storm: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub rain_and_sunset: bool,

    /// Some placeholder that looks like Rain and Sunset
    /// but has a buggy icon.
    /// Disabled by default.
    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub equator_doldrums: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub big_bridge: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub capital_in_flames: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub whirlpool_of_malice: bool,

    _0x2d: UnknownU8,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub nature_10k: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub crashed_spaceship: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub guardians_chamber: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub moonlight_dance_hall: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub sumika_hideout: bool,
}

impl BackgroundImageSheet {
    pub const FULLY_UNLOCKED: Self = Self {
        before_the_war: true,
        war_10k_years_ago: true,
        canyon_of_wind: true,
        dust_storm: true,
        rain_and_sunset: true,
        equator_doldrums: true,
        big_bridge: true,
        capital_in_flames: true,
        whirlpool_of_malice: true,
        _0x2d: UnknownU8(0),
        nature_10k: true,
        crashed_spaceship: true,
        guardians_chamber: true,
        moonlight_dance_hall: true,
        sumika_hideout: true,
    };
}

impl Default for BackgroundImageSheet {
    fn default() -> Self {
        Self {
            before_the_war: true,
            war_10k_years_ago: true,
            canyon_of_wind: true,
            dust_storm: true,
            rain_and_sunset: true,
            big_bridge: true,
            nature_10k: true,

            capital_in_flames: false,
            whirlpool_of_malice: false,
            crashed_spaceship: false,
            guardians_chamber: false,
            moonlight_dance_hall: false,
            sumika_hideout: false,

            equator_doldrums: false,
            _0x2d: UnknownU8(0),
        }
    }
}

/// Skips unknown field because who knows what it does.
/// Otherwise must be derived.
impl PartialEq for BackgroundImageSheet {
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

impl Eq for BackgroundImageSheet {}
