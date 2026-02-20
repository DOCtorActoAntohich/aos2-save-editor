use binary_file::UnsupportedVersion;
use binrw::{BinRead, BinResult};

#[binrw::binrw]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, derive_more::Display)]
#[brw(little)]
pub struct Version(#[br(parse_with = Version::bin_parse)] u32);

impl Version {
    const EXPECTED: u32 = u32::from_le_bytes([0xA2, 0x05, 0x00, 0x00]);

    pub const fn current() -> Self {
        Self(Self::EXPECTED)
    }

    fn bin_parse<R: binrw::io::Read + binrw::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: (),
    ) -> BinResult<u32> {
        let pos = reader.stream_position().unwrap_or_default();
        let actual_version = <u32 as BinRead>::read_options(reader, endian, args)?;
        if Self::EXPECTED == actual_version {
            Ok(Self::EXPECTED)
        } else {
            Err(binrw::Error::Custom {
                pos,
                err: Box::new(UnsupportedVersion {
                    expected: Self::EXPECTED,
                    actual: actual_version,
                }),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use binrw::{BinRead, BinWrite};

    use super::Version;

    #[rstest::rstest]
    fn version_encodes_properly() {
        let expected_version = b"\xA2\x05\x00\x00";

        let mut writer = Cursor::new(Vec::new());
        Version::current()
            .write(&mut writer)
            .expect("Must write here");

        let actual_version = &writer.into_inner()[..];
        assert_eq!(expected_version, actual_version);
    }

    #[rstest::rstest]
    fn version_decodes_properly() {
        let expected_version = Version::current();

        let mut reader = Cursor::new(b"\xA2\x05\x00\x00".to_vec());
        let actual_version = Version::read(&mut reader).expect("Must read here");

        assert_eq!(expected_version, actual_version);
    }

    #[rstest::rstest]
    fn bad_version_returns_proper_error() {
        let expected_err = binary_file::UnsupportedVersion {
            expected: Version::EXPECTED,
            actual: u32::from_le_bytes([0x00, 0x00, 0x00, 0x00]),
        };

        let mut reader = Cursor::new(b"\x00\x00\x00\x00");
        let err = Version::read(&mut reader).expect_err("Must fail here");

        let actual_err = match binary_file::ErrorDetail::from(err) {
            binary_file::ErrorDetail::UnsupportedVersion(error) => error,
            other => panic!("Unexpected error variant: {other:?}"),
        };

        assert_eq!(expected_err, actual_err);
    }
}
