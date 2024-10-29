use crate::{bin_bool::BinBool, file::game::UnknownU8};

#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
        _0x2d: UnknownU8(1),
        nature_10k: true,
        crashed_spaceship: true,
        guardians_chamber: true,
        moonlight_dance_hall: true,
        sumika_hideout: true,
    };
}
