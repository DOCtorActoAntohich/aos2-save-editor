#[binrw::binrw]
#[derive(Debug, Clone, Copy, PartialEq, Eq, derive_more::TryFrom)]
#[brw(little, repr(u32))]
#[repr(u32)]
pub enum TitleCharacter {
    None = 0x00,
    Sora = 0x01,
    Alte = 0x02,
    Tsih = 0x03,
    Mira = 0x04,
    Sham = 0x05,
    Nath = 0x06,
    StarBreaker = 0x07,
    Suguri = 0x08,
    Saki = 0x09,
    Iru = 0x0a,
    Nanako = 0x0b,
    Kae = 0x0c,
    Kyoko = 0x0d,
    Hime = 0x0e,
    Sumika = 0x0f,
    OjAlte = 0x10,
    OjHime = 0x11,
    OjHimeWinter = 0x12,
    OjKae = 0x13,
    OjKyoko = 0x14,
    OjNanako = 0x15,
    OjNath = 0x16,
    OjSaki = 0x17,
    OjSham = 0x18,
    OjSora = 0x19,
    OjSoraMilitary = 0x1a,
    OjStarBreaker = 0x1b,
    OjSuguri = 0x1c,
    OjSuguriWinter = 0x1d,
    OjIru = 0x1e,
    OjMira = 0x1f,
    DisableTitle = 0xff,
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use binrw::BinRead;

    use super::TitleCharacter;

    #[rstest::rstest]
    fn all_valid_values_are_read() {
        let range = 0x00u8..=0x1fu8;

        for value in range {
            let input_field = [value, 0x00, 0x00, 0x00];
            let mut cursor = Cursor::new(input_field);

            let _value = TitleCharacter::read(&mut cursor).expect("Must read here");
        }
    }
}
