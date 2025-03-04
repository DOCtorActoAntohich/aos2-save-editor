use std::ops::RangeInclusive;

#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, derive_more::Display)]
#[brw(little)]
#[br(try_map = |encoded: EncodedText| encoded.try_into())]
#[bw(map = EncodedText::from)]
pub struct AsciiText<const MIN_LENGTH: u8, const MAX_LENGTH: u8>(String);

#[derive(Debug, thiserror::Error)]
pub enum Error<const MIN_LENGTH: u8, const MAX_LENGTH: u8> {
    #[error("Length must be {}-{} characters", MIN_LENGTH, MAX_LENGTH)]
    Length,
    #[error("Non-ASCII characters are not allowed")]
    Ascii,
    #[error("Bad encoding")]
    Encoding,
}

#[binrw::binrw]
#[derive(Debug, Clone)]
#[brw(little)]
struct EncodedText {
    #[bw(try_calc = ascii_chars.len().try_into())]
    length: u32,
    #[br(count = length as usize)]
    ascii_chars: Vec<u8>,
}

impl<const MIN_LENGTH: u8, const MAX_LENGTH: u8> AsciiText<MIN_LENGTH, MAX_LENGTH> {
    pub const LENGTH_RANGE: RangeInclusive<usize> = const {
        assert!(MIN_LENGTH < MAX_LENGTH);
        MIN_LENGTH as usize..=MAX_LENGTH as usize
    };

    pub fn new(text: impl Into<String>) -> Result<Self, Error<MIN_LENGTH, MAX_LENGTH>> {
        let text: String = text.into();

        if !Self::LENGTH_RANGE.contains(&text.len()) {
            Err(Error::Length)
        } else if !text.is_ascii() {
            Err(Error::Ascii)
        } else {
            Ok(Self(text))
        }
    }
}

impl<const MIN: u8, const MAX: u8> TryFrom<String> for AsciiText<MIN, MAX> {
    type Error = Error<MIN, MAX>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<const MIN: u8, const MAX: u8> TryFrom<EncodedText> for AsciiText<MIN, MAX> {
    type Error = Error<MIN, MAX>;

    fn try_from(EncodedText { ascii_chars }: EncodedText) -> Result<Self, Self::Error> {
        let s = String::from_utf8(ascii_chars).map_err(|_| Error::Encoding)?;
        Self::new(s)
    }
}

impl<const MIN: u8, const MAX: u8> From<&AsciiText<MIN, MAX>> for EncodedText {
    fn from(AsciiText(s): &AsciiText<MIN, MAX>) -> Self {
        EncodedText {
            ascii_chars: s.as_bytes().to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use binrw::{BinRead, BinWrite};

    use super::AsciiText;

    #[rstest_reuse::template]
    #[rstest::rstest]
    #[case::shortest(
        b"\x01\x00\x00\x00W",
        AsciiText::<1,16>::new("W").expect("Precondition: Valid text")
    )]
    #[case::longest(
        b"\x10\x00\x00\x00Crazy Boii XDDDD",
        AsciiText::<1,16>::new("Crazy Boii XDDDD").expect("Precondition: Valid text")
    )]
    fn encode_decode_pairs(#[case] raw: &[u8], #[case] safe: AsciiText<MIN, MAX>) {}

    #[rstest_reuse::apply(encode_decode_pairs)]
    fn ok_decoding<const MIN: u8, const MAX: u8>(
        #[case] input: &[u8],
        #[case] expected: AsciiText<MIN, MAX>,
    ) {
        let mut cursor = Cursor::new(input.to_vec());
        let actual = AsciiText::<MIN, MAX>::read(&mut cursor).expect("Must read here");

        assert_eq!(expected, actual);
    }

    #[rstest_reuse::apply(encode_decode_pairs)]
    fn ok_encoding<const MIN: u8, const MAX: u8>(
        #[case] expected: &[u8],
        #[case] input: AsciiText<MIN, MAX>,
    ) {
        let mut cursor = Cursor::new(Vec::new());
        input.write(&mut cursor).expect("Must write here");

        let actual = cursor.into_inner();

        assert_eq!(expected, actual);
    }

    #[rstest::rstest]
    #[case::too_short(b"\x01\x00\x00\x00L")]
    #[case::too_long(b"\x10\x00\x00\x001234567890123456")]
    fn bad_decoding(#[case] input: &[u8]) {
        let mut cursor = Cursor::new(input.to_vec());
        let _err = AsciiText::<4, 8>::read(&mut cursor).expect_err("Must fail here");
    }
}
