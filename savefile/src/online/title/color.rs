/// Local-only background color for all titles in the lobby.
#[binrw::binrw]
#[derive(Debug, Clone, Copy, PartialEq, Eq, derive_more::TryFrom, derive_more::Display)]
#[brw(little, repr(u32))]
#[repr(u32)]
pub enum Color {
    Yellow = 0x00,
    Blue = 0x01,
    Green = 0x02,
    Red = 0x03,
}
