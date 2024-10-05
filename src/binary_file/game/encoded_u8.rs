#[binrw::binrw]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[brw(little)]
pub struct EncodedU8<const KEY: u8>(u8);

impl<const KEY: u8> EncodedU8<KEY> {
    pub const fn pre_encoded(value: u8) -> Self {
        Self(value)
    }

    pub const fn encode_from_raw(raw: u8) -> Self {
        Self(swap_nibbles(raw) ^ KEY)
    }

    pub const fn decode(self) -> u8 {
        swap_nibbles(self.0 ^ KEY)
    }

    pub const fn get(&self) -> u8 {
        self.0
    }
}

const fn swap_nibbles(byte: u8) -> u8 {
    const HALF_BYTE: u32 = 4;
    byte.rotate_left(HALF_BYTE)
}

#[cfg(test)]
mod tests {
    use super::EncodedU8;

    #[rstest_reuse::apply(mapping)]
    fn encodes<const KEY: u8>(#[case] expected_raw: u8, #[case] encoded: EncodedU8<KEY>) {
        let actual_raw = encoded.decode();

        assert_eq!(expected_raw, actual_raw);
    }

    #[rstest_reuse::apply(mapping)]
    fn decodes<const KEY: u8>(#[case] raw: u8, #[case] expected_encoded: EncodedU8<KEY>) {
        let actual_encoded = EncodedU8::<KEY>::encode_from_raw(raw);

        assert_eq!(expected_encoded, actual_encoded);
    }

    #[rstest_reuse::template]
    #[rstest::rstest]
    #[case(000, EncodedU8::<0x8e>::pre_encoded(0x8e))]
    #[case(001, EncodedU8::<0x8e>::pre_encoded(0x9e))]
    #[case(002, EncodedU8::<0x8e>::pre_encoded(0xae))]
    #[case(003, EncodedU8::<0x8e>::pre_encoded(0xbe))]
    #[case(004, EncodedU8::<0x8e>::pre_encoded(0xce))]
    #[case(005, EncodedU8::<0x8e>::pre_encoded(0xde))]
    #[case(006, EncodedU8::<0x8e>::pre_encoded(0xee))]
    #[case(007, EncodedU8::<0x8e>::pre_encoded(0xfe))]
    #[case(008, EncodedU8::<0x8e>::pre_encoded(0x0e))]
    #[case(009, EncodedU8::<0x8e>::pre_encoded(0x1e))]
    #[case(010, EncodedU8::<0x8e>::pre_encoded(0x2e))]
    #[case(011, EncodedU8::<0x8e>::pre_encoded(0x3e))]
    #[case(012, EncodedU8::<0x8e>::pre_encoded(0x4e))]
    #[case(013, EncodedU8::<0x8e>::pre_encoded(0x5e))]
    #[case(014, EncodedU8::<0x8e>::pre_encoded(0x6e))]
    #[case(015, EncodedU8::<0x8e>::pre_encoded(0x7e))]
    #[case(016, EncodedU8::<0x8e>::pre_encoded(0x8f))]
    #[case(017, EncodedU8::<0x8e>::pre_encoded(0x9f))]
    #[case(018, EncodedU8::<0x8e>::pre_encoded(0xaf))]
    #[case(019, EncodedU8::<0x8e>::pre_encoded(0xbf))]
    #[case(020, EncodedU8::<0x8e>::pre_encoded(0xcf))]
    #[case(021, EncodedU8::<0x8e>::pre_encoded(0xdf))]
    #[case(022, EncodedU8::<0x8e>::pre_encoded(0xef))]
    #[case(023, EncodedU8::<0x8e>::pre_encoded(0xff))]
    #[case(024, EncodedU8::<0x8e>::pre_encoded(0x0f))]
    #[case(025, EncodedU8::<0x8e>::pre_encoded(0x1f))]
    #[case(026, EncodedU8::<0x8e>::pre_encoded(0x2f))]
    #[case(027, EncodedU8::<0x8e>::pre_encoded(0x3f))]
    #[case(028, EncodedU8::<0x8e>::pre_encoded(0x4f))]
    #[case(029, EncodedU8::<0x8e>::pre_encoded(0x5f))]
    #[case(030, EncodedU8::<0x8e>::pre_encoded(0x6f))]
    #[case(031, EncodedU8::<0x8e>::pre_encoded(0x7f))]
    #[case(032, EncodedU8::<0x8e>::pre_encoded(0x8c))]
    #[case(048, EncodedU8::<0x8e>::pre_encoded(0x8d))]
    #[case(064, EncodedU8::<0x8e>::pre_encoded(0x8a))]
    #[case(080, EncodedU8::<0x8e>::pre_encoded(0x8b))]
    #[case(096, EncodedU8::<0x8e>::pre_encoded(0x88))]
    #[case(112, EncodedU8::<0x8e>::pre_encoded(0x89))]
    #[case(128, EncodedU8::<0x8e>::pre_encoded(0x86))]
    #[case(144, EncodedU8::<0x8e>::pre_encoded(0x87))]
    #[case(160, EncodedU8::<0x8e>::pre_encoded(0x84))]
    #[case(176, EncodedU8::<0x8e>::pre_encoded(0x85))]
    #[case(192, EncodedU8::<0x8e>::pre_encoded(0x82))]
    #[case(208, EncodedU8::<0x8e>::pre_encoded(0x83))]
    #[case(224, EncodedU8::<0x8e>::pre_encoded(0x80))]
    #[case(240, EncodedU8::<0x8e>::pre_encoded(0x81))]
    #[case((0000 >> 8) as u8, EncodedU8::<0x9e>::pre_encoded(0x9e))]
    #[case((0256 >> 8) as u8, EncodedU8::<0x9e>::pre_encoded(0x8e))]
    #[case((0512 >> 8) as u8, EncodedU8::<0x9e>::pre_encoded(0xbe))]
    #[case((0768 >> 8) as u8, EncodedU8::<0x9e>::pre_encoded(0xae))]
    #[case((1024 >> 8) as u8, EncodedU8::<0x9e>::pre_encoded(0xde))]
    #[case((1280 >> 8) as u8, EncodedU8::<0x9e>::pre_encoded(0xce))]
    #[case((1536 >> 8) as u8, EncodedU8::<0x9e>::pre_encoded(0xfe))]
    #[case((1792 >> 8) as u8, EncodedU8::<0x9e>::pre_encoded(0xee))]
    #[case((2048 >> 8) as u8, EncodedU8::<0x9e>::pre_encoded(0x1e))]
    #[case((2304 >> 8) as u8, EncodedU8::<0x9e>::pre_encoded(0x0e))]
    fn mapping<const KEY: u8>(#[case] raw: u8, #[case] encoded: EncodedU8<KEY>) {}
}
