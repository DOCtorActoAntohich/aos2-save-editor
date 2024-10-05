use super::encoded_u8::EncodedU8;

/// Keys are in little endian order.
#[binrw::binrw]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[brw(little)]
pub struct EncodedU32<const KEY1: u8, const KEY2: u8, const KEY3: u8, const KEY4: u8>(u32);

impl<const KEY1: u8, const KEY2: u8, const KEY3: u8, const KEY4: u8>
    EncodedU32<KEY1, KEY2, KEY3, KEY4>
{
    pub const fn pre_encoded(value: u32) -> Self {
        Self(value)
    }

    pub const fn encode_from_raw(value: u32) -> Self {
        let [b1, b2, b3, b4] = value.to_le_bytes();
        let bytes = [
            EncodedU8::<KEY1>::encode_from_raw(b1).get(),
            EncodedU8::<KEY2>::encode_from_raw(b2).get(),
            EncodedU8::<KEY3>::encode_from_raw(b3).get(),
            EncodedU8::<KEY4>::encode_from_raw(b4).get(),
        ];
        Self(u32::from_le_bytes(bytes))
    }

    pub const fn decode(self) -> u32 {
        let [b1, b2, b3, b4] = self.0.to_le_bytes();
        let bytes = [
            EncodedU8::<KEY1>::pre_encoded(b1).decode(),
            EncodedU8::<KEY2>::pre_encoded(b2).decode(),
            EncodedU8::<KEY3>::pre_encoded(b3).decode(),
            EncodedU8::<KEY4>::pre_encoded(b4).decode(),
        ];
        u32::from_le_bytes(bytes)
    }

    pub const fn get(&self) -> u32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::EncodedU32;

    #[rstest::rstest]
    #[case(0, EncodedU32::<0x4E, 0x5E, 0x6E, 0x7E>::pre_encoded(u32::from_le_bytes([0x4E, 0x5E, 0x6E, 0x7E])))]
    #[case(0, EncodedU32::<0x8E, 0x9E, 0xAE, 0xBE>::pre_encoded(u32::from_le_bytes([0x8E, 0x9E, 0xAE, 0xBE])))]
    #[case(1, EncodedU32::<0x8E, 0x9E, 0xAE, 0xBE>::pre_encoded(u32::from_le_bytes([0x9E, 0x9E, 0xAE, 0xBE])))]
    #[case(2304, EncodedU32::<0x8E, 0x9E, 0xAE, 0xBE>::pre_encoded(u32::from_le_bytes([0x8E, 0x0E, 0xAE, 0xBE])))]
    fn it_decodes<const K1: u8, const K2: u8, const K3: u8, const K4: u8>(
        #[case] expected_raw: u32,
        #[case] encoded: EncodedU32<K1, K2, K3, K4>,
    ) {
        let actual_raw = encoded.decode();

        assert_eq!(expected_raw, actual_raw);
    }
}
