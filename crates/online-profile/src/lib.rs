#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

pub mod avatar;
pub mod title;

mod sized_section;
mod version;

// Re-export;
mod text;

pub use crate::text::lobby_name::LobbyName;
pub use crate::text::lobby_password::LobbyPassword;
pub use crate::text::nickname::Nickname;

use aos2_env::AoS2Env;

use crate::sized_section::SizedBinarySection;
use crate::version::Version;

pub type UnlockableAvatarsSection = SizedBinarySection<33, 33>;
pub type UnlockableBackbroundsSection = SizedBinarySection<19, 19>;
pub type TitlesSection = SizedBinarySection<285, 285>;

#[binrw::binrw]
#[derive(Debug)]
#[brw(little)]
pub struct PlayerOnlineProfile {
    pub version: Version,
    pub country: Visibility,
    pub nickname: Nickname,
    pub lobby_name: LobbyName,
    pub lobby_password: LobbyPassword,
    pub avatar_character: avatar::Character,
    pub avatar_background: avatar::Background,
    pub unlockable_avatars: UnlockableAvatarsSection,
    pub unlockable_backgrounds: UnlockableBackbroundsSection,
    pub title_character_in_background: title::Character,
    pub title_text_id: title::Text,
    pub titles: TitlesSection,
    pub ingame_title: Visibility,
    pub hitstun_meter: Visibility,
    pub spectators: Visibility,
    pub title_color: title::Color,
}

#[binrw::binrw]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[brw(little)]
pub enum Visibility {
    #[brw(magic = 0x01u8)]
    Show,
    #[default]
    #[brw(magic = 0x00u8)]
    Hide,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to open file for reading")]
    FileRead,
    #[error("Failed to open file for writing")]
    FileWrite,
    #[error("No permission to write to a file")]
    WritePermission,
    #[error("Failed to write binary stream")]
    BinWrite(#[source] binrw::Error),
    #[error("Failed to read binary stream")]
    BinRead(#[source] binrw::Error),
}

impl PlayerOnlineProfile {
    const FILE_NAME: &'static str = "player.rkg";

    pub fn load(env: &AoS2Env) -> Result<Option<Self>, Error> {
        Self::from_file(env.saves_folder.join(Self::FILE_NAME))
    }

    pub fn save(&self, env: &AoS2Env) -> Result<(), Error> {
        self.save_to_file(env.saves_folder.join(Self::FILE_NAME))
    }

    pub fn from_file<P>(path: P) -> Result<Option<Self>, Error>
    where
        P: AsRef<std::path::Path>,
        for<'a> <Self as binrw::BinRead>::Args<'a>: Default,
    {
        let reader = match std::fs::File::open(path) {
            Ok(reader) => Ok(Some(reader)),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(_) => return Err(Error::FileRead),
        }?;

        reader
            .map(|mut reader| <Self as binrw::BinRead>::read(&mut reader).map_err(Error::BinRead))
            .transpose()
    }

    pub fn save_to_file<P>(&self, path: P) -> Result<(), Error>
    where
        P: AsRef<std::path::Path>,
        for<'a> <Self as binrw::BinWrite>::Args<'a>: Default,
    {
        let mut writer = match std::fs::File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
        {
            Ok(writer) => Ok(writer),
            Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied => {
                Err(Error::WritePermission)
            }
            Err(_) => Err(Error::FileWrite),
        }?;

        <Self as binrw::BinWrite>::write(self, &mut writer).map_err(Error::BinWrite)
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
    fn player_file_parses(#[from(manually_constructed_player_file)] player_file: Vec<u8>) {
        let mut cursor = Cursor::new(player_file);
        PlayerOnlineProfile::read(&mut cursor).expect("Must parse");
    }
}
