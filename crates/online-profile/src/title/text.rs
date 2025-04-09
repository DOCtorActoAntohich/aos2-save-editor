/// Title top text to display for everyone to see.
///
/// There are many. Way too many.
/// Some are "paid" via the in-game currency,
/// but why pay when you can do it for free.
///
/// "What a pain in the neck" (c) Kyoko, and me when writing all those down.
#[binrw::binrw]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    derive_more::TryFrom,
    derive_more::Display,
    enum_array::EnumMembersArray,
)]
#[brw(little, repr(u32))]
#[repr(u32)]
#[try_from(repr)]
pub enum Text {
    #[default]
    #[display("\"None\"")]
    None = 0x00,
    HelloWorld = 1,
    Aos2Player,
    OjPlayer,
    RushdownPlayer,
    ZoningPlayer,
    OffensivePlayer,
    DefensivePlayer,
    CasualPlayer,
    CompetitivePlayer,
    HeatingUp,
    GoingForWin,
    NiceToMeetYou,
    FairFight,
    GrindTime,
    LittleWar,
    Glhf,
    FightingGameFan,
    OrangeJuiceFan,
    OnTheUpAndAp,
    TeachLesson,
    GoodMorning,
    GoodAfternoon,
    GoodEvening,
    Newbie,
    Veteran,
    PlayOnWeekends,
    PlayOnWeekdays,
    PlayAtNight,
    PlayAtDay,
    DangerZone,
    BlameTheLag,
    BringItOn,
    BodyMindAndSoul,
    PhdInMeterManagement,
    Warmup,
    ButtonMasher,
    Accelerating,
    NeverLose,
    DieAHero,
    PartTimer,
    FullTimer,
    /// No fucking way.
    LookingForFriends = 42,
    LookingForRivals,
    LookingForGoodChallenge,
    TrainingForTournament,
    WannaGetGood,
    NewbiesOnly,
    VeteransOnly,
    NoLuckButStill,
    LuckIsSkill,
    ComebackMaster,
    BreakingASweat,
    Dash,
    Attack,
    Cancel,
    Hyper,
    Guard,
    Play100OjToo,
    CasualMatch,
    SeriousMatch,
    NorthAmerica,
    Europe,
    Asia,
    Japan,
    Oceania,
    Africa,
    MiddleEast,
    LatinAmerica,
    SoraUltimateWeaponGirl = 69, // Damn she hot??
    SoraUltimateBeatdown,
    SoraSkyIsTheLimit,
    SoraCantLetYouDoThatStarBreaker,
    SoraCommencingMission,
    SoraMissionAccomplished,
    SoraNewbie,
    SoraMaster,
    SoraFan,
    SoraTraining,
    SoraSpecialist,
    SoraPlayer,
    SoraWaifu,
    AlteSearchParty = 82,
    AlteLightningRod,
    AlteSupremeLoyalty,
    AltePrettyInPink,
    AlteLambda,
    AlteFreeHugs,
    AlteNewbie,
    AlteMaster,
    AlteFan,
    AlteTraining,
    AlteSpecialist,
    AltePlayer,
    AlteWaifu,
    TsihTactitalEspyonyageNanoraction = 95,
    TsihChameleon,
    TsihRockAndRoll,
    TsihGamma,
    TsihPigyamoooh,
    TsihNora,
    TsihNanora,
    TsihNewbie,
    TsihMaster,
    TsihFan,
    TsihTraining,
    TsihSpecialist,
    TsihPlayer,
    TsihWaifu,
    MiraLetItRip = 109,
    MiraNinjaMaster,
    MiraSupremeFour,
    MiraMasterOfSpinningBlades,
    MiraOmicron,
    MiraTwinDragonTornado,
    MiraTwoInOne,
    MiraWonderful,
    MiraNewbie,
    MiraMaster,
    MiraFan,
    MiraTraining,
    MiraSpecialist,
    MiraPlayer,
    MiraWaifu,
    ShamMasterIdol = 124,
    ShamAlpha,
    ShamWarlandSage,
    ShamHiveQueen,
    ShamInstructor,
    ShamRobotSwarm,
    ShamNewbie,
    ShamMaster,
    ShamFan,
    ShamTraining,
    ShamSpecialist,
    ShamPlayer,
    ShamWaifu,
    NathChopSuey = 137,
    NathBeta,
    NathTrifecta,
    NathMech3,
    NathExtension,
    NathGetInTheRobot,
    NathNatto,
    NathAnotherUltimateWeapon,
    NathNewbie,
    NathMaster,
    NathFan,
    NathTraining,
    NathSpecialist,
    NathPlayer,
    NathWaifu,
    StarBreakerBlastingFuse = 152,
    StarBreakerPyromaniac,
    StarBreakerLikesWellDone,
    StarBreakerKaboom,
    StarBreakerSuperNove,
    StarBreakerStardust,
    StarBreakerNewbie,
    StarBreakerMaster,
    StarBreakerFan,
    StarBreakerTraining,
    StarBreakerSpecialist,
    StarBreakerPlayer,
    StarBreakerWaifu,
    SuguriYearsOfExperience = 165,
    SuguriProjectOne,
    SuguriYearsTooEarlyToDefeat,
    SuguriIcarus,
    SuguriProtagonist,
    SuguriLittleWar,
    SuguriGaia,
    SuguriNewbie,
    SuguriMaster,
    SuguriFan,
    SuguriTraining,
    SuguriSpecialist,
    SuguriPlayer,
    SuguriWaifu,
    SakiSweetMaker = 179,
    SakiPercussionist,
    SakiBigBangBell,
    SakiSamba,
    SakiMauryah,
    SakiPleaseDie,
    SakiNewbie,
    SakiMaster,
    SakiFan,
    SakiTraining,
    SakiSpecialist,
    SakiPlayer,
    SakiWaifu,
    IruMarksman = 192,
    IruTomboy,
    IruMinesweeper,
    IruLongDistanceRelationship,
    IruFastestGun,
    IruRocketeer,
    IruConfirmedKiller,
    IruNewbie,
    IruMaster,
    IruFan,
    IruTraining,
    IruSpecialist,
    IruPlayer,
    IruWaifu,
    NanakoInFormation = 206,
    NanakoSevenBitEra,
    NanakoShorty,
    NanakoPro75,
    NanakoLuckySeven,
    NanakoBeatsByBit,
    NanakoNewbie,
    NanakoMaster,
    NanakoFan,
    NanakoTraining,
    NanakoSpecialist,
    NanakoPlayer,
    NanakoWaifu,
    KaeHeat300 = 219,
    KaeSummerNight,
    KaeBurningHeart,
    KaeChildishSpirit,
    KaeSpeedOfSound,
    KaeHeatwave,
    KaeNewbie,
    KaeMaster,
    KaeFan,
    KaeTraining,
    KaeSpecialist,
    KaePlayer,
    KaeWaifu,
    KyokoDeepFreeze = 232,
    KyokoAbsoluteZero,
    KyokoBipolar,
    KyokoBrittle,
    KyokoMotherKnowsBest,
    KyokoIceQueen,
    KyokoAvalanche,
    KyokoImmovableObject,
    KyokoStreamsOfSorrow,
    KyokoNewbie,
    KyokoMaster,
    KyokoFan,
    KyokoTraining,
    KyokoSpecialist,
    KyokoPlayer,
    KyokoWaifu,
    HimeGuardian = 248,
    HimeTiesThatBind,
    HimeBoundByDestiny,
    HimePrincess,
    HimeElegantDancer,
    HimeNewbie,
    HimeMaster,
    HimeFan,
    HimeTraining,
    HimeSpecialist,
    HimePlayer,
    HimeWaifu, // i guess so.
    SumikaBarrelCrazy = 260,
    SumikaShipGirl,
    SumikaFeatherDance,
    SumikaWaterAndMelon,
    SumikaToysMeister,
    SumikaCarnival,
    SumikaNewbie,
    SumikaMaster,
    SumikaFan,
    SumikaTraining,
    SumikaSpecialist,
    SumikaPlayer,
    SumikaWaifu,
    #[display("<Invisible text>")]
    Blank = 273,
    #[display("<Disable Title>")]
    Disabled = 0xffff_ffff,
}

impl From<Text> for u32 {
    fn from(value: Text) -> Self {
        value as u32
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use binrw::BinRead;

    use super::Text;

    #[rstest::rstest]
    fn it_parses() {
        let range = 0u32..=273u32;

        for value in range {
            let mut cursor = Cursor::new(value.to_le_bytes());

            let _parsed = Text::read(&mut cursor).expect("Must read here");
        }
    }
}
