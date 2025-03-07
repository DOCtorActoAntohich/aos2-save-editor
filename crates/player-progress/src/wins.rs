#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[brw(little)]
pub struct SingleplayerWins {
    /// A total number of singleplayer wins for any character on any difficulty.
    ///
    /// Simply winning one fight is enough to increase the counter.
    pub total: u32,
    /// How many times a player finished
    /// Arcade mode for any character on Easy difficulty without dying.
    pub n_arcade_easy_1ccs: u32,
    /// How many times a player finished
    /// Arcade mode for any character on Medium difficulty without dying.
    pub n_arcade_medium_1ccs: u32,
    /// How many times a player finished
    /// Arcade mode for any character on Hard difficulty without dying.
    pub n_arcade_hard_1ccs: u32,
    /// How many times a player finished
    /// Story mode for any character on any difficulty without dying.
    pub n_story_1ccs: u32,
}
