#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

pub mod avatar;
pub mod bin_bool;
pub mod sized_section;
pub mod title;

mod version;

use anyhow::Context;
use aos2_env::AoS2Env;

use crate::{bin_bool::BinBool, sized_section::SizedBinarySection};

use self::version::Version;

pub type NicknameSection = SizedBinarySection<1, 16>;
pub type LobbyNameSection = SizedBinarySection<1, 24>;
pub type LobbyPasswordSection = SizedBinarySection<0, 24>;
pub type UnlockableAvatarsSection = SizedBinarySection<33, 33>;
pub type UnlockableBackbroundsSection = SizedBinarySection<19, 19>;
pub type TitlesSection = SizedBinarySection<285, 285>;

#[binrw::binrw]
#[derive(Debug)]
#[brw(little)]
pub struct PlayerOnlineProfile {
    pub version: Version,
    pub show_country: BinBool,
    pub nickname: NicknameSection,
    pub lobby_name: LobbyNameSection,
    pub lobby_password: LobbyPasswordSection,
    pub avatar_character: avatar::Character,
    pub avatar_background: avatar::Background,
    pub unlockable_avatars: UnlockableAvatarsSection,
    pub unlockable_backgrounds: UnlockableBackbroundsSection,
    pub title_character_in_background: title::Character,
    pub title_text_id: title::Text,
    pub titles: TitlesSection,
    pub show_ingame_title: BinBool,
    pub show_hitstun_meter: BinBool,
    pub show_spectators: BinBool,
    pub title_color: title::Color,
}

impl PlayerOnlineProfile {
    const FILE_NAME: &'static str = "player.rkg";

    pub fn load(env: &AoS2Env) -> anyhow::Result<Self> {
        Self::from_file(env.saves_folder.join(Self::FILE_NAME))
    }

    pub fn save(&self, env: &AoS2Env) -> anyhow::Result<()> {
        self.save_to_file(env.saves_folder.join(Self::FILE_NAME))
    }

    pub fn from_file<P>(path: P) -> anyhow::Result<Self>
    where
        P: AsRef<std::path::Path>,
        for<'a> <Self as binrw::BinRead>::Args<'a>: Default,
    {
        let mut reader = std::fs::File::open(path).context("Failed to open file")?;

        binrw::BinRead::read(&mut reader).context("Failed to parse file")
    }

    pub fn save_to_file<P>(&self, path: P) -> anyhow::Result<()>
    where
        P: AsRef<std::path::Path>,
        for<'a> <Self as binrw::BinWrite>::Args<'a>: Default,
    {
        let mut writer = std::fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path)
            .context("Failed to create or open the file for writing")?;

        binrw::BinWrite::write(self, &mut writer).context("Failed to overwrite file contents")
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use binrw::BinRead;

    use super::PlayerOnlineProfile;

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
            vec![0x13, 0x00, 0x00, 0x00],      // 0x31-0x34: Avatar Background
            33u32.to_le_bytes().to_vec(), // 0x35-0x38: Number of unlockable avatars. Must be 0x21/33u
            [0x00; 33].to_vec(),          // 0x39-0x59: Unlockable avatars.
            19u32.to_le_bytes().to_vec(), // 0x5a-0x5d: Number of unlockable backgrounds. Must be 0x13/19u.
            [0x00; 19].to_vec(),          // 0x5e-0x70: Unlockable backgrounds.
            vec![0x0e, 0x00, 0x00, 0x00], // 0x71-0x74: Character in title background.
            vec![0x03, 0x01, 0x00, 0x00], // 0x75-0x78: Title text ID.
            285u32.to_le_bytes().to_vec(), // 0x79-0x7c: Number of unlockable titles. Must be 285 (0x1d_01 little endian).
            [0x00; 285].to_vec(),          // 0x7d-0x199: Unlockable titles.
            vec![0x01],                    // 0x19a: Show in-game title
            vec![0x01],                    // 0x19b: Show hitstun meter
            vec![0x01],                    // 0x19c: Show spectators
            vec![0x01, 0x00, 0x00, 0x00],  // 0x19d-0x1a0: Title color.
        ];

        sections.into_iter().flatten().collect()
    }

    #[rstest::rstest]
    fn it_parses(#[from(manually_constructed_player_file)] player_file: Vec<u8>) {
        let mut cursor = Cursor::new(player_file);
        PlayerOnlineProfile::read(&mut cursor).expect("Must parse");
    }
}
