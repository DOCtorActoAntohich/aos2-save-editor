use std::string::FromUtf8Error;

pub type Nickname = SizedString<1, 16>;
pub type LobbyName = SizedString<1, 24>;
pub type LobbyPassword = SizedString<0, 24>;

#[binrw::binrw]
#[derive(Debug, Clone, Copy, derive_more::Into)]
#[brw(little, assert(Self::contains_u32(&self_0)))]
pub struct StringSize<const MIN: usize, const MAX: usize>(u32);

#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq)]
#[brw(little)]
pub struct SizedString<const MIN: usize, const MAX: usize> {
    #[bw(try_calc = bytes.len().try_into())]
    size: StringSize<MIN, MAX>,
    #[br(count = usize::from(size))]
    bytes: Vec<u8>,
}

#[derive(Debug, thiserror::Error)]
pub enum SizedStringError<const MIN: usize, const MAX: usize> {
    #[error("String size is not in range {}-{}", MIN, MAX)]
    BadSize,
    #[error("Sequence is not a valid UTF-8 string")]
    Utf8(
        #[from]
        #[source]
        FromUtf8Error,
    ),
}

impl<const MIN: usize, const MAX: usize> StringSize<MIN, MAX> {
    pub fn contains(value: usize) -> bool {
        (MIN..=MAX).contains(&value)
    }

    fn contains_u32(value: &u32) -> bool {
        Self::contains(*value as usize)
    }
}

impl<const MIN: usize, const MAX: usize> TryFrom<u32> for StringSize<MIN, MAX> {
    type Error = SizedStringError<MIN, MAX>;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(value as usize)
    }
}

impl<const MIN: usize, const MAX: usize> TryFrom<usize> for StringSize<MIN, MAX> {
    type Error = SizedStringError<MIN, MAX>;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if Self::contains(value) {
            Ok(Self(value as u32))
        } else {
            Err(SizedStringError::BadSize)
        }
    }
}

impl<const MIN: usize, const MAX: usize> From<StringSize<MIN, MAX>> for usize {
    fn from(value: StringSize<MIN, MAX>) -> Self {
        value.0 as usize
    }
}

impl<const MIN: usize, const MAX: usize> TryFrom<SizedString<MIN, MAX>> for String {
    type Error = SizedStringError<MIN, MAX>;

    fn try_from(value: SizedString<MIN, MAX>) -> Result<Self, Self::Error> {
        String::from_utf8(value.bytes).map_err(SizedStringError::Utf8)
    }
}

impl<const MIN: usize, const MAX: usize> TryFrom<String> for SizedString<MIN, MAX> {
    type Error = SizedStringError<MIN, MAX>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if StringSize::<MIN, MAX>::contains(value.len()) {
            Ok(Self {
                bytes: value.into_bytes(),
            })
        } else {
            Err(SizedStringError::BadSize)
        }
    }
}

impl<const MIN: usize, const MAX: usize> TryFrom<&str> for SizedString<MIN, MAX> {
    type Error = SizedStringError<MIN, MAX>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if StringSize::<MIN, MAX>::contains(value.len()) {
            Ok(Self {
                bytes: value.into(),
            })
        } else {
            Err(SizedStringError::BadSize)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use binrw::{BinRead, BinWrite};

    use super::{LobbyName, LobbyPassword, Nickname, SizedString};

    #[rstest::rstest]
    #[case::shortest_nick(Nickname::try_from("W").expect("Valid nickname"), b"\x01\x00\x00\x00W")]
    #[case::longest_nick(
        Nickname::try_from("Crazy Boii XDDDD").expect("Valid nickname"),
        b"\x10\x00\x00\x00Crazy Boii XDDDD"
    )]
    #[case::lobby_name(LobbyName::try_from("1234567890 1234567890").expect("Valid nickname"), b"\x15\x00\x00\x001234567890 1234567890")]
    #[case::lobby_password(LobbyPassword::try_from("can't hacc this password").expect("Valid nickname"), b"\x18\x00\x00\x00can't hacc this password")]
    fn encodes_properly<const MIN: usize, const MAX: usize>(
        #[case] value: SizedString<MIN, MAX>,
        #[case] expected_binary: &[u8],
    ) {
        let mut cursor = Cursor::new(Vec::new());
        value.write(&mut cursor).expect("Must write here");

        let actual_binary = &cursor.into_inner()[..];
        assert_eq!(expected_binary, actual_binary);
    }

    #[rstest::rstest]
    #[case::empty_lobby_pass(b"\x00\x00\x00\x00", LobbyPassword{ bytes: vec![] }, "")]
    #[case::lobby_name(b"\x01\x00\x00\x00A", LobbyName{ bytes: b"A".into() }, "A")]
    #[case::nickname(b"\x06\x00\x00\x00abobus", Nickname{ bytes: b"abobus".into() }, "abobus")]
    fn decodes_properly<const MIN: usize, const MAX: usize>(
        #[case] input: &[u8],
        #[case] expected_value: SizedString<MIN, MAX>,
        #[case] expected_string: &str,
    ) {
        let mut cursor = Cursor::new(input.to_vec());
        let actual_value = SizedString::<MIN, MAX>::read(&mut cursor).expect("Must read here");
        let actual_string: String = actual_value
            .clone()
            .try_into()
            .expect("Must be a valid string");

        assert_eq!(expected_value, actual_value);
        assert_eq!(expected_string, actual_string);
    }
}
