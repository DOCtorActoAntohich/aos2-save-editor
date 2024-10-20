use super::{key::Key, swap_nibbles};

#[binrw::binrw]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[brw(little)]
pub struct EncodedU8(u8);

impl EncodedU8 {
    pub const fn pre_encoded(value: u8) -> Self {
        Self(value)
    }

    pub const fn from_raw(raw: u8, key: Key) -> Self {
        Self(swap_nibbles(raw) ^ key.get())
    }

    pub const fn decode(self, key: Key) -> u8 {
        let Self(encoded) = self;
        swap_nibbles(encoded ^ key.get())
    }

    pub const fn get(&self) -> u8 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::xor_encoding::key::Key;

    use super::EncodedU8;

    #[rstest_reuse::apply(mapping)]
    fn decodes(#[case] expected_raw: u8, #[case] encoded: EncodedU8, #[case] key: Key) {
        let actual_raw = encoded.decode(key);

        assert_eq!(expected_raw, actual_raw);
    }

    #[rstest_reuse::apply(mapping)]
    fn encodes(#[case] raw: u8, #[case] expected_encoded: EncodedU8, #[case] key: Key) {
        let actual_encoded = EncodedU8::from_raw(raw, key);

        assert_eq!(expected_encoded, actual_encoded);
    }

    #[rstest_reuse::template]
    #[rstest::rstest]
    #[case(000, EncodedU8::pre_encoded(0x8e), Key::new(0x8e))]
    #[case(001, EncodedU8::pre_encoded(0x9e), Key::new(0x8e))]
    #[case(002, EncodedU8::pre_encoded(0xae), Key::new(0x8e))]
    #[case(003, EncodedU8::pre_encoded(0xbe), Key::new(0x8e))]
    #[case(004, EncodedU8::pre_encoded(0xce), Key::new(0x8e))]
    #[case(005, EncodedU8::pre_encoded(0xde), Key::new(0x8e))]
    #[case(006, EncodedU8::pre_encoded(0xee), Key::new(0x8e))]
    #[case(007, EncodedU8::pre_encoded(0xfe), Key::new(0x8e))]
    #[case(008, EncodedU8::pre_encoded(0x0e), Key::new(0x8e))]
    #[case(009, EncodedU8::pre_encoded(0x1e), Key::new(0x8e))]
    #[case(010, EncodedU8::pre_encoded(0x2e), Key::new(0x8e))]
    #[case(011, EncodedU8::pre_encoded(0x3e), Key::new(0x8e))]
    #[case(012, EncodedU8::pre_encoded(0x4e), Key::new(0x8e))]
    #[case(013, EncodedU8::pre_encoded(0x5e), Key::new(0x8e))]
    #[case(014, EncodedU8::pre_encoded(0x6e), Key::new(0x8e))]
    #[case(015, EncodedU8::pre_encoded(0x7e), Key::new(0x8e))]
    #[case(016, EncodedU8::pre_encoded(0x8f), Key::new(0x8e))]
    #[case(017, EncodedU8::pre_encoded(0x9f), Key::new(0x8e))]
    #[case(018, EncodedU8::pre_encoded(0xaf), Key::new(0x8e))]
    #[case(019, EncodedU8::pre_encoded(0xbf), Key::new(0x8e))]
    #[case(020, EncodedU8::pre_encoded(0xcf), Key::new(0x8e))]
    #[case(021, EncodedU8::pre_encoded(0xdf), Key::new(0x8e))]
    #[case(022, EncodedU8::pre_encoded(0xef), Key::new(0x8e))]
    #[case(023, EncodedU8::pre_encoded(0xff), Key::new(0x8e))]
    #[case(024, EncodedU8::pre_encoded(0x0f), Key::new(0x8e))]
    #[case(025, EncodedU8::pre_encoded(0x1f), Key::new(0x8e))]
    #[case(026, EncodedU8::pre_encoded(0x2f), Key::new(0x8e))]
    #[case(027, EncodedU8::pre_encoded(0x3f), Key::new(0x8e))]
    #[case(028, EncodedU8::pre_encoded(0x4f), Key::new(0x8e))]
    #[case(029, EncodedU8::pre_encoded(0x5f), Key::new(0x8e))]
    #[case(030, EncodedU8::pre_encoded(0x6f), Key::new(0x8e))]
    #[case(031, EncodedU8::pre_encoded(0x7f), Key::new(0x8e))]
    #[case(032, EncodedU8::pre_encoded(0x8c), Key::new(0x8e))]
    #[case(048, EncodedU8::pre_encoded(0x8d), Key::new(0x8e))]
    #[case(064, EncodedU8::pre_encoded(0x8a), Key::new(0x8e))]
    #[case(080, EncodedU8::pre_encoded(0x8b), Key::new(0x8e))]
    #[case(096, EncodedU8::pre_encoded(0x88), Key::new(0x8e))]
    #[case(112, EncodedU8::pre_encoded(0x89), Key::new(0x8e))]
    #[case(128, EncodedU8::pre_encoded(0x86), Key::new(0x8e))]
    #[case(144, EncodedU8::pre_encoded(0x87), Key::new(0x8e))]
    #[case(160, EncodedU8::pre_encoded(0x84), Key::new(0x8e))]
    #[case(176, EncodedU8::pre_encoded(0x85), Key::new(0x8e))]
    #[case(192, EncodedU8::pre_encoded(0x82), Key::new(0x8e))]
    #[case(208, EncodedU8::pre_encoded(0x83), Key::new(0x8e))]
    #[case(224, EncodedU8::pre_encoded(0x80), Key::new(0x8e))]
    #[case(240, EncodedU8::pre_encoded(0x81), Key::new(0x8e))]
    #[case((0000 >> 8) as u8, EncodedU8::pre_encoded(0x9e), Key::new(0x9e))]
    #[case((0256 >> 8) as u8, EncodedU8::pre_encoded(0x8e), Key::new(0x9e))]
    #[case((0512 >> 8) as u8, EncodedU8::pre_encoded(0xbe), Key::new(0x9e))]
    #[case((0768 >> 8) as u8, EncodedU8::pre_encoded(0xae), Key::new(0x9e))]
    #[case((1024 >> 8) as u8, EncodedU8::pre_encoded(0xde), Key::new(0x9e))]
    #[case((1280 >> 8) as u8, EncodedU8::pre_encoded(0xce), Key::new(0x9e))]
    #[case((1536 >> 8) as u8, EncodedU8::pre_encoded(0xfe), Key::new(0x9e))]
    #[case((1792 >> 8) as u8, EncodedU8::pre_encoded(0xee), Key::new(0x9e))]
    #[case((2048 >> 8) as u8, EncodedU8::pre_encoded(0x1e), Key::new(0x9e))]
    #[case((2304 >> 8) as u8, EncodedU8::pre_encoded(0x0e), Key::new(0x9e))]
    fn mapping(#[case] raw: u8, #[case] encoded: EncodedU8, #[case] key: Key) {}
}
