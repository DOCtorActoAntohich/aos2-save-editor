#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EncodedByte {
    encoded_value: u8,
    key: u8,
}

impl EncodedByte {
    pub const fn encode_with_key(value: u8, key: u8) -> Self {
        Self {
            encoded_value: swap_nibbles(value) ^ key,
            key,
        }
    }

    pub const fn decode(self) -> u8 {
        swap_nibbles(self.encoded_value ^ self.key)
    }
}

const fn swap_nibbles(byte: u8) -> u8 {
    const HALF_BYTE: u32 = 4;
    byte.rotate_left(HALF_BYTE)
}

#[cfg(test)]
mod tests {
    use super::EncodedByte;

    #[rstest_reuse::apply(number_mapping)]
    fn text_stuff(#[case] expected_decoded: u8, #[case] encoded: u8, #[case] key: u8) {
        let encoded_byte = EncodedByte {
            encoded_value: encoded,
            key,
        };

        let actual_decoded = encoded_byte.decode();

        assert_eq!(expected_decoded, actual_decoded);
    }

    #[rstest_reuse::template]
    #[rstest::rstest]
    #[case::suguri_wins_lowest_byte_x8e(000, 0x8e, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(001, 0x9e, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(002, 0xae, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(003, 0xbe, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(004, 0xce, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(005, 0xde, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(006, 0xee, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(007, 0xfe, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(008, 0x0e, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(009, 0x1e, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(010, 0x2e, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(011, 0x3e, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(012, 0x4e, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(013, 0x5e, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(014, 0x6e, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(015, 0x7e, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(016, 0x8f, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(017, 0x9f, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(018, 0xaf, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(019, 0xbf, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(020, 0xcf, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(021, 0xdf, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(022, 0xef, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(023, 0xff, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(024, 0x0f, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(025, 0x1f, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(026, 0x2f, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(027, 0x3f, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(028, 0x4f, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(029, 0x5f, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(030, 0x6f, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(031, 0x7f, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(032, 0x8c, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(048, 0x8d, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(064, 0x8a, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(080, 0x8b, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(096, 0x88, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(112, 0x89, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(128, 0x86, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(144, 0x87, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(160, 0x84, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(176, 0x85, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(192, 0x82, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(208, 0x83, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(224, 0x80, 0x8e)]
    #[case::suguri_wins_lowest_byte_x8e(240, 0x81, 0x8e)]
    #[case::suguri_wins_second_lowest_byte_0x9e((0000 >> 8) as u8, 0x9e, 0x9e)]
    #[case::suguri_wins_second_lowest_byte_0x9e((0256 >> 8) as u8, 0x8e, 0x9e)]
    #[case::suguri_wins_second_lowest_byte_0x9e((0512 >> 8) as u8, 0xbe, 0x9e)]
    #[case::suguri_wins_second_lowest_byte_0x9e((0768 >> 8) as u8, 0xae, 0x9e)]
    #[case::suguri_wins_second_lowest_byte_0x9e((1024 >> 8) as u8, 0xde, 0x9e)]
    #[case::suguri_wins_second_lowest_byte_0x9e((1280 >> 8) as u8, 0xce, 0x9e)]
    #[case::suguri_wins_second_lowest_byte_0x9e((1536 >> 8) as u8, 0xfe, 0x9e)]
    #[case::suguri_wins_second_lowest_byte_0x9e((1792 >> 8) as u8, 0xee, 0x9e)]
    #[case::suguri_wins_second_lowest_byte_0x9e((2048 >> 8) as u8, 0x1e, 0x9e)]
    #[case::suguri_wins_second_lowest_byte_0x9e((2304 >> 8) as u8, 0x0e, 0x9e)]
    fn number_mapping(#[case] unencoded: u8, #[case] encoded: u8, #[case] key: u8) {}
}
