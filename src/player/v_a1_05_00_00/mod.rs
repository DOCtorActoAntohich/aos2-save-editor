mod version;

use version::Version;

use super::{
    bin_bool::BinBool,
    sized_string::{
        LobbyNameSection, LobbyPasswordSection, NicknameSection, TitlesSection,
        UnlockableAvatarsSection, UnlockableBackbroundsSection,
    },
};

#[binrw::binrw]
#[derive(Debug)]
#[brw(little)]
pub struct PlayerFile {
    version: Version,
    show_country: BinBool,
    nickname: NicknameSection,
    lobby_name: LobbyNameSection,
    lobby_password: LobbyPasswordSection,
    avatar_character: u32,                                // 0x29
    avatar_background: u32,                               // 0x2d
    unlockable_avatars: UnlockableAvatarsSection,         // start 0x31, 0x21 / 33 bytes
    unlockable_backgrounds: UnlockableBackbroundsSection, // start 0x56, 0x13 / 19 bytes
    title_character_in_background: u32,                   // start 0x6e
    title_text_id: u32,                                   // start 0x71
    titles: TitlesSection,                                // start 0x75, 0x011d / 285 bytes
    show_ingame_title: BinBool,                           // start 0x196
    show_hitstun_meter: BinBool,                          // start 0x197
    show_spectators: BinBool,                             // start 0x198
    title_color: u32,                                     // start 0x199
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use binrw::BinRead;

    use crate::player::PlayerFile;

    #[rstest::fixture]
    fn manually_constructed_player_file() -> Vec<u8> {
        let sections: Vec<Vec<u8>> = vec![
            vec![0xa1u8, 0x05, 0x00, 0x00],    // 0x00-0x03: Version
            vec![0x01],                        // 0x04: Show country
            3u32.to_le_bytes().to_vec(),       // 0x05-0x08: Nickname length
            b"DOC".to_vec(),                   // 0x09-0x0b: Nickname
            21u32.to_le_bytes().to_vec(),      // 0x0c-0x0f: Lobby name length
            b"DOC is dead 3/28/2012".to_vec(), // 0x10-0x24: Lobby name
            4u32.to_le_bytes().to_vec(),       // 0x25-0x28: Password length
            b"hehe".to_vec(),                  // 0x29-0x2c: Password
            vec![0x0e, 0x00, 0x00, 0x00],      // 0x2d-0x30: Avatar Character
            vec![0x13, 0x00, 0x00, 0x00],      // 0x31-0x34: Avatar Backbround
            33u32.to_le_bytes().to_vec(), // 0x35-0x38: Number of unlockable avatars. Must be 0x21/33u
            [0x00; 33].to_vec(),          // 0x39-0x59: Unlockable avatars.
            19u32.to_le_bytes().to_vec(), // 0x5a-0x5d: Number of unlockable backgrounds. Must be 0x13/19u.
            [0x00; 19].to_vec(),          // 0x5e-0x70: Unlockable backgrounds.
            vec![0x0e, 0x00, 0x00, 0x00], // 0x71-0x74: Character in title background.
            vec![0x03, 0x01, 0x00, 0x00], // 0x75-0x78: Title text ID.
            285u32.to_le_bytes().to_vec(), // 0x79-0x7c: Number of unlockable titles. Must be 285 (0x1d_01 little endian).
            [0x00; 285].to_vec(),          // 0x7d-0x199: Ublockable titles.
            vec![0x01],                    // 0x19a: Show in-game title
            vec![0x01],                    // 0x19b: Show hitstun meter
            vec![0x01],                    // 0x19c: Show spectators
            vec![0x01, 0x00, 0x00, 0x00],  // 0x19d-0x1a0: Title color.
        ];

        sections.into_iter().flatten().collect()
    }

    #[rstest::rstest]
    fn it_parses(#[from(manually_constructed_player_file)] player_file: Vec<u8>) {
        let mut cursor = Cursor::new(dbg!(player_file));
        PlayerFile::read(&mut cursor).expect("Must parse");
    }
}
