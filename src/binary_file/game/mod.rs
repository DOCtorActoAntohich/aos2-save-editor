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
    pub _0x14: EncodedU8<0x1B>,
    pub _0x15_unlock_star_breaker: EncodedBool<0x1B>,
    pub _0x16: EncodedU8<0x3B>,
    pub _0x17: EncodedU8<0x2B>,
    pub _0x18: EncodedU32<0x5B, 0x4B, 0x7B, 0x6B>,
    pub _0x1c_unlock_hime: EncodedBool<0x8B>,
    pub _0x1d: EncodedBool<0x9B>,
    pub _0x1e: EncodedBool<0xAB>,
    pub _0x1f: EncodedBool<0x8B>,
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
    pub _0x50_arcade_easy_1ccs: EncodedU32<0xCE, 0xDE, 0xEE, 0xFE>,
    pub _0x54_arcade_medium_1ccs: EncodedU32<0x0F, 0x1F, 0x2F, 0x3F>,
    pub _0x58_arcade_hard_1ccs: EncodedU32<0x4F, 0x5F, 0x6F, 0x7F>,
    pub _0x5c_story_1cc_completions: EncodedU32<0x8F, 0x9F, 0xAF, 0xBF>,
    pub _0x60: u8,
    pub _0x61: u8,
    pub _0x62: u8,
    pub _0x63_is_arcade_easy_1cc_sora: EncodedBool<0xff>,
    pub _0x64_is_arcade_easy_1cc_alte: EncodedBool<0x00>,
    pub _0x65_is_arcade_easy_1cc_tsih: EncodedBool<0x10>,
    pub _0x66_is_arcade_easy_1cc_mira: EncodedBool<0x20>,
    pub _0x67_is_arcade_easy_1cc_sham: EncodedBool<0x30>,
    pub _0x68_is_arcade_easy_1cc_nath: EncodedBool<0x40>,
    pub _0x69_is_arcade_easy_1cc_star_breaker: EncodedBool<0x50>,
    pub _0x6a_is_arcade_easy_1cc_suguri: EncodedBool<0x60>,
    pub _0x6b_is_arcade_easy_1cc_saki: EncodedBool<0x70>,
    pub _0x6c_is_arcade_easy_1cc_iru: EncodedBool<0x80>,
    pub _0x6d_is_arcade_easy_1cc_nanako: EncodedBool<0x90>,
    pub _0x6e_is_arcade_easy_1cc_kae: EncodedBool<0xA0>,
    pub _0x6f_is_arcade_easy_1cc_kyoko: EncodedBool<0xB0>,
    pub _0x70_is_arcade_easy_1cc_hime: EncodedBool<0xC0>,
    pub _0x71: u8,
    pub _0x72: u8,
    pub _0x73: u8,
    pub _0x74: u8,
    pub _0x75: u8,
    pub _0x76_is_arcade_medium_1cc_sora: EncodedBool<0x21>,
    pub _0x77_is_arcade_medium_1cc_alte: EncodedBool<0x31>,
    pub _0x78_is_arcade_medium_1cc_tsih: EncodedBool<0x41>,
    pub _0x79_is_arcade_medium_1cc_mira: EncodedBool<0x51>,
    pub _0x7a_is_arcade_medium_1cc_sham: EncodedBool<0x61>,
    pub _0x7b_is_arcade_medium_1cc_nath: EncodedBool<0x71>,
    pub _0x7c_is_arcade_medium_1cc_star_breaker: EncodedBool<0x81>,
    pub _0x7d_is_arcade_medium_1cc_suguri: EncodedBool<0x91>,
    pub _0x7e_is_arcade_medium_1cc_saki: EncodedBool<0xA1>,
    pub _0x7f_is_arcade_medium_1cc_iru: EncodedBool<0xB1>,
    pub _0x80_is_arcade_medium_1cc_nanako: EncodedBool<0xC1>,
    pub _0x81_is_arcade_medium_1cc_kae: EncodedBool<0xD1>,
    pub _0x82_is_arcade_medium_1cc_kyoko: EncodedBool<0xE1>,
    pub _0x83_is_arcade_medium_1cc_hime: EncodedBool<0xF1>,
    pub _0x84: u32,
    pub _0x88: EncodedBool<0x42>,
    pub _0x89_is_arcade_hard_1cc_sora: EncodedBool<0x52>,
    pub _0x8a_is_arcade_hard_1cc_alte: EncodedBool<0x62>,
    pub _0x8b_is_arcade_hard_1cc_tsih: EncodedBool<0x72>,
    pub _0x8c_is_arcade_hard_1cc_mira: EncodedBool<0x82>,
    pub _0x8d_is_arcade_hard_1cc_sham: EncodedBool<0x92>,
    pub _0x8e_is_arcade_hard_1cc_nath: EncodedBool<0xA2>,
    pub _0x8f_is_arcade_hard_1cc_star_breaker: EncodedBool<0xB2>,
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
    pub _0xa2_is_story_1cc_star_breaker: EncodedBool<0xE3>,
    pub _0xa3_is_story_1cc_suguri: EncodedBool<0xF3>,
    pub _0xa4_is_story_1cc_saki: EncodedBool<0x04>,
    pub _0xa5_is_story_1cc_iru: EncodedBool<0x14>,
    pub _0xa6_is_story_1cc_nanako: EncodedBool<0x24>,
    pub _0xa7_is_story_1cc_kae: EncodedBool<0x34>,
    pub _0xa8_is_story_1cc_kyoko: EncodedBool<0x44>,
    pub _0xa9_is_story_1cc_hime: EncodedBool<0x54>,
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
