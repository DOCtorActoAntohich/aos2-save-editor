use binrw::{BinRead, BinWrite};

#[derive(Debug, Clone, PartialEq, Eq, Hash, BinRead, BinWrite, derive_more::Display)]
#[brw(magic = b"\xA1\x05\x00\x00")]
#[display("0xA1_05_00_00")]
pub struct Version;

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use binrw::{BinRead, BinWrite};

    use crate::player::v_a1_05_00_00::version::Version;

    #[rstest::rstest]
    fn version_encodes_properly() {
        let expected_version = b"\xA1\x05\x00\x00";

        let mut writer = Cursor::new(Vec::new());
        Version.write(&mut writer).expect("Must write here");

        let actual_version = &writer.into_inner()[..];
        assert_eq!(expected_version, actual_version);
    }

    #[rstest::rstest]
    fn version_decodes_properly() {
        let expected_version = Version;

        let mut reader = Cursor::new(b"\xA1\x05\x00\x00".to_vec());
        let actual_version = Version::read(&mut reader).expect("Must read here");

        assert_eq!(expected_version, actual_version);
    }
}
