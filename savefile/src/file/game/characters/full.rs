use binrw::{BinRead, BinWrite};

use crate::bin_bool::BinBool;

/// Full list of characters.
///
/// ORDER MATTERS. That's how they are coded in game.
///
/// Also see [`StoryCharacterSheet`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FullCharacterSheet {
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
    pub sumika: bool,
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
    sumika: BinBool,
}

impl BinRead for FullCharacterSheet {
    type Args<'a> = <BinarySheet as BinRead>::Args<'a>;

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        <BinarySheet as BinRead>::read_options(reader, endian, args).map(Into::into)
    }
}

impl BinWrite for FullCharacterSheet {
    type Args<'a> = <BinarySheet as BinWrite>::Args<'a>;

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        let binary = self.clone().into();
        <BinarySheet as BinWrite>::write_options(&binary, writer, endian, args)
    }
}

impl From<FullCharacterSheet> for BinarySheet {
    fn from(
        FullCharacterSheet {
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
            sumika,
        }: FullCharacterSheet,
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
            sumika: sumika.into(),
        }
    }
}

impl From<BinarySheet> for FullCharacterSheet {
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
            sumika,
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
            sumika: sumika.into(),
        }
    }
}
