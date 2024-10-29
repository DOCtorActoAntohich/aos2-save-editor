#[binrw::binrw]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[brw(little)]
pub struct BinBool(u8);

impl BinBool {
    pub const fn new(value: bool) -> Self {
        match value {
            true => Self(1),
            false => Self(0),
        }
    }

    pub const fn as_u8(&self) -> u8 {
        self.0
    }
}

impl From<bool> for BinBool {
    fn from(value: bool) -> Self {
        match value {
            true => BinBool(1),
            false => BinBool(0),
        }
    }
}

impl From<&bool> for BinBool {
    fn from(&value: &bool) -> Self {
        Self::from(value)
    }
}

impl From<BinBool> for bool {
    fn from(value: BinBool) -> Self {
        match value {
            BinBool(0) => false,
            BinBool(_) => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use binrw::{BinRead, BinWrite};

    use super::BinBool;

    #[rstest_reuse::apply(raw_bin_bool)]
    fn encodes_properly(#[case] expected_buffer_content: &[u8], #[case] value: BinBool) {
        let mut writer = Cursor::new(Vec::new());
        value.write(&mut writer).expect("Must write here");

        let actual_buffer_content = &writer.into_inner()[..];
        assert_eq!(expected_buffer_content, actual_buffer_content);
    }

    #[rstest_reuse::apply(raw_bin_bool)]
    fn decodes_properly(#[case] input: &[u8], #[case] expected_value: bool) {
        let mut reader = Cursor::new(input.to_vec());
        let actual_value = BinBool::read(&mut reader).expect("Must read here");

        assert_eq!(expected_value, actual_value.into());
    }

    #[binrw::binrw]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[brw(little)]
    struct Status {
        #[br(map = From::<BinBool>::from)]
        #[bw(map = BinBool::from)]
        is_enabled: bool,
    }

    #[rstest_reuse::apply(struct_with_bool_field)]
    fn struct_with_bool_field_encodes(#[case] expected_raw: u8, #[case] input: Status) {
        let mut writer = binrw::io::Cursor::new([123]);
        BinWrite::write(&input, &mut writer).expect("Struct must encode here");
        let [actual_raw] = writer.into_inner();

        assert_eq!(expected_raw, actual_raw);
    }

    #[rstest_reuse::apply(struct_with_bool_field)]
    fn struct_with_bool_field_decodes(#[case] input: u8, #[case] expected_struct: Status) {
        let mut reader = binrw::io::Cursor::new([input]);
        let actual_struct: Status = BinRead::read(&mut reader).expect("Struct must decode here");

        assert_eq!(expected_struct, actual_struct);
    }

    #[rstest_reuse::template]
    #[rstest::rstest]
    #[case::false_value(b"\x00", false.into())]
    #[case::true_value(b"\x01", true.into())]
    fn raw_bin_bool(#[case] expected_buffer_content: &[u8], #[case] value: BinBool) {}

    #[rstest_reuse::template]
    #[rstest::rstest]
    #[case(0x00, Status { is_enabled: false })]
    #[case(0x01, Status { is_enabled: true })]
    fn struct_with_bool_field(#[case] raw: u8, #[case] status: Status) {}
}
