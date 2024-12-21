pub mod full;
pub mod story;

use std::ops::{Add, AddAssign, Sub, SubAssign};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct CharacterIndex(usize);

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

impl Add<usize> for CharacterIndex {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        let Self(lhs) = self;
        lhs.saturating_add(rhs).into()
    }
}

impl AddAssign<usize> for CharacterIndex {
    fn add_assign(&mut self, rhs: usize) {
        *self = *self + rhs
    }
}

impl Sub<usize> for CharacterIndex {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        let Self(lhs) = self;
        lhs.saturating_sub(rhs).into()
    }
}

impl SubAssign<usize> for CharacterIndex {
    fn sub_assign(&mut self, rhs: usize) {
        *self = *self - rhs
    }
}

impl From<usize> for CharacterIndex {
    fn from(value: usize) -> Self {
        Self(value.clamp(0, 14))
    }
}

impl From<CharacterIndex> for Character {
    fn from(CharacterIndex(index): CharacterIndex) -> Self {
        Character::try_from(index).expect("Invariant Broken: character index was invalid")
    }
}
