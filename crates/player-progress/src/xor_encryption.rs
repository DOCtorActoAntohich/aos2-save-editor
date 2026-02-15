/// `AoS2` uses a fun data encryption algorithm for relatively important data.
///
/// See [`KeyU8`] for more info.
#[must_use]
#[binrw::binrw]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::Display)]
#[brw(little)]
#[display("{_0:#x}")]
pub struct EncryptedU8(u8);

/// A xor "encryption" key for `game.sys`.
///
/// To encrypt/decrypt a stream, one has to know the start key.
/// For each next byte in the stream, the key "increments".
///
/// Each key is just a [`u8`] with nibbles swapped.
/// That's why in a savefile they increment as follows:
///
/// - `0xDA`
/// - `0xEA`
/// - `0xFA`
/// - `0x0B`
/// - `0x1B`
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::Display)]
#[display("{_0:#x}")]
pub struct KeyU8(u8);

impl EncryptedU8 {
    pub const fn encrypted(encrypted: u8) -> Self {
        Self(encrypted)
    }

    pub const fn encrypt(raw: u8, key: KeyU8) -> Self {
        let encrypted = swap_nibbles(raw) ^ key.get();
        Self::encrypted(encrypted)
    }

    #[must_use]
    pub const fn decrypt(self, key: KeyU8) -> u8 {
        let Self(encrypted) = self;
        swap_nibbles(encrypted ^ key.get())
    }

    #[must_use]
    pub const fn get(self) -> u8 {
        let Self(encrypted) = self;
        encrypted
    }
}

impl KeyU8 {
    pub const fn new(key: u8) -> Self {
        Self(key)
    }

    #[must_use]
    pub const fn get(self) -> u8 {
        let Self(key) = self;
        key
    }

    pub const fn wrapping_add(self, rhs: u8) -> Self {
        let Self(swapped) = self;
        let normal = swap_nibbles(swapped);
        Self(swap_nibbles(normal.wrapping_add(rhs)))
    }

    #[allow(clippy::cast_possible_truncation)]
    pub const fn wrapping_add_usize(self, rhs: usize) -> Self {
        // Should be truncated properly here.
        let rhs: u8 = (rhs % 256) as u8;
        self.wrapping_add(rhs)
    }
}

/// Nibble is a half of an octet, which is 4 most/least significant bits.
/// In Hex `0x8A`, `8` is the highest nibble, and `A` is the lowest.
/// So, after swaping, the number will be `0xA8`.
const fn swap_nibbles(byte: u8) -> u8 {
    const HALF_BYTE: u32 = 4;
    byte.rotate_left(HALF_BYTE)
}

#[cfg(test)]
mod tests {
    use super::{EncryptedU8, KeyU8, swap_nibbles};

