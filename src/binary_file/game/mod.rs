pub mod encoded_bool;
pub mod encoded_u32;
pub mod encoded_u8;

use std::path::Path;

use encoded_bool::EncodedBool;
use encoded_u32::EncodedU32;
use encoded_u8::EncodedU8;

use super::{savefile::GameBinarySaveFile, sized_section::SizedBinarySection};

pub struct GameSysFile {
    pub singleplayer_mode_wins: u32,
}

#[binrw::binrw]
#[derive(Debug)]
#[brw(little)]
pub struct GameSysBinaryFile {
    _looks_like_version: Version,
    pub content: SizedBinarySection<0xa4, 0xa4>,
}

#[binrw::binrw]
#[derive(Debug)]
#[brw(little)]
pub struct GameSysBinaryFile2 {
    _0x00_version: Version,
    _0x04_size: Size,
    pub _0x08: u32,
    pub _0x0c: u32,
    pub _0x10: u32,
    pub _0x14: u32,
    pub _0x18: EncodedU32<0x5B, 0x4B, 0x7B, 0x6B>,
    pub _0x1c_unlock_hime: EncodedU32<0x8B, 0x9B, 0xAB, 0xBB>,
    pub _0x20: EncodedU32<0xCB, 0xDB, 0xEB, 0xFB>,
    pub _0x24: u32,
    pub _0x28: u32,
    pub _0x2c: u32,
    pub _0x30: u32,
    pub _0x34: u32,
    pub _0x38: u32,
    pub _0x3c: u32,
    pub _0x40: u32,
    pub _0x44: u32,
    pub _0x48: u32,
    pub _0x4c_singleplayer_wins: EncodedU32<0x8E, 0x9E, 0xAE, 0xBE>,
    pub _0x50_arcade_easy_1ccs_unsure: EncodedU32<0xCE, 0xDE, 0xEE, 0xFE>,
    pub _0x54: u32,
    pub _0x58_arcade_hard_1ccs_unsure: EncodedU32<0x4F, 0x5F, 0x6F, 0x7F>,
    pub _0x5c_story_1cc_completions: EncodedU32<0x8F, 0x9F, 0xAF, 0xBF>,
    pub _0x60: u32,
    pub _0x64: u32,
    pub _0x68: u8,
    pub _0x69: u8,
    pub _0x6a_is_arcade_easy_1cc_suguri: EncodedBool<0x60>,
    pub _0x6b: u8,
    pub _0x6c: u32,
    pub _0x70: u32,
    pub _0x74: u32,
    pub _0x78: u32,
    pub _0x7c: u32,
    pub _0x80: u32,
    pub _0x84: u32,
    pub _0x88: u32,
    pub _0x8c: u32,
    pub _0x90_is_arcade_hard_1cc_suguri: EncodedBool<0xC2>,
    pub _0x91_is_arcade_hard_1cc_saki: EncodedBool<0xD2>,
    pub _0x92_is_arcade_hard_1cc_iru: EncodedBool<0xE2>,
    pub _0x93_is_arcade_hard_1cc_nanako: EncodedBool<0xF2>,
    pub _0x94_is_arcade_hard_1cc_kae: EncodedBool<0x03>,
    pub _0x95_is_arcade_hard_1cc_kyoko: EncodedBool<0x13>,
    pub _0x96_is_arcade_hard_1cc_hime: EncodedBool<0x23>,
    pub _0x97: EncodedBool<0x33>,
    pub _0x98: u32,
    pub _0x9c_is_story_1cc_sora: EncodedBool<0x83>,
    pub _0x9d_is_story_1cc_alte: EncodedBool<0x93>,
    pub _0x9e_is_story_1cc_tsih: EncodedBool<0xA3>,
    pub _0x9f_is_story_1cc_mira: EncodedBool<0xB3>,
    pub _0xa0_is_story_1cc_sham: EncodedBool<0xC3>,
    pub _0xa1_is_story_1cc_nath: EncodedBool<0xD3>,
    pub _0xa2: u8,
    pub _0xa3_is_story_1cc_suguri: EncodedBool<0xF3>,
    pub _0xa4_is_story_1cc_saki: EncodedBool<0x04>,
    pub _0xa5_is_story_1cc_iru: EncodedBool<0x14>,
    pub _0xa6_is_story_1cc_nanako: EncodedBool<0x24>,
    pub _0xa7_is_story_1cc_kae: EncodedBool<0x34>,
    pub _0xa8_is_story_1cc_kyoko: EncodedBool<0x44>,
    pub _0xa9: u8,
    pub _0xaa: u8,
    pub _0xab: u8,
}

#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq, Hash, derive_more::Display)]
#[brw(magic = b"\x4C\x53\x44\x00")]
struct Version;

#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq, Hash, derive_more::Display)]
#[brw(magic = b"\xA4\x00\x00\x00")]
struct Size;

impl GameBinarySaveFile for GameSysBinaryFile {}

impl GameBinarySaveFile for GameSysBinaryFile2 {}

impl GameSysFile {
    pub fn from_file<P>(path: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        GameSysBinaryFile2::from_file(path).map(Into::into)
    }
}

impl From<GameSysBinaryFile2> for GameSysFile {
    fn from(value: GameSysBinaryFile2) -> Self {
        dbg!(value);
        todo!()
    }
}
