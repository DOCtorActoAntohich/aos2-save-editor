#[binrw::binrw]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[brw(little)]
pub enum Status {
    #[brw(magic = 0x01u8)]
    Enabled,
    #[default]
    #[brw(magic = 0x00u8)]
    Disabled,
}

impl Status {
    #[must_use]
    pub fn is_enabled(&self) -> bool {
        match self {
            Status::Enabled => true,
            Status::Disabled => false,
        }
    }
}

impl std::ops::Not for Status {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Status::Enabled => Status::Disabled,
            Status::Disabled => Status::Enabled,
        }
    }
}
