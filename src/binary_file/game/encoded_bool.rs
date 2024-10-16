use super::encoded_u8::EncodedU8;

#[binrw::binrw]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[brw(little)]
pub struct EncodedBool<const KEY: u8>(EncodedU8<KEY>);

impl<const KEY: u8> EncodedBool<KEY> {
    pub const fn pre_encoded(value: u8) -> Self {
        Self(EncodedU8::pre_encoded(value))
    }

    pub const fn encode_from_raw(raw: bool) -> Self {
        let inner = match raw {
            true => 1,
            false => 0,
        };
        Self(EncodedU8::encode_from_raw(inner))
    }

    pub const fn decode(self) -> bool {
        let inner = self.0.decode();
        inner != 0
    }

    pub const fn get(&self) -> u8 {
        self.0.get()
    }
}
