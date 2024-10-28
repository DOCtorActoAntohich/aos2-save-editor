use super::swap_nibbles;

#[binrw::binrw]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[brw(little)]
pub struct EncodedU8(u8);

/// A xor "encryption" key for bytes in `game.sys`.
///
/// Values are typical `u8`s with nibbles swapped.
/// That's why in a savefile they increment as follows:
/// - 0xDA
/// - 0xEA
/// - 0xFA
/// - 0x0B
/// - 0x1B
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::Display)]
#[display("{_0:#x}")]
pub struct KeyU8(u8);

impl EncodedU8 {
    pub const fn pre_encoded(value: u8) -> Self {
        Self(value)
    }

    pub const fn from_raw(raw: u8, key: KeyU8) -> Self {
        Self(swap_nibbles(raw) ^ key.get())
    }

    pub const fn decode(self, key: KeyU8) -> u8 {
        let Self(encoded) = self;
        swap_nibbles(encoded ^ key.get())
    }

    pub const fn get(&self) -> u8 {
        self.0
    }
}

impl KeyU8 {
    pub const fn new(key: u8) -> Self {
        Self(key)
    }

    pub const fn get(self) -> u8 {
        self.0
    }

    pub const fn increment(self) -> Self {
        self.wrapping_add(1)
    }

    pub const fn wrapping_add(self, rhs: u8) -> Self {
        let Self(swapped) = self;
        let normal = swap_nibbles(swapped);
        Self(swap_nibbles(normal.wrapping_add(rhs)))
    }
}

#[cfg(test)]
mod tests {
    use super::{EncodedU8, KeyU8};

    #[rstest_reuse::apply(mapping)]
    fn decodes(#[case] expected_raw: u8, #[case] encoded: EncodedU8, #[case] key: KeyU8) {
        let actual_raw = encoded.decode(key);

        assert_eq!(expected_raw, actual_raw);
    }

    #[rstest_reuse::apply(mapping)]
    fn encodes(#[case] raw: u8, #[case] expected_encoded: EncodedU8, #[case] key: KeyU8) {
        let actual_encoded = EncodedU8::from_raw(raw, key);

        assert_eq!(expected_encoded, actual_encoded);
    }

    #[rstest::rstest]
    #[case(KeyU8::new(0x8A), 1, KeyU8::new(0x9A))]
    #[case(KeyU8::new(0x9A), 1, KeyU8::new(0xAA))]
    #[case(KeyU8::new(0xFA), 1, KeyU8::new(0x0B))]
    fn adds(#[case] key: KeyU8, #[case] to_add: u8, #[case] expected: KeyU8) {
        let actual = key.wrapping_add(to_add);
        assert_eq!(expected, actual);
    }

