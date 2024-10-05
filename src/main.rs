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

    game_sys._0x1c_unlock_hime = EncodedBool::encode_from_raw(true);
    game_sys._0x15_unlock_star_breaker = EncodedBool::encode_from_raw(true);

    game_sys._0x4c_singleplayer_wins = EncodedU32::encode_from_raw(0);
    game_sys._0x50_arcade_easy_1ccs_unsure = EncodedU32::encode_from_raw(0);
    game_sys._0x58_arcade_hard_1ccs = EncodedU32::encode_from_raw(0);
    game_sys._0x5c_story_1cc_completions = EncodedU32::encode_from_raw(0);

    game_sys._0x9c_is_story_1cc_sora = EncodedBool::encode_from_raw(false);
    game_sys._0x9d_is_story_1cc_alte = EncodedBool::encode_from_raw(false);
    game_sys._0x9e_is_story_1cc_tsih = EncodedBool::encode_from_raw(false);
    game_sys._0x9f_is_story_1cc_mira = EncodedBool::encode_from_raw(false);
    game_sys._0xa0_is_story_1cc_sham = EncodedBool::encode_from_raw(false);
    game_sys._0xa1_is_story_1cc_nath = EncodedBool::encode_from_raw(false);
    game_sys._0xa2_is_story_1cc_star_breaker = EncodedBool::encode_from_raw(true);
    game_sys._0xa3_is_story_1cc_suguri = EncodedBool::encode_from_raw(false);
    game_sys._0xa4_is_story_1cc_saki = EncodedBool::encode_from_raw(false);
    game_sys._0xa5_is_story_1cc_iru = EncodedBool::encode_from_raw(false);
    game_sys._0xa6_is_story_1cc_nanako = EncodedBool::encode_from_raw(false);
    game_sys._0xa7_is_story_1cc_kae = EncodedBool::encode_from_raw(false);
    game_sys._0xa8_is_story_1cc_kyoko = EncodedBool::encode_from_raw(false);
    game_sys._0xa9_is_story_1cc_hime = EncodedBool::encode_from_raw(true);

    game_sys._0x63_is_arcade_easy_1cc_sora = EncodedBool::encode_from_raw(false);
    game_sys._0x64_is_arcade_easy_1cc_alte = EncodedBool::encode_from_raw(false);
    game_sys._0x65_is_arcade_easy_1cc_tsih = EncodedBool::encode_from_raw(false);
    game_sys._0x66_is_arcade_easy_1cc_mira = EncodedBool::encode_from_raw(false);
    game_sys._0x67_is_arcade_easy_1cc_sham = EncodedBool::encode_from_raw(false);
    game_sys._0x68_is_arcade_easy_1cc_nath = EncodedBool::encode_from_raw(false);
    game_sys._0x69_is_arcade_easy_1cc_star_breaker = EncodedBool::encode_from_raw(false);
    game_sys._0x6a_is_arcade_easy_1cc_suguri = EncodedBool::encode_from_raw(false);
    game_sys._0x6b_is_arcade_easy_1cc_saki = EncodedBool::encode_from_raw(false);
    game_sys._0x6c_is_arcade_easy_1cc_iru = EncodedBool::encode_from_raw(false);
    game_sys._0x6d_is_arcade_easy_1cc_nanako = EncodedBool::encode_from_raw(false);
    game_sys._0x6e_is_arcade_easy_1cc_kae = EncodedBool::encode_from_raw(false);
    game_sys._0x6f_is_arcade_easy_1cc_kyoko = EncodedBool::encode_from_raw(false);
    game_sys._0x70_is_arcade_easy_1cc_hime = EncodedBool::encode_from_raw(false);

    game_sys._0x89_is_arcade_hard_1cc_sora = EncodedBool::encode_from_raw(false);
    game_sys._0x8a_is_arcade_hard_1cc_alte = EncodedBool::encode_from_raw(false);
    game_sys._0x8b_is_arcade_hard_1cc_tsih = EncodedBool::encode_from_raw(false);
    game_sys._0x8c_is_arcade_hard_1cc_mira = EncodedBool::encode_from_raw(false);
    game_sys._0x8d_is_arcade_hard_1cc_sham = EncodedBool::encode_from_raw(false);
    game_sys._0x8e_is_arcade_hard_1cc_nath = EncodedBool::encode_from_raw(false);
    game_sys._0x8f_is_arcade_hard_1cc_star_breaker = EncodedBool::encode_from_raw(false);
    game_sys._0x90_is_arcade_hard_1cc_suguri = EncodedBool::encode_from_raw(false);
    game_sys._0x91_is_arcade_hard_1cc_saki = EncodedBool::encode_from_raw(false);
    game_sys._0x92_is_arcade_hard_1cc_iru = EncodedBool::encode_from_raw(false);
    game_sys._0x93_is_arcade_hard_1cc_nanako = EncodedBool::encode_from_raw(false);
    game_sys._0x94_is_arcade_hard_1cc_kae = EncodedBool::encode_from_raw(false);
    game_sys._0x95_is_arcade_hard_1cc_kyoko = EncodedBool::encode_from_raw(false);
    game_sys._0x96_is_arcade_hard_1cc_hime = EncodedBool::encode_from_raw(false);

    game_sys.save(&settings.game_sys)?;

    println!("Saved file");

    Ok(())
}
