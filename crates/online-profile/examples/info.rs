use aos2_env::AoS2Env;
use online_profile::PlayerOnlineProfile;

fn main() -> anyhow::Result<()> {
    let env = AoS2Env::from_env()?;
    let profile = PlayerOnlineProfile::load(&env)?;

    println!("Player: {}", profile.nickname);

    println!("- Avatar: {}", profile.avatar_character);
    println!("- Background: {}\t", profile.avatar_background);

    println!(
        "- {} Title: {} [{}]",
        profile.title_color, profile.title_text_id, profile.title_character_in_background
    );

    Ok(())
}
