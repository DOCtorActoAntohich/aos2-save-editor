use super::ascii_text;

type Text = ascii_text::AsciiText<0, 24>;
type Error = ascii_text::Error<0, 24>;

#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, derive_more::From, derive_more::Display)]
#[brw(little)]
pub struct LobbyName(Text);

impl TryFrom<String> for LobbyName {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Text::new(value).map(Self)
    }
}

impl TryFrom<&str> for LobbyName {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Text::new(value).map(Self)
    }
}

impl Default for LobbyName {
    fn default() -> Self {
        Self(Text::new("Suguri").expect("Invariant: Valid lobby name"))
    }
}
