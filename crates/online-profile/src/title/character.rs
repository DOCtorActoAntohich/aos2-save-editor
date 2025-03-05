/// Little character eyes peeking from the title background.
///
/// In the game, only AoS2 characters are available.
/// They are also coupled to the characters whose title you select.
///
/// Sadly, AoS2 doesn't expose any interface to change those freely,
/// but it internally supports OJ variants too,
/// and even allows setting one character's title and another character's eyes.
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
pub enum Character {
    #[default]
    #[display("<No character>")]
    None = 0x00,
    Sora = 0x01,
    Alte = 0x02,
    Tsih = 0x03,
    Mira = 0x04,
    Sham = 0x05,
    Nath = 0x06,
    #[display("Star Breaker")]
    StarBreaker = 0x07,
    Suguri = 0x08,
    Saki = 0x09,
    Iru = 0x0a,
    Nanako = 0x0b,
    Kae = 0x0c,
    Kyoko = 0x0d,
    Hime = 0x0e,
    Sumika = 0x0f,
    #[display("OJ Alte")]
    OjAlte = 0x10,
    #[display("OJ Hime")]
    OjHime = 0x11,
    #[display("OJ Winter Hime")]
    OjHimeWinter = 0x12,
    #[display("OJ Kae")]
    OjKae = 0x13,
    #[display("OJ Kyoko")]
    OjKyoko = 0x14,
    #[display("OJ Nanako")]
    OjNanako = 0x15,
    #[display("OJ Nath")]
    OjNath = 0x16,
    #[display("OJ Saki")]
    OjSaki = 0x17,
    #[display("OJ Sham")]
    OjSham = 0x18,
    #[display("OJ Sora")]
    OjSora = 0x19,
    #[display("OJ Military Sora")]
    OjSoraMilitary = 0x1a,
    #[display("OJ Star Breaker")]
    OjStarBreaker = 0x1b,
    #[display("OJ Suguri")]
    OjSuguri = 0x1c,
    #[display("OJ Winter Suguri")]
    OjSuguriWinter = 0x1d,
    #[display("OJ Iru")]
    OjIru = 0x1e,
    #[display("OJ Mira")]
    OjMira = 0x1f,
    #[display("<Disable Title>")]
    DisableTitle = 0xff,
}
