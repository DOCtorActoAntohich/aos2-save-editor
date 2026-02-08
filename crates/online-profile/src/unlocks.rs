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
    ($name:ident, DEFAULT_SIZE = $size:expr) => {
        #[binrw::binrw]
        #[derive(Debug, Clone)]
        #[brw(little)]
        pub struct $name {
            #[bw(try_calc = items.len().try_into())]
            length: u32,
            #[br(count = length as usize)]
            items: Vec<Status>,
        }

        impl $name {
            #[must_use]
            pub fn is_fully_unlocked(&self) -> bool {
                let Self { items } = self;
                items.iter().all(|&item| item == Status::Open)
            }

            pub fn unlock_all(&mut self) {
                let Self { items } = self;
                for item in items.iter_mut() {
                    *item = Status::Open;
                }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    items: vec![Status::Locked; $size],
                }
            }
        }
    };
}

declare_sized_section!(TitlesSection, DEFAULT_SIZE = 0x01_11);
declare_sized_section!(AvatarsSection, DEFAULT_SIZE = 0x1f);
declare_sized_section!(BackgroundsSection, DEFAULT_SIZE = 0x13);
