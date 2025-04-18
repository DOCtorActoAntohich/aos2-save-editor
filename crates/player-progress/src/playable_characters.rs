use std::ops::{Index, IndexMut};

use crate::Status;

#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq, Hash, derive_more::Deref, derive_more::AsRef)]
#[brw(little)]
#[as_ref(forward)]
pub struct PlayableCharacters([Status; PlayableCharacters::AMOUNT]);

/// IMPORTANT: ORDER AND INDEX VALUES MATTER.
/// That's how they are coded in game.
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
pub enum Character {
    Sora = 0,
    Alte = 1,
    Tsih = 2,
    Mira = 3,
    Sham = 4,
    Nath = 5,
    #[display("Star Breaker")]
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
    pub const AMOUNT: usize = 15;

    pub const ALL: Self = Self([Status::Enabled; Self::AMOUNT]);

    pub fn toggle(&mut self, character: Character) {
        self[character] = !self[character];
    }
}

impl Index<Character> for PlayableCharacters {
    type Output = Status;

    fn index(&self, index: Character) -> &Self::Output {
        let Self(characters) = self;
        &characters[index as usize]
    }
}

impl IndexMut<Character> for PlayableCharacters {
    fn index_mut(&mut self, index: Character) -> &mut Self::Output {
        let Self(characters) = self;
        &mut characters[index as usize]
    }
}

impl Default for PlayableCharacters {
    fn default() -> Self {
        let mut characters = Self::ALL;

        characters[Character::StarBreaker] = Status::Disabled;
        characters[Character::Hime] = Status::Disabled;
        characters[Character::Sumika] = Status::Disabled;

        characters
    }
}
