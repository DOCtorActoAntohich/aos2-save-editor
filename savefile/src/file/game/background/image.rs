use binrw::{BinRead, BinWrite};

use crate::{bin_bool::BinBool, file::game::UnknownU8};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BackgroundImageSheet {
    pub before_the_war: bool,
    pub war_10k_years_ago: bool,
    pub canyon_of_wind: bool,
    pub dust_storm: bool,
    pub rain_and_sunset: bool,
    /// Some placeholder that looks like Rain and Sunset
    /// but has a buggy icon.
    /// Disabled by default.
    pub equator_doldrums: bool,
    pub big_bridge: bool,
    pub capital_in_flames: bool,
    pub whirlpool_of_malice: bool,
    _0x2d: UnknownU8,
    pub nature_10k: bool,
    pub crashed_spaceship: bool,
    pub guardians_chamber: bool,
    pub moonlight_dance_hall: bool,
    pub sumika_hideout: bool,
}

#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq)]
#[brw(little)]
pub struct BinarySheet {
    before_the_war: BinBool,
    war_10k_years_ago: BinBool,
    canyon_of_wind: BinBool,
    dust_storm: BinBool,
    rain_and_sunset: BinBool,
    equator_doldrums: BinBool,
    big_bridge: BinBool,
    capital_in_flames: BinBool,
    whirlpool_of_malice: BinBool,
    _0x2d: UnknownU8,
    nature_10k: BinBool,
    crashed_spaceship: BinBool,
    guardians_chamber: BinBool,
    moonlight_dance_hall: BinBool,
    sumika_hideout: BinBool,
}

impl BinRead for BackgroundImageSheet {
    type Args<'a> = <BinarySheet as BinRead>::Args<'a>;

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        <BinarySheet as BinRead>::read_options(reader, endian, args).map(Into::into)
    }
}

impl BinWrite for BackgroundImageSheet {
    type Args<'a> = <BinarySheet as BinWrite>::Args<'a>;

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        let binary: BinarySheet = self.clone().into();
        <BinarySheet as BinWrite>::write_options(&binary, writer, endian, args)
    }
}

impl From<BinarySheet> for BackgroundImageSheet {
    fn from(
        BinarySheet {
            before_the_war,
            war_10k_years_ago,
            canyon_of_wind,
            dust_storm,
            rain_and_sunset,
            equator_doldrums,
            big_bridge,
            capital_in_flames,
            whirlpool_of_malice,
            _0x2d,
            nature_10k,
            crashed_spaceship,
            guardians_chamber,
            moonlight_dance_hall,
            sumika_hideout,
        }: BinarySheet,
    ) -> Self {
        Self {
            before_the_war: before_the_war.into(),
            war_10k_years_ago: war_10k_years_ago.into(),
            canyon_of_wind: canyon_of_wind.into(),
            dust_storm: dust_storm.into(),
            rain_and_sunset: rain_and_sunset.into(),
            equator_doldrums: equator_doldrums.into(),
            big_bridge: big_bridge.into(),
            capital_in_flames: capital_in_flames.into(),
            whirlpool_of_malice: whirlpool_of_malice.into(),
            _0x2d,
            nature_10k: nature_10k.into(),
            crashed_spaceship: crashed_spaceship.into(),
            guardians_chamber: guardians_chamber.into(),
            moonlight_dance_hall: moonlight_dance_hall.into(),
            sumika_hideout: sumika_hideout.into(),
        }
    }
}

impl From<BackgroundImageSheet> for BinarySheet {
    fn from(
        BackgroundImageSheet {
            before_the_war,
            war_10k_years_ago,
            canyon_of_wind,
            dust_storm,
            rain_and_sunset,
            equator_doldrums,
            big_bridge,
            capital_in_flames,
            whirlpool_of_malice,
            _0x2d,
            nature_10k,
            crashed_spaceship,
            guardians_chamber,
            moonlight_dance_hall,
            sumika_hideout,
        }: BackgroundImageSheet,
    ) -> Self {
        Self {
            before_the_war: before_the_war.into(),
            war_10k_years_ago: war_10k_years_ago.into(),
            canyon_of_wind: canyon_of_wind.into(),
            dust_storm: dust_storm.into(),
            rain_and_sunset: rain_and_sunset.into(),
            equator_doldrums: equator_doldrums.into(),
            big_bridge: big_bridge.into(),
            capital_in_flames: capital_in_flames.into(),
            whirlpool_of_malice: whirlpool_of_malice.into(),
            _0x2d,
            nature_10k: nature_10k.into(),
            crashed_spaceship: crashed_spaceship.into(),
            guardians_chamber: guardians_chamber.into(),
            moonlight_dance_hall: moonlight_dance_hall.into(),
            sumika_hideout: sumika_hideout.into(),
        }
    }
}
