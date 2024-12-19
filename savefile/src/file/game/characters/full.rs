use crate::bin_bool::BinBool;

/// Full list of characters.
///
/// ORDER MATTERS. That's how they are coded in game.
///
/// Also see [`StoryCharacterSheet`].
#[binrw::binrw]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[brw(little)]
pub struct FullCharacterSheet {
    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub sora: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub alte: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub tsih: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub mira: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub sham: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub nath: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub star_breaker: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub suguri: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub saki: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub iru: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub nanako: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub kae: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub kyoko: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub hime: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub sumika: bool,
}

impl FullCharacterSheet {
    pub const FULLY_UNLOCKED: Self = Self {
        sora: true,
        alte: true,
        tsih: true,
        mira: true,
        sham: true,
        nath: true,
        star_breaker: true,
        suguri: true,
        saki: true,
        iru: true,
        nanako: true,
        kae: true,
        kyoko: true,
        hime: true,
        sumika: true,
    };
}
