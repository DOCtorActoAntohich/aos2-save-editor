use super::u8::{EncodedU8, KeyU8};

/// An encoded [`u32`] with [`KeyU32`].
///
/// Equivalent to four consecutive [`EncodedU8`] values, each with its own key.
///
/// Bytes are in Little Endian order.
///
/// This thing exists to make life easier cos some counters in savefile are [`u32`].
#[binrw::binrw]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[brw(little)]
pub struct EncodedU32(u32);

/// A xor "encryption" key for [`u32`] values in `game.sys`.
///
/// Behaves as 4 consecutive [`KeyU8`] values,
/// with each next one incremented by 1.
///
/// Bytes are in Little Endian order, just like values in savefile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyU32([KeyU8; 4]);

impl EncodedU32 {
    pub const fn pre_encoded(value: u32) -> Self {
        Self(value)
    }

    pub const fn from_raw(value: u32, key: KeyU32) -> Self {
        let [b1, b2, b3, b4] = value.to_le_bytes();
        let [k1, k2, k3, k4] = key.get();
        let bytes = [
            EncodedU8::from_raw(b1, k1).get(),
            EncodedU8::from_raw(b2, k2).get(),
            EncodedU8::from_raw(b3, k3).get(),
            EncodedU8::from_raw(b4, k4).get(),
        ];
        Self(u32::from_le_bytes(bytes))
    }

    pub const fn decode(self, key: KeyU32) -> u32 {
        let [b1, b2, b3, b4] = self.0.to_le_bytes();
        let [k1, k2, k3, k4] = key.get();
        let bytes = [
            EncodedU8::pre_encoded(b1).decode(k1),
            EncodedU8::pre_encoded(b2).decode(k2),
            EncodedU8::pre_encoded(b3).decode(k3),
            EncodedU8::pre_encoded(b4).decode(k4),
        ];
        u32::from_le_bytes(bytes)
    }

    pub const fn get(&self) -> u32 {
        self.0
    }
}

impl KeyU32 {
    pub const fn new_at_start(start: u8) -> Self {
        let k1 = KeyU8::new(start);
        let k2 = k1.increment();
        let k3 = k2.increment();
        let k4 = k3.increment();
        Self([k1, k2, k3, k4])
    }

    pub const fn from_u32(key: u32) -> Self {
        Self::new(key.to_le_bytes())
    }

    pub const fn new([k1, k2, k3, k4]: [u8; 4]) -> Self {
        Self([
            KeyU8::new(k1),
            KeyU8::new(k2),
            KeyU8::new(k3),
            KeyU8::new(k4),
        ])
    }

    pub const fn get(self) -> [KeyU8; 4] {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{EncodedU32, KeyU32};

    #[rstest::rstest]
    #[case(0, EncodedU32::pre_encoded(u32::from_le_bytes([0x4E, 0x5E, 0x6E, 0x7E])), KeyU32::new([0x4E, 0x5E, 0x6E, 0x7E]))]
    #[case(0, EncodedU32::pre_encoded(u32::from_le_bytes([0x8E, 0x9E, 0xAE, 0xBE])), KeyU32::new([0x8E, 0x9E, 0xAE, 0xBE]))]
    #[case(1, EncodedU32::pre_encoded(u32::from_le_bytes([0x9E, 0x9E, 0xAE, 0xBE])), KeyU32::new([0x8E, 0x9E, 0xAE, 0xBE]))]
    #[case(2304, EncodedU32::pre_encoded(u32::from_le_bytes([0x8E, 0x0E, 0xAE, 0xBE])), KeyU32::new([0x8E, 0x9E, 0xAE, 0xBE]))]
    fn decodes(#[case] expected_raw: u32, #[case] encoded: EncodedU32, #[case] key: KeyU32) {
        let actual_raw = encoded.decode(key);

        assert_eq!(expected_raw, actual_raw);
    }
}