    #[rstest::rstest]
    #[case(0x8A, 0xA8)]
    #[case(0x00, 0x00)]
    #[case(0x12, 0x21)]
    fn nibbles_swap_properly(#[case] input: u8, #[case] expected: u8) {
        let actual = swap_nibbles(input);
        assert_eq!(expected, actual);
    }

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::zero_prefixed_literal)]
    #[rstest_reuse::apply(mapping)]
    fn decrypts(#[case] expected_raw: u8, #[case] encrypted: EncryptedU8, #[case] key: KeyU8) {
        let actual_raw = encrypted.decrypt(key);

        assert_eq!(expected_raw, actual_raw);
    }

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::zero_prefixed_literal)]
    #[rstest_reuse::apply(mapping)]
    fn encrypts(#[case] raw: u8, #[case] expected_encrypted: EncryptedU8, #[case] key: KeyU8) {
        let actual_encrypted = EncryptedU8::encrypt(raw, key);

        assert_eq!(expected_encrypted, actual_encrypted);
    }

    #[rstest::rstest]
    #[case(KeyU8::new(0x8A), 1, KeyU8::new(0x9A))]
    #[case(KeyU8::new(0x9A), 1, KeyU8::new(0xAA))]
    #[case(KeyU8::new(0xFA), 1, KeyU8::new(0x0B))]
    fn adds(#[case] key: KeyU8, #[case] to_add: u8, #[case] expected: KeyU8) {
        let actual = key.wrapping_add(to_add);
        assert_eq!(expected, actual);
    }

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::zero_prefixed_literal)]
    #[rstest_reuse::template]
    #[rstest::rstest]
    #[case(0xe8, EncryptedU8::encrypted(0x0), KeyU8::new(0x8e))]
    #[case(000, EncryptedU8::encrypted(0x8e), KeyU8::new(0x8e))]
    #[case(001, EncryptedU8::encrypted(0x9e), KeyU8::new(0x8e))]
    #[case(002, EncryptedU8::encrypted(0xae), KeyU8::new(0x8e))]
    #[case(003, EncryptedU8::encrypted(0xbe), KeyU8::new(0x8e))]
    #[case(004, EncryptedU8::encrypted(0xce), KeyU8::new(0x8e))]
    #[case(005, EncryptedU8::encrypted(0xde), KeyU8::new(0x8e))]
    #[case(006, EncryptedU8::encrypted(0xee), KeyU8::new(0x8e))]
    #[case(007, EncryptedU8::encrypted(0xfe), KeyU8::new(0x8e))]
    #[case(008, EncryptedU8::encrypted(0x0e), KeyU8::new(0x8e))]
    #[case(009, EncryptedU8::encrypted(0x1e), KeyU8::new(0x8e))]
    #[case(010, EncryptedU8::encrypted(0x2e), KeyU8::new(0x8e))]
    #[case(011, EncryptedU8::encrypted(0x3e), KeyU8::new(0x8e))]
    #[case(012, EncryptedU8::encrypted(0x4e), KeyU8::new(0x8e))]
    #[case(013, EncryptedU8::encrypted(0x5e), KeyU8::new(0x8e))]
    #[case(014, EncryptedU8::encrypted(0x6e), KeyU8::new(0x8e))]
    #[case(015, EncryptedU8::encrypted(0x7e), KeyU8::new(0x8e))]
    #[case(016, EncryptedU8::encrypted(0x8f), KeyU8::new(0x8e))]
    #[case(017, EncryptedU8::encrypted(0x9f), KeyU8::new(0x8e))]
    #[case(018, EncryptedU8::encrypted(0xaf), KeyU8::new(0x8e))]
    #[case(019, EncryptedU8::encrypted(0xbf), KeyU8::new(0x8e))]
    #[case(020, EncryptedU8::encrypted(0xcf), KeyU8::new(0x8e))]
    #[case(021, EncryptedU8::encrypted(0xdf), KeyU8::new(0x8e))]
    #[case(022, EncryptedU8::encrypted(0xef), KeyU8::new(0x8e))]
    #[case(023, EncryptedU8::encrypted(0xff), KeyU8::new(0x8e))]
    #[case(024, EncryptedU8::encrypted(0x0f), KeyU8::new(0x8e))]
    #[case(025, EncryptedU8::encrypted(0x1f), KeyU8::new(0x8e))]
    #[case(026, EncryptedU8::encrypted(0x2f), KeyU8::new(0x8e))]
    #[case(027, EncryptedU8::encrypted(0x3f), KeyU8::new(0x8e))]
    #[case(028, EncryptedU8::encrypted(0x4f), KeyU8::new(0x8e))]
    #[case(029, EncryptedU8::encrypted(0x5f), KeyU8::new(0x8e))]
    #[case(030, EncryptedU8::encrypted(0x6f), KeyU8::new(0x8e))]
    #[case(031, EncryptedU8::encrypted(0x7f), KeyU8::new(0x8e))]
    #[case(032, EncryptedU8::encrypted(0x8c), KeyU8::new(0x8e))]
    #[case(048, EncryptedU8::encrypted(0x8d), KeyU8::new(0x8e))]
    #[case(064, EncryptedU8::encrypted(0x8a), KeyU8::new(0x8e))]
    #[case(080, EncryptedU8::encrypted(0x8b), KeyU8::new(0x8e))]
    #[case(096, EncryptedU8::encrypted(0x88), KeyU8::new(0x8e))]
    #[case(112, EncryptedU8::encrypted(0x89), KeyU8::new(0x8e))]
    #[case(128, EncryptedU8::encrypted(0x86), KeyU8::new(0x8e))]
    #[case(144, EncryptedU8::encrypted(0x87), KeyU8::new(0x8e))]
    #[case(160, EncryptedU8::encrypted(0x84), KeyU8::new(0x8e))]
    #[case(176, EncryptedU8::encrypted(0x85), KeyU8::new(0x8e))]
    #[case(192, EncryptedU8::encrypted(0x82), KeyU8::new(0x8e))]
    #[case(208, EncryptedU8::encrypted(0x83), KeyU8::new(0x8e))]
    #[case(224, EncryptedU8::encrypted(0x80), KeyU8::new(0x8e))]
    #[case(240, EncryptedU8::encrypted(0x81), KeyU8::new(0x8e))]
    #[case((0000 >> 8) as u8, EncryptedU8::encrypted(0x9e), KeyU8::new(0x9e))]
    #[case((0256 >> 8) as u8, EncryptedU8::encrypted(0x8e), KeyU8::new(0x9e))]
    #[case((0512 >> 8) as u8, EncryptedU8::encrypted(0xbe), KeyU8::new(0x9e))]
    #[case((0768 >> 8) as u8, EncryptedU8::encrypted(0xae), KeyU8::new(0x9e))]
    #[case((1024 >> 8) as u8, EncryptedU8::encrypted(0xde), KeyU8::new(0x9e))]
    #[case((1280 >> 8) as u8, EncryptedU8::encrypted(0xce), KeyU8::new(0x9e))]
    #[case((1536 >> 8) as u8, EncryptedU8::encrypted(0xfe), KeyU8::new(0x9e))]
    #[case((1792 >> 8) as u8, EncryptedU8::encrypted(0xee), KeyU8::new(0x9e))]
    #[case((2048 >> 8) as u8, EncryptedU8::encrypted(0x1e), KeyU8::new(0x9e))]
    #[case((2304 >> 8) as u8, EncryptedU8::encrypted(0x0e), KeyU8::new(0x9e))]
    fn mapping(#[case] raw: u8, #[case] encrypted: EncryptedU8, #[case] key: KeyU8) {}
}
