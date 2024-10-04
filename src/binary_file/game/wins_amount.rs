const HALF_BYTE: u8 = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct EncodedValue(u8);

impl From<u8> for EncodedValue {
    fn from(value: u8) -> Self {
        let ahi1 = 0b10000000 & (!(value << HALF_BYTE));
        let ahi3 = 0b01110000 & (value << HALF_BYTE);
        let alo3 = 0b00001110 & !(value >> HALF_BYTE);
        let alo1 = 0b00000001 & (value >> HALF_BYTE);

        EncodedValue(ahi1 | ahi3 | alo3 | alo1)
    }
}

impl From<EncodedValue> for u8 {
    fn from(EncodedValue(value): EncodedValue) -> Self {
        let hi3 = 0b11100000 & ((!value) << HALF_BYTE);
        let hi1 = 0b00010000 & (value << HALF_BYTE);
        let lo1 = 0b00001000 & ((!value) >> HALF_BYTE);
        let lo3 = 0b00000111 & (value >> HALF_BYTE);

        hi3 | hi1 | lo1 | lo3
    }
}

#[cfg(test)]
mod tests {
    use super::EncodedValue;

    #[rstest_reuse::apply(number_mapping)]
    fn single_byte_encodes(#[case] input: u8, #[case] expected_value: EncodedValue) {
        let actual_value = EncodedValue::from(input);

        assert_eq!(expected_value, actual_value);
    }

    #[rstest_reuse::apply(number_mapping)]
    fn single_byte_decodes(#[case] expected_value: u8, #[case] input: EncodedValue) {
        let actual_value: u8 = input.into();

        assert_eq!(expected_value, actual_value);
    }

    #[rstest_reuse::template]
    #[rstest::rstest]
    #[case(000, EncodedValue(0x8e))]
    #[case(001, EncodedValue(0x9e))]
    #[case(002, EncodedValue(0xae))]
    #[case(003, EncodedValue(0xbe))]
    #[case(004, EncodedValue(0xce))]
    #[case(005, EncodedValue(0xde))]
    #[case(006, EncodedValue(0xee))]
    #[case(007, EncodedValue(0xfe))]
    #[case(008, EncodedValue(0x0e))]
    #[case(009, EncodedValue(0x1e))]
    #[case(010, EncodedValue(0x2e))]
    #[case(011, EncodedValue(0x3e))]
    #[case(012, EncodedValue(0x4e))]
    #[case(013, EncodedValue(0x5e))]
    #[case(014, EncodedValue(0x6e))]
    #[case(015, EncodedValue(0x7e))]
    #[case(016, EncodedValue(0x8f))]
    #[case(017, EncodedValue(0x9f))]
    #[case(018, EncodedValue(0xaf))]
    #[case(019, EncodedValue(0xbf))]
    #[case(020, EncodedValue(0xcf))]
    #[case(021, EncodedValue(0xdf))]
    #[case(022, EncodedValue(0xef))]
    #[case(023, EncodedValue(0xff))]
    #[case(024, EncodedValue(0x0f))]
    #[case(025, EncodedValue(0x1f))]
    #[case(026, EncodedValue(0x2f))]
    #[case(027, EncodedValue(0x3f))]
    #[case(028, EncodedValue(0x4f))]
    #[case(029, EncodedValue(0x5f))]
    #[case(030, EncodedValue(0x6f))]
    #[case(031, EncodedValue(0x7f))]
    #[case(032, EncodedValue(0x8c))]
    #[case(048, EncodedValue(0x8d))]
    #[case(064, EncodedValue(0x8a))]
    #[case(080, EncodedValue(0x8b))]
    #[case(096, EncodedValue(0x88))]
    #[case(112, EncodedValue(0x89))]
    #[case(128, EncodedValue(0x86))]
    #[case(144, EncodedValue(0x87))]
    #[case(160, EncodedValue(0x84))]
    #[case(176, EncodedValue(0x85))]
    #[case(192, EncodedValue(0x82))]
    #[case(208, EncodedValue(0x83))]
    #[case(224, EncodedValue(0x80))]
    #[case(240, EncodedValue(0x81))]
    fn number_mapping(#[case] raw: u8, #[case] encoded: EncodedValue) {}
}
