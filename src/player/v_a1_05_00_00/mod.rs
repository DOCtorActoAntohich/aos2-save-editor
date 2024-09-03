mod version;

use version::Version;

use super::{
    bin_bool::BinBool,
    sized_string::{LobbyName, LobbyPassword, Nickname},
};

#[binrw::binrw]
#[derive(Debug)]
pub struct PlayerFile {
    version: Version,
    show_country: BinBool,
    nickname: Nickname,
    lobby_name: LobbyName,
    lobby_password: LobbyPassword,
}
