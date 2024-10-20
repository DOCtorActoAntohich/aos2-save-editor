#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EncodedU8 {
    encoded: u8,
    key: u8,
}

impl EncodedU8 {
    pub const fn pre_encoded(value: u8, key: u8) -> Self {
        Self {
            encoded: value,
            key,
        }
    }

    pub const fn from_raw(raw: u8, key: u8) -> Self {
        Self {
            encoded: swap_nibbles(raw) ^ key,
            key,
        }
    }

    pub const fn decode(self) -> u8 {
        let Self { encoded, key } = self;
        swap_nibbles(encoded ^ key)
    }

    pub const fn get(&self) -> u8 {
        self.encoded
    }

    pub const fn key(&self) -> u8 {
        self.key
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
    fn decodes(#[case] expected_raw: u8, #[case] encoded: EncodedU8) {
        let actual_raw = encoded.decode();

        assert_eq!(expected_raw, actual_raw);
    }

    #[rstest_reuse::apply(mapping)]
    fn encodes(#[case] raw: u8, #[case] expected_encoded: EncodedU8) {
        let actual_encoded = EncodedU8::from_raw(raw, expected_encoded.key());

        assert_eq!(expected_encoded, actual_encoded);
    }

    #[rstest_reuse::template]
    #[rstest::rstest]
    #[case(000, EncodedU8::pre_encoded(0x8e, 0x8e))]
    #[case(001, EncodedU8::pre_encoded(0x9e, 0x8e))]
    #[case(002, EncodedU8::pre_encoded(0xae, 0x8e))]
    #[case(003, EncodedU8::pre_encoded(0xbe, 0x8e))]
    #[case(004, EncodedU8::pre_encoded(0xce, 0x8e))]
    #[case(005, EncodedU8::pre_encoded(0xde, 0x8e))]
    #[case(006, EncodedU8::pre_encoded(0xee, 0x8e))]
    #[case(007, EncodedU8::pre_encoded(0xfe, 0x8e))]
    #[case(008, EncodedU8::pre_encoded(0x0e, 0x8e))]
    #[case(009, EncodedU8::pre_encoded(0x1e, 0x8e))]
    #[case(010, EncodedU8::pre_encoded(0x2e, 0x8e))]
    #[case(011, EncodedU8::pre_encoded(0x3e, 0x8e))]
    #[case(012, EncodedU8::pre_encoded(0x4e, 0x8e))]
    #[case(013, EncodedU8::pre_encoded(0x5e, 0x8e))]
    #[case(014, EncodedU8::pre_encoded(0x6e, 0x8e))]
    #[case(015, EncodedU8::pre_encoded(0x7e, 0x8e))]
    #[case(016, EncodedU8::pre_encoded(0x8f, 0x8e))]
    #[case(017, EncodedU8::pre_encoded(0x9f, 0x8e))]
    #[case(018, EncodedU8::pre_encoded(0xaf, 0x8e))]
    #[case(019, EncodedU8::pre_encoded(0xbf, 0x8e))]
    #[case(020, EncodedU8::pre_encoded(0xcf, 0x8e))]
    #[case(021, EncodedU8::pre_encoded(0xdf, 0x8e))]
    #[case(022, EncodedU8::pre_encoded(0xef, 0x8e))]
    #[case(023, EncodedU8::pre_encoded(0xff, 0x8e))]
    #[case(024, EncodedU8::pre_encoded(0x0f, 0x8e))]
    #[case(025, EncodedU8::pre_encoded(0x1f, 0x8e))]
    #[case(026, EncodedU8::pre_encoded(0x2f, 0x8e))]
    #[case(027, EncodedU8::pre_encoded(0x3f, 0x8e))]
    #[case(028, EncodedU8::pre_encoded(0x4f, 0x8e))]
    #[case(029, EncodedU8::pre_encoded(0x5f, 0x8e))]
    #[case(030, EncodedU8::pre_encoded(0x6f, 0x8e))]
    #[case(031, EncodedU8::pre_encoded(0x7f, 0x8e))]
    #[case(032, EncodedU8::pre_encoded(0x8c, 0x8e))]
    #[case(048, EncodedU8::pre_encoded(0x8d, 0x8e))]
    #[case(064, EncodedU8::pre_encoded(0x8a, 0x8e))]
    #[case(080, EncodedU8::pre_encoded(0x8b, 0x8e))]
    #[case(096, EncodedU8::pre_encoded(0x88, 0x8e))]
    #[case(112, EncodedU8::pre_encoded(0x89, 0x8e))]
    #[case(128, EncodedU8::pre_encoded(0x86, 0x8e))]
    #[case(144, EncodedU8::pre_encoded(0x87, 0x8e))]
    #[case(160, EncodedU8::pre_encoded(0x84, 0x8e))]
    #[case(176, EncodedU8::pre_encoded(0x85, 0x8e))]
    #[case(192, EncodedU8::pre_encoded(0x82, 0x8e))]
    #[case(208, EncodedU8::pre_encoded(0x83, 0x8e))]
    #[case(224, EncodedU8::pre_encoded(0x80, 0x8e))]
    #[case(240, EncodedU8::pre_encoded(0x81, 0x8e))]
    #[case((0000 >> 8) as u8, EncodedU8::pre_encoded(0x9e, 0x9e))]
    #[case((0256 >> 8) as u8, EncodedU8::pre_encoded(0x8e, 0x9e))]
    #[case((0512 >> 8) as u8, EncodedU8::pre_encoded(0xbe, 0x9e))]
    #[case((0768 >> 8) as u8, EncodedU8::pre_encoded(0xae, 0x9e))]
    #[case((1024 >> 8) as u8, EncodedU8::pre_encoded(0xde, 0x9e))]
    #[case((1280 >> 8) as u8, EncodedU8::pre_encoded(0xce, 0x9e))]
    #[case((1536 >> 8) as u8, EncodedU8::pre_encoded(0xfe, 0x9e))]
    #[case((1792 >> 8) as u8, EncodedU8::pre_encoded(0xee, 0x9e))]
    #[case((2048 >> 8) as u8, EncodedU8::pre_encoded(0x1e, 0x9e))]
    #[case((2304 >> 8) as u8, EncodedU8::pre_encoded(0x0e, 0x9e))]
    fn mapping(#[case] raw: u8, #[case] encoded: EncodedU8) {}
}
