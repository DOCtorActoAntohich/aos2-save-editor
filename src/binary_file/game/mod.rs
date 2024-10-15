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
    pub _0x08: EncodedU32<0x4A, 0x5A, 0x6A, 0x7A>,
    pub _0x0c: EncodedBool<0x8A>,
    pub _0x0d: EncodedBool<0x9A>,
    pub _0x0e: EncodedBool<0xAA>,
    pub _0x0f_unlock_sora: EncodedBool<0xBA>,
    pub _0x10_unlock_alte: EncodedBool<0xCA>,
    pub _0x11_unlock_tsih: EncodedBool<0xDA>,
    pub _0x12_unlock_mira: EncodedBool<0xEA>,
    pub _0x13_unlock_sham: EncodedBool<0xFA>,
    pub _0x14_unlock_nath: EncodedBool<0x0B>,
    pub _0x15_unlock_star_breaker: EncodedBool<0x1B>,
    pub _0x16_unlock_suguri: EncodedBool<0x2B>,
    pub _0x17_unlock_saki: EncodedBool<0x3B>,
    pub _0x18_unlock_iru: EncodedBool<0x4B>,
    pub _0x19_unlock_nanako: EncodedBool<0x5B>,
    pub _0x1a_unlock_kae: EncodedBool<0x6B>,
    pub _0x1b_unlock_kyoko: EncodedBool<0x7B>,
    pub _0x1c_unlock_hime: EncodedBool<0x8B>,
    pub _0x1d_unlock_sumika: EncodedBool<0x9B>,
    pub _0x1e: EncodedBool<0xAB>,
    pub _0x1f: EncodedBool<0xBB>,
    pub _0x20: EncodedBool<0xCB>,
    pub _0x21: EncodedBool<0xDB>,
    pub _0x22: EncodedBool<0xEB>,
    pub _0x23: EncodedBool<0xFB>,
    pub _0x24_unlock_bg_before_the_war: EncodedBool<0x0C>,
    pub _0x25_unlock_bg_war_10k_years_ago: EncodedBool<0x1C>,
    pub _0x26_unlock_bg_canyon_of_wind: EncodedBool<0x2C>,
    pub _0x27_unlock_bg_dust_storm: EncodedBool<0x3C>,
    pub _0x28_unlock_bg_rain_and_sunset: EncodedBool<0x4C>,
    /// Haven't seen this one before
    pub _0x29_unlock_bg_equator_doldrums: EncodedBool<0x5C>,
    pub _0x2a_unlock_bg_big_bridge: EncodedBool<0x6C>,
    pub _0x2b_unlock_bg_capital_in_flames: EncodedBool<0x7C>,
    pub _0x2c_unlock_bg_whirlpool_of_malice: EncodedBool<0x8C>,
    pub _0x2d: EncodedBool<0x9C>,
    pub _0x2e_unlock_bg_nature_10k: EncodedBool<0xAC>,
    pub _0x2f_unlock_bg_crashed_spaceship: EncodedBool<0xBC>,
    pub _0x30_unlock_bg_guardians_chamber: EncodedBool<0xCC>,
    pub _0x31_unlock_bg_moonlight_dance_hall: EncodedBool<0xDC>,
    pub _0x32_unlock_bg_sumika_hideout: EncodedBool<0xEC>,
    pub _0x33: EncodedU8<0xFC>,
    pub _0x34: EncodedBool<0x0D>,
    pub _0x35: EncodedBool<0x1D>,
    pub _0x36: EncodedBool<0x2D>,
    pub _0x37: EncodedBool<0x3D>,
    pub _0x38: EncodedBool<0x4D>,
    pub _0x39: EncodedBool<0x5D>,
    pub _0x3a: EncodedBool<0x6D>,
    pub _0x3b: EncodedBool<0x7D>,
    pub _0x3c: EncodedBool<0x8D>,
    pub _0x3d: EncodedBool<0x9D>,
    pub _0x3e_unlock_bgm_need_for_speed: EncodedBool<0xAD>,
    pub _0x3f_unlock_bgm_black_hole: EncodedBool<0xBD>,
    pub _0x40_unlock_bgm_distant_thunder: EncodedBool<0xCD>,
    pub _0x41_unlock_bgm_swordfish: EncodedBool<0xDD>,
    pub _0x42_unlock_bgm_shine: EncodedBool<0xED>,
    pub _0x43_unlock_bgm_expendables: EncodedBool<0xFD>,
    pub _0x44_unlock_bgm_ribbon: EncodedBool<0x0E>,
    pub _0x45_unlock_bgm_moving_out: EncodedBool<0x1E>,
    pub _0x46_unlock_bgm_accelerator: EncodedBool<0x2E>,
    pub _0x47_unlock_bgm_remember_me: EncodedBool<0x3E>,
    pub _0x48_unlock_bgm_mgom: EncodedBool<0x4E>,
    pub _0x49: EncodedBool<0x5E>,
    pub _0x4a: EncodedBool<0x6E>,
    pub _0x4b: EncodedBool<0x7E>,
    pub _0x4c_singleplayer_wins: EncodedU32<0x8E, 0x9E, 0xAE, 0xBE>,
    pub _0x50_arcade_easy_1ccs: EncodedU32<0xCE, 0xDE, 0xEE, 0xFE>,
    pub _0x54_arcade_medium_1ccs: EncodedU32<0x0F, 0x1F, 0x2F, 0x3F>,
    pub _0x58_arcade_hard_1ccs: EncodedU32<0x4F, 0x5F, 0x6F, 0x7F>,
    pub _0x5c_story_1cc_completions: EncodedU32<0x8F, 0x9F, 0xAF, 0xBF>,
    pub _0x60: EncodedU8<0xCF>,
    pub _0x61: EncodedU8<0xDF>,
    pub _0x62: EncodedU8<0xEF>,
    pub _0x63_is_arcade_easy_1cc_sora: EncodedBool<0xFF>,
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
    pub _0x71_is_arcade_easy_1cc_sumika: EncodedBool<0xD0>,
    pub _0x72: EncodedU8<0xE0>,
    pub _0x73: EncodedU8<0xF0>,
    pub _0x74: EncodedU8<0x01>,
    pub _0x75: EncodedU8<0x11>,
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
    pub _0x84_is_arcade_medium_1cc_sumika: EncodedBool<0x02>,
    pub _0x85: EncodedU8<0x12>,
    pub _0x86: EncodedU8<0x22>,
    pub _0x87: EncodedU8<0x32>,
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
    pub _0x97_is_arcade_hard_1cc_sumika: EncodedBool<0x33>,
    pub _0x98: EncodedU32<0x43, 0x53, 0x63, 0x73>,
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
    pub _0xaa: EncodedU8<0x64>,
    pub _0xab: EncodedU8<0x74>,
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
