pub mod full;
pub mod story;

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

impl Character {
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
