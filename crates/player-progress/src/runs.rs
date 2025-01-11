/// Markers for 1CC (no deaths) Story mode completion.
///
/// Unlike in the Arcade mode, it doesn't have Sumika,
/// because she doesn't have her own Story mode run.
#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[brw(little)]
pub struct PerfectStoryMode {
    pub sora: Run,
    pub alte: Run,
    pub tsih: Run,
    pub mira: Run,
    pub sham: Run,
    pub nath: Run,
    pub star_breaker: Run,
    pub suguri: Run,
    pub saki: Run,
    pub iru: Run,
    pub nanako: Run,
    pub kae: Run,
    pub kyoko: Run,
    pub hime: Run,
}

/// Markers for 1CC (no deaths) Arcade mode completion.
///
/// This time Sumika is here, cos she has Arcade mode runs
#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[brw(little)]
pub struct PerfectArcadeMode {
    pub sora: Run,
    pub alte: Run,
    pub tsih: Run,
    pub mira: Run,
    pub sham: Run,
    pub nath: Run,
    pub star_breaker: Run,
    pub suguri: Run,
    pub saki: Run,
    pub iru: Run,
    pub nanako: Run,
    pub kae: Run,
    pub kyoko: Run,
    pub hime: Run,
    pub sumika: Run,
}

#[binrw::binrw]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[brw(little)]
pub enum Run {
    #[brw(magic = 0x01u8)]
    Completed,
    #[default]
    #[brw(magic = 0x00u8)]
    NotCompleted,
}

impl PerfectStoryMode {
    pub const COMPLETED: Self = Self {
        sora: Run::Completed,
        alte: Run::Completed,
        tsih: Run::Completed,
        mira: Run::Completed,
        sham: Run::Completed,
        nath: Run::Completed,
        star_breaker: Run::Completed,
        suguri: Run::Completed,
        saki: Run::Completed,
        iru: Run::Completed,
        nanako: Run::Completed,
        kae: Run::Completed,
        kyoko: Run::Completed,
        hime: Run::Completed,
    };
}

impl PerfectArcadeMode {
    pub const COMPLETED: Self = Self {
        sora: Run::Completed,
        alte: Run::Completed,
        tsih: Run::Completed,
        mira: Run::Completed,
        sham: Run::Completed,
        nath: Run::Completed,
        star_breaker: Run::Completed,
        suguri: Run::Completed,
        saki: Run::Completed,
        iru: Run::Completed,
        nanako: Run::Completed,
        kae: Run::Completed,
        kyoko: Run::Completed,
        hime: Run::Completed,
        sumika: Run::Completed,
    };
}
