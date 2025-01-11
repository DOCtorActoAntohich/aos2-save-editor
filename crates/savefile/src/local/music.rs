use crate::lock::Status;

#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[brw(little)]
pub struct Music {
    pub need_for_speed: Status,
    pub black_hole: Status,
    pub distant_thunder: Status,
    pub swordfish: Status,
    pub shine: Status,
    pub expendables: Status,
    pub ribbon: Status,
    pub moving_out: Status,
    pub accelerator: Status,
    pub remember_me: Status,
    pub mgom: Status,
}

impl Default for Music {
    fn default() -> Self {
        Self {
            need_for_speed: Status::Enabled,
            black_hole: Status::Enabled,
            distant_thunder: Status::Enabled,
            shine: Status::Enabled,
            expendables: Status::Enabled,
            ribbon: Status::Enabled,
            moving_out: Status::Enabled,

            swordfish: Status::Disabled,
            accelerator: Status::Disabled,
            remember_me: Status::Disabled,
            mgom: Status::Disabled,
        }
    }
}

impl Music {
    pub const ALL: Self = Self {
        need_for_speed: Status::Enabled,
        black_hole: Status::Enabled,
        distant_thunder: Status::Enabled,
        swordfish: Status::Enabled,
        shine: Status::Enabled,
        expendables: Status::Enabled,
        ribbon: Status::Enabled,
        moving_out: Status::Enabled,
        accelerator: Status::Enabled,
        remember_me: Status::Enabled,
        mgom: Status::Enabled,
    };
}
