#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

pub mod avatar;
pub mod title;
pub mod unlocks;

mod version;

// Re-export;
mod text;

pub use crate::text::lobby_name::LobbyName;
pub use crate::text::lobby_password::LobbyPassword;
pub use crate::text::nickname::Nickname;

use aos2_env::AoS2Env;

use crate::version::Version;

#[binrw::binrw]
#[derive(Debug, Clone, Default)]
#[brw(little)]
pub struct PlayerOnlineProfile {
    pub version: Version,
    pub country: Visibility,
    pub nickname: Nickname,
    pub lobby_name: LobbyName,
    pub lobby_password: LobbyPassword,
    pub avatar_character: avatar::Character,
    pub avatar_background: avatar::Background,
    pub unlockable_avatars: unlocks::AvatarsSection,
    pub unlockable_backgrounds: unlocks::BackgroundsSection,
    pub title_character_in_background: title::Character,
    pub title_text_id: title::Text,
    pub titles: unlocks::TitlesSection,
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

impl PlayerOnlineProfile {
    pub const FILE_NAME: &'static str = "player.rkg";

    pub fn load(env: &AoS2Env) -> Result<Self, binary_file::Error> {
        Self::from_file(env.saves_folder.join(Self::FILE_NAME))
    }

    pub fn save(&self, env: &AoS2Env) -> Result<(), binary_file::Error> {
        self.save_to_file(env.saves_folder.join(Self::FILE_NAME))
    }

    pub fn from_file<P>(path: P) -> Result<Self, binary_file::Error>
    where
        P: AsRef<std::path::Path>,
        for<'a> <Self as binrw::BinRead>::Args<'a>: Default,
    {
        let mut reader = std::fs::File::open(path.as_ref())
            .map_err(|err| binary_file::Error::reading_file(path.as_ref(), err))?;

        <Self as binrw::BinRead>::read(&mut reader)
            .map_err(|err| binary_file::Error::reading_binary(path.as_ref(), err))
    }

    pub fn save_to_file<P>(&self, path: P) -> Result<(), binary_file::Error>
    where
        P: AsRef<std::path::Path>,
        for<'a> <Self as binrw::BinWrite>::Args<'a>: Default,
    {
        let mut writer = std::fs::File::options()
            .create(false)
            .write(true)
            .truncate(true)
            .open(path.as_ref())
            .map_err(|err| binary_file::Error::writing_file(path.as_ref(), err))?;

        <Self as binrw::BinWrite>::write(self, &mut writer)
            .map_err(|err| binary_file::Error::writing_binary(path.as_ref(), err))
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use std::path::PathBuf;

    use binrw::BinRead;

    use super::PlayerOnlineProfile;

    const CRATE_ROOT: &str = env!("CARGO_MANIFEST_DIR");

    #[rstest::rstest]
    fn manually_constructed_player_file() {
        let sections: Vec<Vec<u8>> = vec![
            vec![0xa2u8, 0x05, 0x00, 0x00],    // 0x00-0x03: Version
            vec![0x01],                        // 0x04: Show country
            3u32.to_le_bytes().to_vec(),       // 0x05-0x08: Nickname length
            b"DOC".to_vec(),                   // 0x09-0x0b: Nickname
            21u32.to_le_bytes().to_vec(),      // 0x0c-0x0f: Lobby name length
            b"DOC is dead 3/28/2021".to_vec(), // 0x10-0x24: Lobby name
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

        let bytes: Vec<u8> = sections.into_iter().flatten().collect();
        let mut cursor = Cursor::new(bytes);

        PlayerOnlineProfile::read(&mut cursor).expect("Must parse");
    }

    #[rstest::rstest]
    #[case("player-generic-0.rkg")]
    #[case("player-generic-1.rkg")]
    #[case("player-generic-2.rkg")]
    fn generic_file_from_fs(#[case] file_name: &str) {
        let input_file = PathBuf::from(CRATE_ROOT)
            .join("test_inputs")
            .join(file_name);
        let f = PlayerOnlineProfile::from_file(input_file).expect("Must parse");
        assert_eq!(f.version, crate::version::Version::current());
    }
}
