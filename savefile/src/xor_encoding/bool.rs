use super::{key::Key, u8::EncodedU8};

#[binrw::binrw]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[brw(little)]
pub struct EncodedBool(EncodedU8);

impl EncodedBool {
    pub const fn pre_encoded(value: u8) -> Self {
        Self(EncodedU8::pre_encoded(value))
    }

    pub const fn from_raw(raw: bool, key: Key) -> Self {
        let inner = match raw {
            true => 1,
            false => 0,
        };
        Self(EncodedU8::from_raw(inner, key))
    }

    pub const fn decode(self, key: Key) -> bool {
        let inner = self.0.decode(key);
        inner != 0
    }

    pub const fn get(&self) -> u8 {
        self.0.get()
    }
}
