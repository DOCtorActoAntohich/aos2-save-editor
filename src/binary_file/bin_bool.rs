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

    #[rstest::rstest]
    #[case::false_value(b"\x00", false.into())]
    #[case::true_value(b"\x01", true.into())]
    fn encodes_properly(#[case] expected_buffer_content: &[u8], #[case] value: BinBool) {
        let mut writer = Cursor::new(Vec::new());
        value.write(&mut writer).expect("Must write here");

        let actual_buffer_content = &writer.into_inner()[..];
        assert_eq!(expected_buffer_content, actual_buffer_content);
    }

    #[rstest::rstest]
    #[case::false_value(b"\x00", false)]
    #[case::true_value(b"\x01", true)]
    fn decodes_properly(#[case] input: &[u8], #[case] expected_value: bool) {
        let mut reader = Cursor::new(input.to_vec());
        let actual_value = BinBool::read(&mut reader).expect("Must read here");

        assert_eq!(expected_value, actual_value.into());
    }
}
