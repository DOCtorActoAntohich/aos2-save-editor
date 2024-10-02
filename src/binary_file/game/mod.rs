use super::{savefile::GameSaveFile, sized_section::SizedBinarySection};

#[binrw::binrw]
#[derive(Debug)]
#[brw(little)]
pub struct GameSysFile {
    _looks_like_version: Version,
    pub content: SizedBinarySection<0xa4, 0xa4>,
}

#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq, Hash, derive_more::Display)]
#[brw(magic = b"\x4C\x53\x44\x00")]
struct Version;

impl GameSaveFile for GameSysFile {}
