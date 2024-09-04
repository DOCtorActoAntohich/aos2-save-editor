pub type NicknameSection = SizedBinarySection<1, 16>;
pub type LobbyNameSection = SizedBinarySection<1, 24>;
pub type LobbyPasswordSection = SizedBinarySection<0, 24>;
pub type UnlockableAvatarsSection = SizedBinarySection<33, 33>;
pub type UnlockableBackbroundsSection = SizedBinarySection<19, 19>;
pub type TitlesSection = SizedBinarySection<285, 285>;

#[binrw::binrw]
#[derive(Debug, Clone, Copy, derive_more::Into)]
#[brw(little, assert(Self::contains_u32(&self_0)))]
pub struct SectionSize<const MIN: usize, const MAX: usize>(u32);

#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq)]
#[brw(little)]
pub struct SizedBinarySection<const MIN: usize, const MAX: usize> {
    #[bw(try_calc = bytes.len().try_into())]
    size: SectionSize<MIN, MAX>,
    #[br(count = usize::from(size))]
    pub bytes: Vec<u8>,
}

#[derive(Debug, thiserror::Error)]
pub enum SizedBinarySectionError<const MIN: usize, const MAX: usize> {
    #[error("Section size is not in range {}-{}", MIN, MAX)]
    BadSize,
}

impl<const MIN: usize, const MAX: usize> SectionSize<MIN, MAX> {
    pub fn contains(value: usize) -> bool {
        (MIN..=MAX).contains(&value)
    }

    fn contains_u32(value: &u32) -> bool {
        Self::contains(*value as usize)
    }
}

impl<const MIN: usize, const MAX: usize> TryFrom<u32> for SectionSize<MIN, MAX> {
    type Error = SizedBinarySectionError<MIN, MAX>;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(value as usize)
    }
}

impl<const MIN: usize, const MAX: usize> TryFrom<usize> for SectionSize<MIN, MAX> {
    type Error = SizedBinarySectionError<MIN, MAX>;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if Self::contains(value) {
            Ok(Self(value as u32))
        } else {
            Err(SizedBinarySectionError::BadSize)
        }
    }
}

impl<const MIN: usize, const MAX: usize> From<SectionSize<MIN, MAX>> for usize {
    fn from(value: SectionSize<MIN, MAX>) -> Self {
        value.0 as usize
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use binrw::{BinRead, BinWrite};

    use super::{LobbyNameSection, LobbyPasswordSection, NicknameSection, SizedBinarySection};

    #[rstest::rstest]
    #[case::shortest_nick(
        NicknameSection { bytes: b"W".to_vec() },
        b"\x01\x00\x00\x00W"
    )]
    #[case::longest_nick(
        NicknameSection { bytes: b"Crazy Boii XDDDD".to_vec() },
        b"\x10\x00\x00\x00Crazy Boii XDDDD"
    )]
    #[case::lobby_name(
        LobbyNameSection { bytes: b"1234567890 1234567890".to_vec() },
        b"\x15\x00\x00\x001234567890 1234567890"
    )]
    #[case::lobby_password(
        LobbyPasswordSection { bytes: b"can't hacc this password".to_vec() },
        b"\x18\x00\x00\x00can't hacc this password"
    )]
    fn encodes_properly<const MIN: usize, const MAX: usize>(
        #[case] value: SizedBinarySection<MIN, MAX>,
        #[case] expected_binary: &[u8],
    ) {
        let mut cursor = Cursor::new(Vec::new());
        value.write(&mut cursor).expect("Must write here");

        let actual_binary = &cursor.into_inner()[..];
        assert_eq!(expected_binary, actual_binary);
    }

    #[rstest::rstest]
    #[case::empty_lobby_pass(b"\x00\x00\x00\x00", LobbyPasswordSection{ bytes: vec![] })]
    #[case::lobby_name(b"\x01\x00\x00\x00A", LobbyNameSection{ bytes: b"A".to_vec() })]
    #[case::nickname(b"\x06\x00\x00\x00abobus", NicknameSection{ bytes: b"abobus".to_vec() })]
    fn decodes_properly<const MIN: usize, const MAX: usize>(
        #[case] input: &[u8],
        #[case] expected_value: SizedBinarySection<MIN, MAX>,
    ) {
        let mut cursor = Cursor::new(input.to_vec());
        let actual_value =
            SizedBinarySection::<MIN, MAX>::read(&mut cursor).expect("Must read here");

        assert_eq!(expected_value, actual_value);
    }
}
