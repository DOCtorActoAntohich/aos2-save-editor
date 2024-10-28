mod background;
mod characters;

pub use self::{
    background::{image::BackgroundImageSheet, music::BackgroundMusicSheet},
    characters::{full::FullCharacterSheet, story::StoryCharacterSheet},
};

use std::{io::Cursor, path::Path};

use anyhow::Context;
use binrw::{BinRead, BinWrite};

use crate::xor_encoding::u8::{EncodedU8, KeyU8};

/// Player progress file, aka `game.sys`.
///
/// Order of fields MATTERS. Do NOT reorder.
#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq)]
#[brw(little)]
pub struct PlayerProgress {
    _0x00: UnknownU32,
    _0x04: UnknownU32,
    _0x08: UnknownU32,
    _0x0c: UnknownU8,
    _0x0d: UnknownU8,
    _0x0e: UnknownU8,
    /// Remembers if a character is unlocked.
    ///
    /// It's possible to disable characters unlocked by default.
    ///
    /// Offset: 0x0f - 0x1d.
    pub playable_character: FullCharacterSheet,
    _0x1e: UnknownU8,
    _0x1f: UnknownU8,
    _0x20: UnknownU8,
    _0x21: UnknownU8,
    _0x22: UnknownU8,
    _0x23: UnknownU8,
    /// Remembers if the background image is unlocked.
    ///
    /// Offset: 0x24 - 0x32.
    pub background_image: BackgroundImageSheet,
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
    pub unlocked_background_music: BackgroundMusicSheet,
    _0x49: UnknownU8,
    _0x4a: UnknownU8,
    _0x4b: UnknownU8,
    /// A total number of singleplayer wins for any character on any difficulty.
    ///
    /// Simple winning one match in story/arcade mode is enough to increase the counter.
    ///
    /// Offset: 0x4c - 0x4f.
    pub n_singleplayer_match_wins: u32,
    /// A total number of Arcade 1CC's on Easy difficulty for any character.
    ///
    /// Offset: 0x50 - 0x53.
    pub n_arcade_easy_1cc_completions: u32,
    /// A total number of Arcade 1CC's on Medium difficulty for any character.
    ///
    /// Offset: 0x54 - 0x57.
    pub n_arcade_medium_1cc_completions: u32,
    /// A total number of Arcade 1CC's on Hard difficulty for any character.
    ///
    /// Offset: 0x58 - 0x5b.
    pub n_arcade_hard_1cc_completions: u32,
    /// A total number of Story 1CC's on any difficulty for any character.
    ///
    /// Offset: 0x5c - 0x5f.
    pub n_story_1cc_completions: u32,
    _0x60: UnknownU8,
    _0x61: UnknownU8,
    _0x62: UnknownU8,
    /// Remembers if a character 1CC'ed Arcade mode on Easy difficulty.
    ///
    /// In game, it shows a star next to the character's portrait.
    ///
    /// Save file offset: 0x63 - 0x71.
    pub arcade_easy_1cc_by_character: FullCharacterSheet,
    _0x72: UnknownU8,
    _0x73: UnknownU8,
    _0x74: UnknownU8,
    _0x75: UnknownU8,
    /// Remembers if a character 1CC'ed Arcade mode on Medium difficulty.
    ///
    /// In game, it shows a star next to the character's portrait.
    ///
    /// Save file offset: 0x76 - 0x84.
    pub arcade_medium_1cc_by_character: FullCharacterSheet,
    _0x85: UnknownU8,
    _0x86: UnknownU8,
    _0x87: UnknownU8,
    _0x88: UnknownU8,
    /// Remembers if a character 1CC'ed Arcade mode on Hard difficulty.
    ///
    /// In game, it shows a star next to the character's portrait.
    ///
    /// Save file offset: 0x89 - 0x97.
    pub arcade_hard_1cc_by_character: FullCharacterSheet,
    _0x98: UnknownU8,
    _0x99: UnknownU8,
    _0x9a: UnknownU8,
    _0x9b: UnknownU8,
    /// Remembers if a character 1CC'ed Story mode on any difficulty.
    ///
    /// In game, it shows a star next to the character's portrait.
    ///
    /// Save file offset: 0x9c - 0xa9.
    pub story_1cc_by_character: StoryCharacterSheet,
    _0xaa: UnknownU8,
    _0xab: UnknownU8,
}

