use crate::Status;

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
pub struct PlayableCharacters {
    pub sora: Status,
    pub alte: Status,
    pub tsih: Status,
    pub mira: Status,
    pub sham: Status,
    pub nath: Status,
    pub star_breaker: Status,
    pub suguri: Status,
    pub saki: Status,
    pub iru: Status,
    pub nanako: Status,
    pub kae: Status,
    pub kyoko: Status,
    pub hime: Status,
    pub sumika: Status,
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
)]
#[try_from(repr)]
#[repr(usize)]
pub enum Character {
    Sora = 0,
    Alte = 1,
    Tsih = 2,
    Mira = 3,
    Sham = 4,
    Nath = 5,
    StarBreaker = 6,
    Suguri = 7,
    Saki = 8,
    Iru = 9,
    Nanako = 10,
    Kae = 11,
    Kyoko = 12,
    Hime = 13,
    Sumika = 14,
}

impl PlayableCharacters {
    pub const N_CHARACTERS: usize = 15;

    pub const ALL: Self = Self {
        sora: Status::Enabled,
        alte: Status::Enabled,
        tsih: Status::Enabled,
        mira: Status::Enabled,
        sham: Status::Enabled,
        nath: Status::Enabled,
        star_breaker: Status::Enabled,
        suguri: Status::Enabled,
        saki: Status::Enabled,
        iru: Status::Enabled,
        nanako: Status::Enabled,
        kae: Status::Enabled,
        kyoko: Status::Enabled,
        hime: Status::Enabled,
        sumika: Status::Enabled,
    };

    #[must_use]
    pub fn as_array(&self) -> [Status; PlayableCharacters::N_CHARACTERS] {
        (*self).into()
    }

    pub fn iter(&self) -> impl Iterator<Item = (Character, Status)> {
        Character::list().into_iter().zip(self.as_array())
    }
}

impl Default for PlayableCharacters {
    fn default() -> Self {
        Self {
            sora: Status::Enabled,
            alte: Status::Enabled,
            tsih: Status::Enabled,
            mira: Status::Enabled,
            sham: Status::Enabled,
            nath: Status::Enabled,
            suguri: Status::Enabled,
            saki: Status::Enabled,
            iru: Status::Enabled,
            nanako: Status::Enabled,
            kae: Status::Enabled,
            kyoko: Status::Enabled,

            star_breaker: Status::Disabled,
            hime: Status::Disabled,
            sumika: Status::Disabled,
        }
    }
}

impl From<PlayableCharacters> for [Status; PlayableCharacters::N_CHARACTERS] {
    fn from(
        PlayableCharacters {
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
        }: PlayableCharacters,
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

impl From<[Status; PlayableCharacters::N_CHARACTERS]> for PlayableCharacters {
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
    ]: [Status; PlayableCharacters::N_CHARACTERS],
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

impl Character {
    #[must_use]
    pub fn list() -> impl IntoIterator<Item = Character> {
        [
            Self::Sora,
            Self::Alte,
            Self::Tsih,
            Self::Mira,
            Self::Sham,
            Self::Nath,
            Self::StarBreaker,
            Self::Suguri,
            Self::Saki,
            Self::Iru,
            Self::Nanako,
            Self::Kae,
            Self::Kyoko,
            Self::Hime,
            Self::Sumika,
        ]
    }
}
