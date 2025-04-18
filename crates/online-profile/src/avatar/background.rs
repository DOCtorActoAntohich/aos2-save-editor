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
pub enum Background {
    #[default]
    LightBlue = 0x00,
    Pink = 0x01,
    Green = 0x02,
    Red = 0x03,
    Yellow = 0x04,
    Purple = 0x05,
    Black = 0x06,
    DarkOrange = 0x07,
    LightOrange = 0x08,
    Blue = 0x09,
    DarkBrown = 0x0a,
    Silver = 0x0b,
    Peach = 0x0c,
    LightGreen = 0x0d,
    LightBrown = 0x0e,
    Turquoise = 0x0f,
    Raspberry = 0x10,
    DarkGreen = 0x11,
    DeepBlue = 0x12,
    Aurora = 0x13,
    Sunset = 0x14,
    Teal = 0x15,
    RedAndBlue = 0x16,
    Orange = 0x17,
    Lavender = 0x18,
    Cyan = 0x19,
    SeaWater = 0x1a,
    Olive = 0x1b,
    Sky = 0x1c,
    StrawberryChocolate = 0x1d,
    DeepPink = 0x1e,
    Beach = 0x1f,
    Beige = 0x20,
    Aquamarine = 0x21,
    Tropic = 0x22,
    QuarantinedRapport = 0x23,
    BulletOrange = 0x24,
    /// Looks similar to Character Silhouette but it's just an image.
    /// It's slightly different.
    #[display("<Default Silhouette>")]
    LightGrayBackgroundWithSilhouette = 0xff,
}

impl From<Background> for u32 {
    fn from(value: Background) -> Self {
        value as u32
    }
}
