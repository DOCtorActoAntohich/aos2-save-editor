#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

// No export
mod xor_encryption;

// Re-export.
mod arenas;
mod lock;
mod music;
mod playable_characters;
mod runs;
mod wins;

pub use self::arenas::{Arena, Arenas};
pub use self::lock::Status;
pub use self::music::{MusicTrack, MusicTracks};
pub use self::playable_characters::{Character, PlayableCharacters};
pub use self::runs::{PerfectArcadeMode, PerfectStoryMode, Run};
pub use self::wins::SingleplayerWins;

use std::{io::Cursor, path::Path};

use aos2_env::AoS2Env;
use binrw::{BinRead, BinWrite};

use crate::xor_encryption::{EncryptedU8, KeyU8};

/// Player progress file, aka `game.sys`.
///
/// Order of fields MATTERS. Do NOT reorder.
#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[brw(little)]
pub struct PlayerProgress {
    _0x00: UnknownU8,
    _0x01: UnknownU8,
    _0x02: UnknownU8,
    _0x03: UnknownU8,
    /// Offset: 0x04 - 0x07.
    _body_length: BodyLength,
    /// Offset: 0x08 - 0x0b
    _version: Version,
    _0x0c: UnknownU8,
    _0x0d: UnknownU8,
    _0x0e: UnknownU8,
    /// Remembers if a character is unlocked.
    ///
    /// It's possible to disable characters unlocked by default.
    ///
    /// Offset: 0x0f - 0x1d.
    pub playable_characters: PlayableCharacters,
    _0x1e: UnknownU8,
    _0x1f: UnknownU8,
    _0x20: UnknownU8,
    _0x21: UnknownU8,
    _0x22: UnknownU8,
    _0x23: UnknownU8,
    /// Remembers if the background image is unlocked.
    ///
    /// Offset: 0x24 - 0x32.
    pub arenas: Arenas,
    _0x33: UnknownU8,
    _0x34: UnknownU8,
    _0x35: UnknownU8,
    _0x36: UnknownU8,
    _0x37: UnknownU8,
    _0x38: UnknownU8,
    _0x39: UnknownU8,
    _0x3a: UnknownU8,
    _0x3b: UnknownU8,
    _0x3c: UnknownU8,
    _0x3d: UnknownU8,
    /// Remembers if the background music is unlocked.
    ///
    /// Offset: 0x3e - 0x48.
    pub music_tracks: MusicTracks,
    _0x49: UnknownU8,
    _0x4a: UnknownU8,
    _0x4b: UnknownU8,
    /// Offset: 0x4c - 0x5f.
    pub wins: SingleplayerWins,
    _0x60: UnknownU8,
    _0x61: UnknownU8,
    _0x62: UnknownU8,
    /// Remembers if a character 1CC'ed Arcade mode on Easy difficulty.
    ///
    /// In game, it shows a star next to the character's portrait.
    ///
    /// Save file offset: 0x63 - 0x71.
    pub arcade_easy_1ccs: PerfectArcadeMode,
    _0x72: UnknownU8,
    _0x73: UnknownU8,
    _0x74: UnknownU8,
    _0x75: UnknownU8,
    /// Remembers if a character 1CC'ed Arcade mode on Medium difficulty.
    ///
    /// In game, it shows a star next to the character's portrait.
    ///
    /// Save file offset: 0x76 - 0x84.
    pub arcade_medium_1ccs: PerfectArcadeMode,
    _0x85: UnknownU8,
    _0x86: UnknownU8,
    _0x87: UnknownU8,
    _0x88: UnknownU8,
    /// Remembers if a character 1CC'ed Arcade mode on Hard difficulty.
    ///
    /// In game, it shows a star next to the character's portrait.
    ///
    /// Save file offset: 0x89 - 0x97.
    pub arcade_hard_1ccs: PerfectArcadeMode,
    _0x98: UnknownU8,
    _0x99: UnknownU8,
    _0x9a: UnknownU8,
    _0x9b: UnknownU8,
    /// Remembers if a character 1CC'ed Story mode on any difficulty.
    ///
    /// In game, it shows a star next to the character's portrait.
    ///
    /// Save file offset: 0x9c - 0xa9.
    pub story_1ccs: PerfectStoryMode,
    _0xaa: UnknownU8,
    _0xab: UnknownU8,
}

/// Means the purpose of the field is unknown.
///
/// "Explicit is better than implicit".
#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[brw(little)]
struct UnknownU8(u8);

/// Somehow it just matches.
#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[brw(little)]
#[br(assert(self_0 == Self::BYTES))]
struct BodyLength(u32);

impl BodyLength {
    const BYTES: u32 = 164;
}

/// Reprents savefile version, parsed by a specific game version.
///
/// Here's the table with the values of
/// the least signigicant byte for each game version.
///
/// | Game version | Byte value | Hex value |
/// |:------------:|:----------:|:---------:|
/// |  1.6 (demo)  |     135    |   0x87    |
/// |     1.6.2    |     137    |   0x89    |
/// |    1.6.3b    |     138    |   0x8A    |
/// |     1.7.6    |     152    |   0x98    |
/// |     1.8.4    |     160    |   0xA0    |
/// |      1.9     |     161    |   0xA1    |
///
/// Therefore, version is the most probable explanation,
/// because in a fresh savefile all other values are the same,
/// except for the one at `0x08`.
#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[brw(little)]
struct Version(u32);

