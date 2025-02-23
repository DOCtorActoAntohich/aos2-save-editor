use super::ascii_text;

type Text = ascii_text::AsciiText<1, 16>;
type Error = ascii_text::Error<1, 16>;

#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, derive_more::From)]
#[brw(little)]
pub struct Nickname(Text);

impl TryFrom<String> for Nickname {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Text::new(value).map(Self)
    }
}

impl TryFrom<&str> for Nickname {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Text::new(value).map(Self)
    }
}
