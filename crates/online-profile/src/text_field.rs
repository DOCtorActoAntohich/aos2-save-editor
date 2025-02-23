use crate::ascii_text;

type NicknameAsciiText = ascii_text::AsciiText<1, 16>;
type NicknameAsciiError = ascii_text::Error<1, 16>;

#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, derive_more::From)]
#[brw(little)]
pub struct Nickname(NicknameAsciiText);

impl TryFrom<String> for Nickname {
    type Error = NicknameAsciiError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        NicknameAsciiText::new(value).map(Self)
    }
}

impl TryFrom<&str> for Nickname {
    type Error = NicknameAsciiError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        NicknameAsciiText::new(value).map(Self)
    }
}