#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq)]
#[brw(little)]
struct EncryptedProgress {
    /// Header to which the encryption is not applied.
    header: [u8; Self::HEADER_SIZE],
    /// Encrypted body section.
    body: [u8; Self::BODY_SIZE],
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
struct EncryptionError(#[from] binrw::Error);

impl PlayerProgress {
    pub const FILE_NAME: &'static str = "game.sys";

    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, binary_file::Error> {
        EncryptedProgress::from_file(path.as_ref())?
            .try_into()
            .map_err(|EncryptionError(err)| binary_file::Error::reading_binary(path.as_ref(), err))
    }

    pub fn save_to_file(&self, path: impl AsRef<Path>) -> Result<(), binary_file::Error> {
        EncryptedProgress::try_from(self.clone())
            .map_err(|EncryptionError(err)| binary_file::Error::writing_binary(path.as_ref(), err))?
            .save_to_file(path.as_ref())
    }

    pub fn save(&self, env: &AoS2Env) -> Result<(), binary_file::Error> {
        self.save_to_file(env.saves_folder.join(Self::FILE_NAME))
    }

    pub fn load(env: &AoS2Env) -> Result<Self, binary_file::Error> {
        Self::from_file(env.saves_folder.join(Self::FILE_NAME))
    }
}

impl EncryptedProgress {
    pub const TOTAL_SIZE: usize = 172;
    pub const HEADER_SIZE: usize = 8;
    pub const BODY_SIZE: usize = Self::TOTAL_SIZE - Self::HEADER_SIZE;
    pub const ENCRYPTION_START_KEY: KeyU8 = KeyU8::new(0x4A);

    pub fn from_file(path: &Path) -> Result<Self, binary_file::Error> {
        let mut reader =
            std::fs::File::open(path).map_err(|err| binary_file::Error::reading_file(path, err))?;

        <Self as BinRead>::read(&mut reader)
            .map_err(|err| binary_file::Error::reading_binary(path, err))
    }

    pub fn save_to_file(&self, path: &Path) -> Result<(), binary_file::Error> {
        let mut writer = std::fs::File::options()
            .create(false)
            .write(true)
            .truncate(true)
            .open(path)
            .map_err(|err| binary_file::Error::writing_file(path, err))?;

        BinWrite::write(self, &mut writer)
            .map_err(|err| binary_file::Error::writing_binary(path, err))
    }
}

impl TryFrom<EncryptedProgress> for PlayerProgress {
    type Error = EncryptionError;

    fn try_from(
        EncryptedProgress { header, body }: EncryptedProgress,
    ) -> Result<Self, Self::Error> {
        let decrypted_body = body.into_iter().enumerate().map(|(index, byte)| {
            let key = EncryptedProgress::ENCRYPTION_START_KEY.wrapping_add_usize(index);
            EncryptedU8::encrypted(byte).decrypt(key)
        });
        let raw_decrypted: Vec<u8> = header.into_iter().chain(decrypted_body).collect();

        let mut reader = Cursor::new(raw_decrypted);
        BinRead::read(&mut reader).map_err(EncryptionError)
    }
}

impl TryFrom<PlayerProgress> for EncryptedProgress {
    type Error = EncryptionError;

    fn try_from(value: PlayerProgress) -> Result<Self, Self::Error> {
        let buffer: [u8; Self::TOTAL_SIZE] = {
            let mut writer = Cursor::new([0u8; Self::TOTAL_SIZE]);
            BinWrite::write(&value, &mut writer).map_err(EncryptionError)?;
            writer.into_inner()
        };

        let header: [u8; Self::HEADER_SIZE] =
            TryInto::<&[u8; Self::HEADER_SIZE]>::try_into(&buffer[..Self::HEADER_SIZE])
                .expect("Invariant: Arrays of fixed and known size must match")
                .to_owned();

        let body: [u8; Self::BODY_SIZE] = buffer[Self::HEADER_SIZE..]
            .iter()
            .enumerate()
            .map(|(index, &byte)| {
                let key = Self::ENCRYPTION_START_KEY.wrapping_add_usize(index);
                EncryptedU8::encrypt(byte, key).get()
            })
            .collect::<Vec<u8>>()
            .try_into()
            .expect("Invariant: Arrays of fixed and known size must match");

        Ok(Self { header, body })
    }
}

#[cfg(test)]
mod tests {
    use std::{io::Cursor, path::PathBuf};

    use binrw::{BinRead, BinWrite};

    use super::{EncryptedProgress, PlayerProgress};

    const CARGO_TOML: &str = env!("CARGO_MANIFEST_DIR");

    #[rstest::fixture]
    fn fresh_savefile() -> Vec<u8> {
        let path = PathBuf::from(CARGO_TOML).join("test_inputs/game-fresh.sys");
        std::fs::read(path).expect("Precondition: must read test input file")
    }

    #[rstest::fixture]
    fn completionist_savefile() -> Vec<u8> {
        let path = PathBuf::from(CARGO_TOML).join("test_inputs/game-completionist.sys");
        std::fs::read(path).expect("Precondition: must read test input file")
    }

    #[rstest::rstest]
    #[case::fresh(fresh_savefile())]
    #[case::lots_of_stuff_unlocked(completionist_savefile())]
    fn progress_file_decrypt_encrypt_roundtrip(#[case] expected_savefile: Vec<u8>) {
        let player_progress = {
            let mut reader = Cursor::new(expected_savefile.clone());
            let encrypted: EncryptedProgress =
                BinRead::read(&mut reader).expect("Must parse the binary file");

            PlayerProgress::try_from(encrypted).expect("Must decrypt the encrypted file")
        };

        let encrypted_progress: EncryptedProgress = player_progress
            .try_into()
            .expect("Must encrypt successfully");
        let mut writer = Cursor::new(Vec::new());
        BinWrite::write(&encrypted_progress, &mut writer).expect("Must write to a buffer");

        assert_eq!(expected_savefile, writer.into_inner());
    }
}
