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

impl Run {
    pub fn is_completed(&self) -> bool {
        match self {
            Run::Completed => true,
            Run::NotCompleted => false,
        }
    }
}

impl PerfectStoryMode {
    pub const N_CHARACTERS: usize = 14;

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

    #[must_use]
    pub fn to_array(&self) -> [Run; Self::N_CHARACTERS] {
        self.clone().into()
    }
}

impl PerfectArcadeMode {
    pub const N_CHARACTERS: usize = 15;

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

    #[must_use]
    pub fn to_array(&self) -> [Run; Self::N_CHARACTERS] {
        self.clone().into()
    }
}

impl From<PerfectStoryMode> for [Run; PerfectStoryMode::N_CHARACTERS] {
    fn from(
        PerfectStoryMode {
            sora,
            alte,
            tsih,
            mira,
            sham,
            nath,
            star_breaker,
            suguri,
            saki,
            iru,
            nanako,
            kae,
            kyoko,
            hime,
        }: PerfectStoryMode,
    ) -> Self {
        [
            sora,
            alte,
            tsih,
            mira,
            sham,
            nath,
            star_breaker,
            suguri,
            saki,
            iru,
            nanako,
            kae,
            kyoko,
            hime,
        ]
    }
}

impl From<[Run; PerfectStoryMode::N_CHARACTERS]> for PerfectStoryMode {
    fn from(
        [
        sora,
        alte,
        tsih,
        mira,
        sham,
        nath,
        star_breaker,
        suguri,
        saki,
        iru,
        nanako,
        kae,
        kyoko,
        hime,
    ]: [Run; PerfectStoryMode::N_CHARACTERS],
    ) -> Self {
        Self {
            sora,
            alte,
            tsih,
            mira,
            sham,
            nath,
            star_breaker,
            suguri,
            saki,
            iru,
            nanako,
            kae,
            kyoko,
            hime,
        }
    }
}

impl From<PerfectArcadeMode> for [Run; PerfectArcadeMode::N_CHARACTERS] {
    fn from(
        PerfectArcadeMode {
            sora,
            alte,
            tsih,
            mira,
            sham,
            nath,
            star_breaker,
            suguri,
            saki,
            iru,
            nanako,
            kae,
            kyoko,
            hime,
            sumika,
        }: PerfectArcadeMode,
    ) -> Self {
        [
            sora,
            alte,
            tsih,
            mira,
            sham,
            nath,
            star_breaker,
            suguri,
            saki,
            iru,
            nanako,
            kae,
            kyoko,
            hime,
            sumika,
        ]
    }
}

impl From<[Run; PerfectArcadeMode::N_CHARACTERS]> for PerfectArcadeMode {
    fn from(
        [
        sora,
        alte,
        tsih,
        mira,
        sham,
        nath,
        star_breaker,
        suguri,
        saki,
        iru,
        nanako,
        kae,
        kyoko,
        hime,
        sumika,
    ]: [Run; PerfectArcadeMode::N_CHARACTERS],
    ) -> Self {
        Self {
            sora,
            alte,
            tsih,
            mira,
            sham,
            nath,
            star_breaker,
            suguri,
            saki,
            iru,
            nanako,
            kae,
            kyoko,
            hime,
            sumika,
        }
    }
}
