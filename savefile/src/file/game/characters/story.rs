use binrw::{BinRead, BinWrite};

use crate::bin_bool::BinBool;

/// Same as [`FullCharacterSheet`] but doesn't have Sumika.
///
/// Because that silly girl doesn't have her story mode playthrough.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StoryCharacterSheet {
    pub sora: bool,
    pub alte: bool,
    pub tsih: bool,
    pub mira: bool,
    pub sham: bool,
    pub nath: bool,
    pub star_breaker: bool,
    pub suguri: bool,
    pub saki: bool,
    pub iru: bool,
    pub nanako: bool,
    pub kae: bool,
    pub kyoko: bool,
    pub hime: bool,
}

#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq)]
#[brw(little)]
pub struct BinarySheet {
    sora: BinBool,
    alte: BinBool,
    tsih: BinBool,
    mira: BinBool,
    sham: BinBool,
    nath: BinBool,
    star_breaker: BinBool,
    suguri: BinBool,
    saki: BinBool,
    iru: BinBool,
    nanako: BinBool,
    kae: BinBool,
    kyoko: BinBool,
    hime: BinBool,
}

impl StoryCharacterSheet {
    pub const FULLY_UNLOCKED: Self = Self {
        sora: true,
        alte: true,
        tsih: true,
        mira: true,
        sham: true,
        nath: true,
        star_breaker: true,
        suguri: true,
        saki: true,
        iru: true,
        nanako: true,
        kae: true,
        kyoko: true,
        hime: true,
    };
}

impl BinRead for StoryCharacterSheet {
    type Args<'a> = <BinarySheet as BinRead>::Args<'a>;

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        <BinarySheet as BinRead>::read_options(reader, endian, args).map(Into::into)
    }
}

impl BinWrite for StoryCharacterSheet {
    type Args<'a> = <BinarySheet as BinWrite>::Args<'a>;

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        let binary: BinarySheet = self.clone().into();
        <BinarySheet as BinWrite>::write_options(&binary, writer, endian, args)
    }
}

impl From<BinarySheet> for StoryCharacterSheet {
    fn from(
        BinarySheet {
            sora,
            alte,
            tsih,
            mira,
            sham,
            nath,
            star_breaker,
            suguri,
            saki,
            iru,
            nanako,
            kae,
            kyoko,
            hime,
        }: BinarySheet,
    ) -> Self {
        Self {
            sora: sora.into(),
            alte: alte.into(),
            tsih: tsih.into(),
            mira: mira.into(),
            sham: sham.into(),
            nath: nath.into(),
            star_breaker: star_breaker.into(),
            suguri: suguri.into(),
            saki: saki.into(),
            iru: iru.into(),
            nanako: nanako.into(),
            kae: kae.into(),
            kyoko: kyoko.into(),
            hime: hime.into(),
        }
    }
}

impl From<StoryCharacterSheet> for BinarySheet {
    fn from(
        StoryCharacterSheet {
            sora,
            alte,
            tsih,
            mira,
            sham,
            nath,
            star_breaker,
            suguri,
            saki,
            iru,
            nanako,
            kae,
            kyoko,
            hime,
        }: StoryCharacterSheet,
    ) -> Self {
        Self {
            sora: sora.into(),
            alte: alte.into(),
            tsih: tsih.into(),
            mira: mira.into(),
            sham: sham.into(),
            nath: nath.into(),
            star_breaker: star_breaker.into(),
            suguri: suguri.into(),
            saki: saki.into(),
            iru: iru.into(),
            nanako: nanako.into(),
            kae: kae.into(),
            kyoko: kyoko.into(),
            hime: hime.into(),
        }
    }
}
