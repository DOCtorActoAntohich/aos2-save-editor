#[binrw::binrw]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    derive_more::TryFrom,
    derive_more::Display,
    enum_array::EnumMembersArray,
)]
#[try_from(repr)]
#[repr(u32)]
#[brw(little, repr(u32))]
pub enum Character {
    Silhouette = 0x00,
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
    #[display("100% Alte")]
    OjAlte = 0x10,
    #[display("100% Hime")]
    OjHime = 0x11,
    #[display("100% Hime Winter")]
    OjHimeWinter = 0x12,
    #[display("100% Kae")]
    OjKae = 0x13,
    #[display("100% Kyoko")]
    OjKyoko = 0x14,
    #[display("100% Nanako")]
    OjNanako = 0x15,
    #[display("100% Nath")]
    OjNath = 0x16,
    #[display("100% Nath Armor")]
    OjNathExtension = 0x17,
    #[display("100% Saki")]
    OjSaki = 0x18,
    #[display("100% Sham")]
    OjSham = 0x19,
    #[display("100% Sora")]
    OjSora = 0x1a,
    #[display("100% Sora Military")]
    OjSoraMilitary = 0x1b,
    #[display("100% Star Breaker")]
    OjStarBreaker = 0x1c,
    #[display("100% Suguri")]
    OjSuguri = 0x1d,
    #[display("100% Suguri Winter")]
    OjSuguriWinter = 0x1e,
    #[display("100% Iru")]
    OjIru = 0x1f,
    #[display("100% Mira")]
    OjMira = 0x20,
    #[display("100% Tsih")]
    OjTsih = 0x21,
    #[display("100% Suguri 46 Billion Years Old")]
    OjSuguri46BilYears = 0x22,
    #[display("100% Sumika")]
    OjSumika = 0x23,
    #[display("100% Suguri Summer")]
    OjSuguriSummer = 0x24,
    #[display("100% Sora Summer")]
    OjSoraSummer = 0x25,
    #[display("100% Hime Summer")]
    OjHimeSummer = 0x26,
    #[display("100% Saki Summer")]
    OjSakiSummer = 0x27,
    #[display("100% Kae Summer")]
    OjKaeSummer = 0x28,
    #[display("100% Nath Summer")]
    OjNathSummer = 0x29,
    #[display("Quarantined Rapport")]
    QuarantinedRapport = 0x2a,
    #[display("Bullet Orange Suguri")]
    SuguriBulletOrange = 0x2b,
    #[display("Bullet Orange Sora")]
    SoraBulletOrange = 0x2c,
    #[display("AoS2 10th Anniversary Suguri")]
    SuguriAnniversary = 0x2d,
    #[display("AoS2 10th Anniversary Sora")]
    SoraAnniversary = 0x2e,
    /// Similar to `Silhouette`, but it's a different looking placeholder.
    #[display("<no avatar>")]
    Empty = 0xff,
}
