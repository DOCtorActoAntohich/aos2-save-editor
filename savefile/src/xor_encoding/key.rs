use super::swap_nibbles;

/// A xor "encryption" key for bytes in `game.sys`.
///
/// Values are typical `u8`s with nibbles swapped.
/// That's why in a savefile they increment as follows:
/// - 0xDA
/// - 0xEA
/// - 0xFA
/// - 0x0B
/// - 0x1B
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Key(u8);

impl Key {
    pub const fn new(key: u8) -> Self {
        Self(key)
    }

    pub const fn get(self) -> u8 {
        self.0
    }

    pub const fn increment(self) -> Self {
        let Self(swapped) = self;
        let normal = swap_nibbles(swapped);
        Self(swap_nibbles(normal + 1))
    }
}