    #[rstest_reuse::template]
    #[rstest::rstest]
    #[case(0xe8, EncodedU8::pre_encoded(0x0), KeyU8::new(0x8e))]
    #[case(000, EncodedU8::pre_encoded(0x8e), KeyU8::new(0x8e))]
    #[case(001, EncodedU8::pre_encoded(0x9e), KeyU8::new(0x8e))]
    #[case(002, EncodedU8::pre_encoded(0xae), KeyU8::new(0x8e))]
    #[case(003, EncodedU8::pre_encoded(0xbe), KeyU8::new(0x8e))]
    #[case(004, EncodedU8::pre_encoded(0xce), KeyU8::new(0x8e))]
    #[case(005, EncodedU8::pre_encoded(0xde), KeyU8::new(0x8e))]
    #[case(006, EncodedU8::pre_encoded(0xee), KeyU8::new(0x8e))]
    #[case(007, EncodedU8::pre_encoded(0xfe), KeyU8::new(0x8e))]
    #[case(008, EncodedU8::pre_encoded(0x0e), KeyU8::new(0x8e))]
    #[case(009, EncodedU8::pre_encoded(0x1e), KeyU8::new(0x8e))]
    #[case(010, EncodedU8::pre_encoded(0x2e), KeyU8::new(0x8e))]
    #[case(011, EncodedU8::pre_encoded(0x3e), KeyU8::new(0x8e))]
    #[case(012, EncodedU8::pre_encoded(0x4e), KeyU8::new(0x8e))]
    #[case(013, EncodedU8::pre_encoded(0x5e), KeyU8::new(0x8e))]
    #[case(014, EncodedU8::pre_encoded(0x6e), KeyU8::new(0x8e))]
    #[case(015, EncodedU8::pre_encoded(0x7e), KeyU8::new(0x8e))]
    #[case(016, EncodedU8::pre_encoded(0x8f), KeyU8::new(0x8e))]
    #[case(017, EncodedU8::pre_encoded(0x9f), KeyU8::new(0x8e))]
    #[case(018, EncodedU8::pre_encoded(0xaf), KeyU8::new(0x8e))]
    #[case(019, EncodedU8::pre_encoded(0xbf), KeyU8::new(0x8e))]
    #[case(020, EncodedU8::pre_encoded(0xcf), KeyU8::new(0x8e))]
    #[case(021, EncodedU8::pre_encoded(0xdf), KeyU8::new(0x8e))]
    #[case(022, EncodedU8::pre_encoded(0xef), KeyU8::new(0x8e))]
    #[case(023, EncodedU8::pre_encoded(0xff), KeyU8::new(0x8e))]
    #[case(024, EncodedU8::pre_encoded(0x0f), KeyU8::new(0x8e))]
    #[case(025, EncodedU8::pre_encoded(0x1f), KeyU8::new(0x8e))]
    #[case(026, EncodedU8::pre_encoded(0x2f), KeyU8::new(0x8e))]
    #[case(027, EncodedU8::pre_encoded(0x3f), KeyU8::new(0x8e))]
    #[case(028, EncodedU8::pre_encoded(0x4f), KeyU8::new(0x8e))]
    #[case(029, EncodedU8::pre_encoded(0x5f), KeyU8::new(0x8e))]
    #[case(030, EncodedU8::pre_encoded(0x6f), KeyU8::new(0x8e))]
    #[case(031, EncodedU8::pre_encoded(0x7f), KeyU8::new(0x8e))]
    #[case(032, EncodedU8::pre_encoded(0x8c), KeyU8::new(0x8e))]
    #[case(048, EncodedU8::pre_encoded(0x8d), KeyU8::new(0x8e))]
    #[case(064, EncodedU8::pre_encoded(0x8a), KeyU8::new(0x8e))]
    #[case(080, EncodedU8::pre_encoded(0x8b), KeyU8::new(0x8e))]
    #[case(096, EncodedU8::pre_encoded(0x88), KeyU8::new(0x8e))]
    #[case(112, EncodedU8::pre_encoded(0x89), KeyU8::new(0x8e))]
    #[case(128, EncodedU8::pre_encoded(0x86), KeyU8::new(0x8e))]
    #[case(144, EncodedU8::pre_encoded(0x87), KeyU8::new(0x8e))]
    #[case(160, EncodedU8::pre_encoded(0x84), KeyU8::new(0x8e))]
    #[case(176, EncodedU8::pre_encoded(0x85), KeyU8::new(0x8e))]
    #[case(192, EncodedU8::pre_encoded(0x82), KeyU8::new(0x8e))]
    #[case(208, EncodedU8::pre_encoded(0x83), KeyU8::new(0x8e))]
    #[case(224, EncodedU8::pre_encoded(0x80), KeyU8::new(0x8e))]
    #[case(240, EncodedU8::pre_encoded(0x81), KeyU8::new(0x8e))]
    #[case((0000 >> 8) as u8, EncodedU8::pre_encoded(0x9e), KeyU8::new(0x9e))]
    #[case((0256 >> 8) as u8, EncodedU8::pre_encoded(0x8e), KeyU8::new(0x9e))]
    #[case((0512 >> 8) as u8, EncodedU8::pre_encoded(0xbe), KeyU8::new(0x9e))]
    #[case((0768 >> 8) as u8, EncodedU8::pre_encoded(0xae), KeyU8::new(0x9e))]
    #[case((1024 >> 8) as u8, EncodedU8::pre_encoded(0xde), KeyU8::new(0x9e))]
    #[case((1280 >> 8) as u8, EncodedU8::pre_encoded(0xce), KeyU8::new(0x9e))]
    #[case((1536 >> 8) as u8, EncodedU8::pre_encoded(0xfe), KeyU8::new(0x9e))]
    #[case((1792 >> 8) as u8, EncodedU8::pre_encoded(0xee), KeyU8::new(0x9e))]
    #[case((2048 >> 8) as u8, EncodedU8::pre_encoded(0x1e), KeyU8::new(0x9e))]
    #[case((2304 >> 8) as u8, EncodedU8::pre_encoded(0x0e), KeyU8::new(0x9e))]
    fn mapping(#[case] raw: u8, #[case] encoded: EncodedU8, #[case] key: KeyU8) {}
}
