use super::ascii_text;

type Text = ascii_text::AsciiText<0, 24>;
type Error = ascii_text::Error<0, 24>;

#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, derive_more::From)]
#[brw(little)]
pub struct LobbyPassword(Text);

impl TryFrom<String> for LobbyPassword {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Text::new(value).map(Self)
    }
}

impl TryFrom<&str> for LobbyPassword {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Text::new(value).map(Self)
    }
}
