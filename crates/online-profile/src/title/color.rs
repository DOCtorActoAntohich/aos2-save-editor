/// Local-only background color for all titles in the lobby.
#[binrw::binrw]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    derive_more::TryFrom,
    derive_more::Display,
    enum_array::EnumMembersArray,
)]
#[brw(little, repr(u32))]
#[repr(u32)]
#[try_from(repr)]
pub enum Color {
    #[default]
    Yellow = 0x00,
    Blue = 0x01,
    Green = 0x02,
    Red = 0x03,
}

impl From<Color> for u32 {
    fn from(value: Color) -> Self {
        value as u32
    }
}
