mod version;

use version::Version;

use super::{
    bin_bool::BinBool,
    sized_string::{LobbyNameSection, LobbyPasswordSection, NicknameSection},
};

#[binrw::binrw]
#[derive(Debug)]
pub struct PlayerFile {
    version: Version,
    show_country: BinBool,
    nickname: NicknameSection,
    lobby_name: LobbyNameSection,
    lobby_password: LobbyPasswordSection,
}
