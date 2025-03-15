#[binrw::binrw]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[brw(little)]
pub enum Status {
    #[brw(magic = 0x01u8)]
    Open,
    #[default]
    #[brw(magic = 0x00u8)]
    Locked,
}

macro_rules! declare_sized_section {
    ($name:ident, SIZE = $size:expr) => {
        #[binrw::binrw]
        #[derive(Debug, Clone)]
        #[brw(little)]
        pub struct $name {
            #[brw(magic = $size)]
            items: [Status; Self::SIZE],
        }

        impl $name {
            pub const SIZE: usize = $size as usize;

            #[must_use]
            pub fn is_fully_unlocked(&self) -> bool {
                let Self { items } = self;
                items.iter().all(|&item| item == Status::Open)
            }

            pub fn unlock_all(&mut self) {
                let Self { items } = self;
                items.iter_mut().for_each(|item| *item = Status::Open);
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    items: [Status::Locked; Self::SIZE],
                }
            }
        }
    };
}

declare_sized_section!(TitlesSection, SIZE = 285u32);
declare_sized_section!(AvatarsSection, SIZE = 33u32);
declare_sized_section!(BackgroundsSection, SIZE = 19u32);