/// This basically means "No idea what it does".
///
/// Marked separately because explicit is better.
#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq)]
#[brw(little)]
struct UnknownU32(u32);

/// Same as [`UnknownU32`].
#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[brw(little)]
struct UnknownU8(u8);

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
    pub fn from_file(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        EncodedProgress::from_file(path)?.try_into()
    }

    pub fn save_to_file(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        EncodedProgress::try_from(self.clone())?.save_to_file(path)
    }
}

impl EncodedProgress {
    pub const TOTAL_SIZE: usize = 172;
    pub const HEADER_SIZE: usize = 8;
    pub const BODY_SIZE: usize = Self::TOTAL_SIZE - Self::HEADER_SIZE;
    pub const ENCODING_START_KEY: KeyU8 = KeyU8::new(0x4A);

    pub fn from_file(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let mut reader = std::fs::File::open(path).context("Couldn't open file")?;
        BinRead::read(&mut reader).context("Failed to read the encoded file")
    }

    pub fn save_to_file(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let mut writer = std::fs::File::options()
            .write(true)
            .truncate(true)
            .open(path)
            .context("Failed to open file for writing")?;
        BinWrite::write(self, &mut writer).context("Failed to overwrite the file")
    }
}

impl TryFrom<EncodedProgress> for PlayerProgress {
    type Error = anyhow::Error;

    fn try_from(EncodedProgress { header, body }: EncodedProgress) -> Result<Self, Self::Error> {
        let decoded_body = body.into_iter().enumerate().map(|(index, byte)| {
            let key = EncodedProgress::ENCODING_START_KEY.wrapping_add(index as u8);
            EncodedU8::pre_encoded(byte).decode(key)
        });
        let raw_decoded: Vec<u8> = header.into_iter().chain(decoded_body).collect();

        let mut reader = Cursor::new(raw_decoded);
        BinRead::read(&mut reader).context("Failed to parse a decoded file")
    }
}

impl TryFrom<PlayerProgress> for EncodedProgress {
    type Error = anyhow::Error;

    fn try_from(value: PlayerProgress) -> Result<Self, Self::Error> {
        let buffer = {
            let mut writer = Cursor::new(Vec::new());
            BinWrite::write(&value, &mut writer)
                .context("Failed to write the savefile to a binary buffer")?;
            writer.into_inner()
        };

        if buffer.len() != Self::TOTAL_SIZE {
            return Err(anyhow::anyhow!(
                "Buffer size must be {}, not {}",
                Self::TOTAL_SIZE,
                buffer.len()
            ));
        }

        let mut iter = buffer.into_iter();
        let header: [u8; Self::HEADER_SIZE] = iter
            .by_ref()
            .take(Self::HEADER_SIZE)
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| {
                anyhow::anyhow!("Failed to convert a header into a buffer of fixed size")
            })?;

        let body: [u8; Self::BODY_SIZE] = iter
            .enumerate()
            .map(|(index, byte)| {
                let key = Self::ENCODING_START_KEY.wrapping_add(index as u8);
                EncodedU8::from_raw(byte, key).get()
            })
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| {
                anyhow::anyhow!("Failed to convert an encoded body into a buffer of fixed size")
            })?;

        Ok(Self { header, body })
    }
}

#[cfg(test)]
mod tests {
    use std::{io::Cursor, path::PathBuf};

    use binrw::{BinRead, BinWrite};

    use crate::file::game::{EncodedProgress, PlayerProgress};

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
