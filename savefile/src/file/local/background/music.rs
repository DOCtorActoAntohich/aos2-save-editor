use crate::bin_bool::BinBool;

#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[brw(little)]
pub struct BackgroundMusicSheet {
    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub need_for_speed: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub black_hole: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub distant_thunder: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub swordfish: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub shine: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub expendables: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub ribbon: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub moving_out: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub accelerator: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub remember_me: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub mgom: bool,
}

impl Default for BackgroundMusicSheet {
    fn default() -> Self {
        Self {
            need_for_speed: true,
            black_hole: true,
            distant_thunder: true,
            shine: true,
            expendables: true,
            ribbon: true,
            moving_out: true,

            swordfish: false,
            accelerator: false,
            remember_me: false,
            mgom: false,
        }
    }
}

impl BackgroundMusicSheet {
    pub const FULLY_UNLOCKED: Self = Self {
        need_for_speed: true,
        black_hole: true,
        distant_thunder: true,
        swordfish: true,
        shine: true,
        expendables: true,
        ribbon: true,
        moving_out: true,
        accelerator: true,
        remember_me: true,
        mgom: true,
    };
}
