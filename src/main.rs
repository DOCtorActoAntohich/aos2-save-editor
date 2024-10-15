use aos2_env::AoS2Paths;
use aos2_save_editor::binary_file::{
    game::{
        encoded_bool::EncodedBool, encoded_u32::EncodedU32, encoded_u8::EncodedU8,
        GameSysBinaryFile2,
    },
    player::PlayerFile,
    savefile::GameBinarySaveFile,
};

fn main() -> anyhow::Result<()> {
    let settings = AoS2Paths::from_env()?;

    let player_file = PlayerFile::from_file(&settings.player_rkg)?;
    player_file.save(&settings.player_rkg)?;

    let mut game_sys = GameSysBinaryFile2::from_file(&settings.game_sys)?;

    game_sys._0x0f_unlock_sora = EncodedBool::encode_from_raw(true);
    game_sys._0x10_unlock_alte = EncodedBool::encode_from_raw(false);
    game_sys._0x11_unlock_tsih = EncodedBool::encode_from_raw(false);
    game_sys._0x12_unlock_mira = EncodedBool::encode_from_raw(false);
    game_sys._0x13_unlock_sham = EncodedBool::encode_from_raw(false);
    game_sys._0x14_unlock_nath = EncodedBool::encode_from_raw(false);
    game_sys._0x15_unlock_star_breaker = EncodedBool::encode_from_raw(false);
    game_sys._0x16_unlock_suguri = EncodedBool::encode_from_raw(true);
    game_sys._0x17_unlock_saki = EncodedBool::encode_from_raw(false);
    game_sys._0x18_unlock_iru = EncodedBool::encode_from_raw(false);
    game_sys._0x19_unlock_nanako = EncodedBool::encode_from_raw(false);
    game_sys._0x1a_unlock_kae = EncodedBool::encode_from_raw(false);
    game_sys._0x1b_unlock_kyoko = EncodedBool::encode_from_raw(false);
    game_sys._0x1c_unlock_hime = EncodedBool::encode_from_raw(true);
    game_sys._0x1d_unlock_sumika = EncodedBool::encode_from_raw(false);

    game_sys._0x4c_singleplayer_wins = EncodedU32::encode_from_raw(142536);
    game_sys._0x50_arcade_easy_1ccs = EncodedU32::encode_from_raw(1);
    game_sys._0x54_arcade_medium_1ccs = EncodedU32::encode_from_raw(2);
    game_sys._0x58_arcade_hard_1ccs = EncodedU32::encode_from_raw(3);
    game_sys._0x5c_story_1cc_completions = EncodedU32::encode_from_raw(4);

    game_sys._0x9c_is_story_1cc_sora = EncodedBool::encode_from_raw(true);
    game_sys._0x9d_is_story_1cc_alte = EncodedBool::encode_from_raw(true);
    game_sys._0x9e_is_story_1cc_tsih = EncodedBool::encode_from_raw(true);
    game_sys._0x9f_is_story_1cc_mira = EncodedBool::encode_from_raw(true);
    game_sys._0xa0_is_story_1cc_sham = EncodedBool::encode_from_raw(true);
    game_sys._0xa1_is_story_1cc_nath = EncodedBool::encode_from_raw(true);
    game_sys._0xa2_is_story_1cc_star_breaker = EncodedBool::encode_from_raw(true);
    game_sys._0xa3_is_story_1cc_suguri = EncodedBool::encode_from_raw(true);
    game_sys._0xa4_is_story_1cc_saki = EncodedBool::encode_from_raw(true);
    game_sys._0xa5_is_story_1cc_iru = EncodedBool::encode_from_raw(true);
    game_sys._0xa6_is_story_1cc_nanako = EncodedBool::encode_from_raw(true);
    game_sys._0xa7_is_story_1cc_kae = EncodedBool::encode_from_raw(true);
    game_sys._0xa8_is_story_1cc_kyoko = EncodedBool::encode_from_raw(true);
    game_sys._0xa9_is_story_1cc_hime = EncodedBool::encode_from_raw(true);

    game_sys._0x63_is_arcade_easy_1cc_sora = EncodedBool::encode_from_raw(true);
    game_sys._0x64_is_arcade_easy_1cc_alte = EncodedBool::encode_from_raw(true);
    game_sys._0x65_is_arcade_easy_1cc_tsih = EncodedBool::encode_from_raw(true);
    game_sys._0x66_is_arcade_easy_1cc_mira = EncodedBool::encode_from_raw(true);
    game_sys._0x67_is_arcade_easy_1cc_sham = EncodedBool::encode_from_raw(true);
    game_sys._0x68_is_arcade_easy_1cc_nath = EncodedBool::encode_from_raw(true);
    game_sys._0x69_is_arcade_easy_1cc_star_breaker = EncodedBool::encode_from_raw(true);
    game_sys._0x6a_is_arcade_easy_1cc_suguri = EncodedBool::encode_from_raw(true);
    game_sys._0x6b_is_arcade_easy_1cc_saki = EncodedBool::encode_from_raw(true);
    game_sys._0x6c_is_arcade_easy_1cc_iru = EncodedBool::encode_from_raw(true);
    game_sys._0x6d_is_arcade_easy_1cc_nanako = EncodedBool::encode_from_raw(true);
    game_sys._0x6e_is_arcade_easy_1cc_kae = EncodedBool::encode_from_raw(true);
    game_sys._0x6f_is_arcade_easy_1cc_kyoko = EncodedBool::encode_from_raw(true);
    game_sys._0x70_is_arcade_easy_1cc_hime = EncodedBool::encode_from_raw(true);
    game_sys._0x71_is_arcade_easy_1cc_sumika = EncodedBool::encode_from_raw(true);

    game_sys._0x76_is_arcade_medium_1cc_sora = EncodedBool::encode_from_raw(true);
    game_sys._0x77_is_arcade_medium_1cc_alte = EncodedBool::encode_from_raw(true);
    game_sys._0x78_is_arcade_medium_1cc_tsih = EncodedBool::encode_from_raw(true);
    game_sys._0x79_is_arcade_medium_1cc_mira = EncodedBool::encode_from_raw(true);
    game_sys._0x7a_is_arcade_medium_1cc_sham = EncodedBool::encode_from_raw(true);
    game_sys._0x7b_is_arcade_medium_1cc_nath = EncodedBool::encode_from_raw(true);
    game_sys._0x7c_is_arcade_medium_1cc_star_breaker = EncodedBool::encode_from_raw(true);
    game_sys._0x7d_is_arcade_medium_1cc_suguri = EncodedBool::encode_from_raw(true);
    game_sys._0x7e_is_arcade_medium_1cc_saki = EncodedBool::encode_from_raw(true);
    game_sys._0x7f_is_arcade_medium_1cc_iru = EncodedBool::encode_from_raw(true);
    game_sys._0x80_is_arcade_medium_1cc_nanako = EncodedBool::encode_from_raw(true);
    game_sys._0x81_is_arcade_medium_1cc_kae = EncodedBool::encode_from_raw(true);
    game_sys._0x82_is_arcade_medium_1cc_kyoko = EncodedBool::encode_from_raw(true);
    game_sys._0x83_is_arcade_medium_1cc_hime = EncodedBool::encode_from_raw(true);
    game_sys._0x84_is_arcade_medium_1cc_sumika = EncodedBool::encode_from_raw(true);

    game_sys._0x89_is_arcade_hard_1cc_sora = EncodedBool::encode_from_raw(true);
    game_sys._0x8a_is_arcade_hard_1cc_alte = EncodedBool::encode_from_raw(true);
    game_sys._0x8b_is_arcade_hard_1cc_tsih = EncodedBool::encode_from_raw(true);
    game_sys._0x8c_is_arcade_hard_1cc_mira = EncodedBool::encode_from_raw(true);
    game_sys._0x8d_is_arcade_hard_1cc_sham = EncodedBool::encode_from_raw(true);
    game_sys._0x8e_is_arcade_hard_1cc_nath = EncodedBool::encode_from_raw(true);
    game_sys._0x8f_is_arcade_hard_1cc_star_breaker = EncodedBool::encode_from_raw(true);
    game_sys._0x90_is_arcade_hard_1cc_suguri = EncodedBool::encode_from_raw(true);
    game_sys._0x91_is_arcade_hard_1cc_saki = EncodedBool::encode_from_raw(true);
    game_sys._0x92_is_arcade_hard_1cc_iru = EncodedBool::encode_from_raw(true);
    game_sys._0x93_is_arcade_hard_1cc_nanako = EncodedBool::encode_from_raw(true);
    game_sys._0x94_is_arcade_hard_1cc_kae = EncodedBool::encode_from_raw(true);
    game_sys._0x95_is_arcade_hard_1cc_kyoko = EncodedBool::encode_from_raw(true);
    game_sys._0x96_is_arcade_hard_1cc_hime = EncodedBool::encode_from_raw(true);
    game_sys._0x97_is_arcade_hard_1cc_sumika = EncodedBool::encode_from_raw(true);

    game_sys._0x24_unlock_bg_before_the_war = EncodedBool::encode_from_raw(false);
    game_sys._0x25_unlock_bg_war_10k_years_ago = EncodedBool::encode_from_raw(false);
    game_sys._0x26_unlock_bg_canyon_of_wind = EncodedBool::encode_from_raw(false);
    game_sys._0x27_unlock_bg_dust_storm = EncodedBool::encode_from_raw(false);
    game_sys._0x28_unlock_bg_rain_and_sunset = EncodedBool::encode_from_raw(false);
    game_sys._0x29_unlock_bg_equator_doldrums = EncodedBool::encode_from_raw(false);
    game_sys._0x2a_unlock_bg_big_bridge = EncodedBool::encode_from_raw(false);
    game_sys._0x2b_unlock_bg_capital_in_flames = EncodedBool::encode_from_raw(false);
    game_sys._0x2c_unlock_bg_whirlpool_of_malice = EncodedBool::encode_from_raw(false);
    game_sys._0x2e_unlock_bg_nature_10k = EncodedBool::encode_from_raw(false);
    game_sys._0x2f_unlock_bg_crashed_spaceship = EncodedBool::encode_from_raw(false);
    game_sys._0x30_unlock_bg_guardians_chamber = EncodedBool::encode_from_raw(false);
    game_sys._0x31_unlock_bg_moonlight_dance_hall = EncodedBool::encode_from_raw(true);
    game_sys._0x32_unlock_bg_sumika_hideout = EncodedBool::encode_from_raw(true);

    game_sys._0x3e_unlock_bgm_need_for_speed = EncodedBool::encode_from_raw(false);
    game_sys._0x3f_unlock_bgm_black_hole = EncodedBool::encode_from_raw(false);
    game_sys._0x40_unlock_bgm_distant_thunder = EncodedBool::encode_from_raw(false);
    game_sys._0x41_unlock_bgm_swordfish = EncodedBool::encode_from_raw(true);
    game_sys._0x46_unlock_bgm_accelerator = EncodedBool::encode_from_raw(false);
    game_sys._0x47_unlock_bgm_remember_me = EncodedBool::encode_from_raw(false);
    game_sys._0x48_unlock_bgm_mgom = EncodedBool::encode_from_raw(false);

    println!("0x08 {}", game_sys._0x08.decode());

    println!("Setting ugly values");

    game_sys._0x08 = EncodedU32::encode_from_raw(0);

    game_sys._0x1e = EncodedBool::encode_from_raw(false);
    game_sys._0x1f = EncodedBool::encode_from_raw(false);
    game_sys._0x20 = EncodedBool::encode_from_raw(false);
    game_sys._0x21 = EncodedBool::encode_from_raw(false);
    game_sys._0x22 = EncodedBool::encode_from_raw(false);
    game_sys._0x23 = EncodedBool::encode_from_raw(false);

    game_sys._0x2d = EncodedBool::encode_from_raw(false);

    game_sys._0x33 = EncodedU8::encode_from_raw(0);
    game_sys._0x34 = EncodedBool::encode_from_raw(false);
    game_sys._0x35 = EncodedBool::encode_from_raw(false);
    game_sys._0x36 = EncodedBool::encode_from_raw(false);
    game_sys._0x37 = EncodedBool::encode_from_raw(false);
    game_sys._0x38 = EncodedBool::encode_from_raw(false);
    game_sys._0x39 = EncodedBool::encode_from_raw(false);
    game_sys._0x3a = EncodedBool::encode_from_raw(false);
    game_sys._0x3b = EncodedBool::encode_from_raw(false);
    game_sys._0x3c = EncodedBool::encode_from_raw(false);
    game_sys._0x3d = EncodedBool::encode_from_raw(false);

    game_sys._0x42 = EncodedU8::encode_from_raw(0);
    game_sys._0x43 = EncodedU8::encode_from_raw(0);
    game_sys._0x44 = EncodedU8::encode_from_raw(0);
    game_sys._0x45 = EncodedU8::encode_from_raw(0);

    game_sys._0x60 = EncodedU8::encode_from_raw(0);
    game_sys._0x61 = EncodedU8::encode_from_raw(0);
    game_sys._0x62 = EncodedU8::encode_from_raw(0);

    game_sys._0x72 = EncodedU8::encode_from_raw(0);
    game_sys._0x73 = EncodedU8::encode_from_raw(0);
    game_sys._0x74 = EncodedU8::encode_from_raw(0);
    game_sys._0x75 = EncodedU8::encode_from_raw(0);

    game_sys._0x85 = EncodedU8::encode_from_raw(0);
    game_sys._0x86 = EncodedU8::encode_from_raw(0);
    game_sys._0x87 = EncodedU8::encode_from_raw(0);
    game_sys._0x88 = EncodedBool::encode_from_raw(false);

    game_sys._0x98 = EncodedU32::encode_from_raw(0);

    game_sys._0xaa = EncodedU8::encode_from_raw(0);
    game_sys._0xab = EncodedU8::encode_from_raw(0);

    println!("Set ugly values");

    game_sys.save(&settings.game_sys)?;

    println!("Saved file");

    Ok(())
}
