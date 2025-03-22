#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

// No export
mod xor_encoding;

// Re-export.
mod arenas;
mod lock;
mod music;
mod playable_characters;
mod runs;
mod wins;

pub use self::arenas::Arenas;
pub use self::lock::Status;
pub use self::music::MusicTracks;
pub use self::playable_characters::{Character, PlayableCharacters};
pub use self::runs::{PerfectArcadeMode, PerfectStoryMode, Run};
pub use self::wins::SingleplayerWins;

use std::{io::Cursor, path::Path};

use aos2_env::AoS2Env;
use binrw::{BinRead, BinWrite};

use crate::xor_encoding::{EncodedU8, KeyU8};

pub trait StatusSequence {
    fn toggle_at(&mut self, index: usize);
    fn list(&self) -> Vec<(String, Status)>;
}

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

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to open file for reading")]
    FileRead,
    #[error("Failed to open file for writing")]
    FileWrite,
    #[error("No permission to write to a file")]
    WritePermission,
    #[error("Failed to write a raw encoded stream")]
    EncodedWrite(#[source] binrw::Error),
    #[error("Failed to read a raw encoded stream (invalid file format)")]
    EncodedRead(#[source] binrw::Error),
    #[error("Failed to write intermediate decoded stream")]
    DecodedWrite(#[source] binrw::Error),
    #[error("Failed to read intermediate decoded stream (invalid file format)")]
    DecodedRead(#[source] binrw::Error),
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
/// Here's the table with the decoded values of
/// the least signigicant byte for each game version.
///
/// | Game version | Byte value |
/// |:------------:|:----------:|
/// |  1.6 (demo)  |     135    |
/// |     1.6.2    |     137    |
/// |    1.6.3b    |     138    |
/// |     1.7.6    |     152    |
/// |     1.8.4    |     160    |
/// |      1.9     |     161    |
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
struct EncodedProgress {
    /// Header to which the encoding is not applied.
    header: [u8; Self::HEADER_SIZE],
    /// Encoded body section.
    body: [u8; Self::BODY_SIZE],
}

impl PlayerProgress {
    pub const FILE_NAME: &'static str = "game.sys";

    pub fn from_file(path: impl AsRef<Path>) -> Result<Option<Self>, Error> {
        EncodedProgress::from_file(path)?
            .map(TryInto::try_into)
            .transpose()
    }

    pub fn save_to_file(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        EncodedProgress::try_from(self.clone())?.save_to_file(path)
    }

    pub fn save(&self, env: &AoS2Env) -> Result<(), Error> {
        self.save_to_file(env.saves_folder.join(Self::FILE_NAME))
    }

    pub fn load(env: &AoS2Env) -> Result<Option<Self>, Error> {
        Self::from_file(env.saves_folder.join(Self::FILE_NAME))
    }
}

impl EncodedProgress {
    pub const TOTAL_SIZE: usize = 172;
    pub const HEADER_SIZE: usize = 8;
    pub const BODY_SIZE: usize = Self::TOTAL_SIZE - Self::HEADER_SIZE;
    pub const ENCODING_START_KEY: KeyU8 = KeyU8::new(0x4A);

    pub fn from_file(path: impl AsRef<Path>) -> Result<Option<Self>, Error> {
        let reader = match std::fs::File::open(path) {
            Ok(reader) => Ok(Some(reader)),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(_) => return Err(Error::FileRead),
        }?;

        reader
            .map(|mut reader| <Self as BinRead>::read(&mut reader).map_err(Error::EncodedRead))
            .transpose()
    }

    pub fn save_to_file(&self, path: impl AsRef<Path>) -> Result<(), Error> {
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

        BinWrite::write(self, &mut writer).map_err(Error::EncodedWrite)
    }
}

impl TryFrom<EncodedProgress> for PlayerProgress {
    type Error = Error;

    fn try_from(EncodedProgress { header, body }: EncodedProgress) -> Result<Self, Self::Error> {
        let decoded_body = body.into_iter().enumerate().map(|(index, byte)| {
            let key = EncodedProgress::ENCODING_START_KEY.wrapping_add_usize(index);
            EncodedU8::pre_encoded(byte).decode(key)
        });
        let raw_decoded: Vec<u8> = header.into_iter().chain(decoded_body).collect();

        let mut reader = Cursor::new(raw_decoded);
        BinRead::read(&mut reader).map_err(Error::DecodedRead)
    }
}

impl TryFrom<PlayerProgress> for EncodedProgress {
    type Error = Error;

    fn try_from(value: PlayerProgress) -> Result<Self, Self::Error> {
        let buffer: [u8; Self::TOTAL_SIZE] = {
            let mut writer = Cursor::new([0u8; Self::TOTAL_SIZE]);
            BinWrite::write(&value, &mut writer).map_err(Error::DecodedWrite)?;
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
                let key = Self::ENCODING_START_KEY.wrapping_add_usize(index);
                EncodedU8::from_raw(byte, key).get()
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

    use super::{EncodedProgress, PlayerProgress};

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
    fn progress_file_decode_encode_roundtrip(#[case] expected_savefile: Vec<u8>) {
        let player_progress = {
            let mut reader = Cursor::new(expected_savefile.clone());
            let encoded: EncodedProgress =
                BinRead::read(&mut reader).expect("Must parse the binary file");

            PlayerProgress::try_from(encoded).expect("Must decode the encoded file")
        };

        let encoded_progress: EncodedProgress = player_progress
            .try_into()
            .expect("Must encode successfully");
        let mut writer = Cursor::new(Vec::new());
        BinWrite::write(&encoded_progress, &mut writer).expect("Must write to a buffer");

        assert_eq!(expected_savefile, writer.into_inner());
    }
}
