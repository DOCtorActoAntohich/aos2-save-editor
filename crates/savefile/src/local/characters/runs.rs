/// Markers for 1CC (no deaths) Story mode completion.
///
/// Unlike in the Arcade mode, it doesn't have Sumika,
/// because she doesn't have her own Story mode run.
#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[brw(little)]
pub struct PerfectStoryModeRuns {
    pub sora: PerfectRun,
    pub alte: PerfectRun,
    pub tsih: PerfectRun,
    pub mira: PerfectRun,
    pub sham: PerfectRun,
    pub nath: PerfectRun,
    pub star_breaker: PerfectRun,
    pub suguri: PerfectRun,
    pub saki: PerfectRun,
    pub iru: PerfectRun,
    pub nanako: PerfectRun,
    pub kae: PerfectRun,
    pub kyoko: PerfectRun,
    pub hime: PerfectRun,
}

/// Markers for 1CC (no deaths) Arcade mode completion.
///
/// This time Sumika is here, cos she has Arcade mode runs
#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[brw(little)]
pub struct PerfectArcadeModeRuns {
    pub sora: PerfectRun,
    pub alte: PerfectRun,
    pub tsih: PerfectRun,
    pub mira: PerfectRun,
    pub sham: PerfectRun,
    pub nath: PerfectRun,
    pub star_breaker: PerfectRun,
    pub suguri: PerfectRun,
    pub saki: PerfectRun,
    pub iru: PerfectRun,
    pub nanako: PerfectRun,
    pub kae: PerfectRun,
    pub kyoko: PerfectRun,
    pub hime: PerfectRun,
    pub sumika: PerfectRun,
}

#[binrw::binrw]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[brw(little)]
pub enum PerfectRun {
    #[brw(magic = 0x01u8)]
    Completed,
    #[default]
    #[brw(magic = 0x00u8)]
    NotCompleted,
}

impl PerfectStoryModeRuns {
    pub const COMPLETED: Self = Self {
        sora: PerfectRun::Completed,
        alte: PerfectRun::Completed,
        tsih: PerfectRun::Completed,
        mira: PerfectRun::Completed,
        sham: PerfectRun::Completed,
        nath: PerfectRun::Completed,
        star_breaker: PerfectRun::Completed,
        suguri: PerfectRun::Completed,
        saki: PerfectRun::Completed,
        iru: PerfectRun::Completed,
        nanako: PerfectRun::Completed,
        kae: PerfectRun::Completed,
        kyoko: PerfectRun::Completed,
        hime: PerfectRun::Completed,
    };
}

impl PerfectArcadeModeRuns {
    pub const COMPLETED: Self = Self {
        sora: PerfectRun::Completed,
        alte: PerfectRun::Completed,
        tsih: PerfectRun::Completed,
        mira: PerfectRun::Completed,
        sham: PerfectRun::Completed,
        nath: PerfectRun::Completed,
        star_breaker: PerfectRun::Completed,
        suguri: PerfectRun::Completed,
        saki: PerfectRun::Completed,
        iru: PerfectRun::Completed,
        nanako: PerfectRun::Completed,
        kae: PerfectRun::Completed,
        kyoko: PerfectRun::Completed,
        hime: PerfectRun::Completed,
        sumika: PerfectRun::Completed,
    };
}
