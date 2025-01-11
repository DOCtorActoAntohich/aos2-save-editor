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
