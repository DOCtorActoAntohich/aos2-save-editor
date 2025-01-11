use savefile::bin_bool::BinBool;

use super::Character;

/// Markers for full list of characters.
///
/// IMPORTANT: ORDER MATTERS.
/// That's how they are coded in game.
///
/// The games uses it to mark characters as follows:
///
/// - Locked/Unlocked in the character selection screen.
/// - Arcade Mode 1CC (no deaths) is completed or not.
#[binrw::binrw]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[brw(little)]
pub struct CharacterSheet {
    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub sora: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub alte: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub tsih: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub mira: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub sham: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub nath: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub star_breaker: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub suguri: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub saki: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub iru: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub nanako: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub kae: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub kyoko: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub hime: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub sumika: bool,
}

impl CharacterSheet {
    pub const N_CHARACTERS: usize = 15;

    pub const FULLY_UNLOCKED: Self = Self {
        sora: true,
        alte: true,
        tsih: true,
        mira: true,
        sham: true,
        nath: true,
        star_breaker: true,
        suguri: true,
        saki: true,
        iru: true,
        nanako: true,
        kae: true,
        kyoko: true,
        hime: true,
        sumika: true,
    };

    #[must_use]
    pub fn as_array(&self) -> [bool; CharacterSheet::N_CHARACTERS] {
        (*self).into()
    }

    pub fn iter(&self) -> impl Iterator<Item = (Character, bool)> {
        Character::list().into_iter().zip(self.as_array())
    }
}

impl Default for CharacterSheet {
    fn default() -> Self {
        Self {
            sora: true,
            alte: true,
            tsih: true,
            mira: true,
            sham: true,
            nath: true,
            suguri: true,
            saki: true,
            iru: true,
            nanako: true,
            kae: true,
            kyoko: true,

            star_breaker: false,
            hime: false,
            sumika: false,
        }
    }
}

impl From<CharacterSheet> for [bool; CharacterSheet::N_CHARACTERS] {
    fn from(
        CharacterSheet {
            sora,
            alte,
            tsih,
            mira,
            sham,
            nath,
            star_breaker,
            suguri,
            saki,
            iru,
            nanako,
            kae,
            kyoko,
            hime,
            sumika,
        }: CharacterSheet,
    ) -> Self {
        [
            sora,
            alte,
            tsih,
            mira,
            sham,
            nath,
            star_breaker,
            suguri,
            saki,
            iru,
            nanako,
            kae,
            kyoko,
            hime,
            sumika,
        ]
    }
}

impl From<[bool; CharacterSheet::N_CHARACTERS]> for CharacterSheet {
    fn from(
        [
        sora,
        alte,
        tsih,
        mira,
        sham,
        nath,
        star_breaker,
        suguri,
        saki,
        iru,
        nanako,
        kae,
        kyoko,
        hime,
        sumika,
    ]: [bool; CharacterSheet::N_CHARACTERS],
    ) -> Self {
        Self {
            sora,
            alte,
            tsih,
            mira,
            sham,
            nath,
            star_breaker,
            suguri,
            saki,
            iru,
            nanako,
            kae,
            kyoko,
            hime,
            sumika,
        }
    }
}
