use binrw::{BinRead, BinWrite};

use crate::bin_bool::BinBool;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BackgroundMusicSheet {
    pub need_for_speed: bool,
    pub black_hole: bool,
    pub distant_thunder: bool,
    pub swordfish: bool,
    pub shine: bool,
    pub expendables: bool,
    pub ribbon: bool,
    pub moving_out: bool,
    pub accelerator: bool,
    pub remember_me: bool,
    pub mgom: bool,
}

#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq)]
#[brw(little)]
pub struct BinarySheet {
    need_for_speed: BinBool,
    black_hole: BinBool,
    distant_thunder: BinBool,
    swordfish: BinBool,
    shine: BinBool,
    expendables: BinBool,
    ribbon: BinBool,
    moving_out: BinBool,
    accelerator: BinBool,
    remember_me: BinBool,
    mgom: BinBool,
}

impl BinRead for BackgroundMusicSheet {
    type Args<'a> = <BinarySheet as BinRead>::Args<'a>;

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        <BinarySheet as BinRead>::read_options(reader, endian, args).map(Into::into)
    }
}

impl BinWrite for BackgroundMusicSheet {
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

impl From<BinarySheet> for BackgroundMusicSheet {
    fn from(
        BinarySheet {
            need_for_speed,
            black_hole,
            distant_thunder,
            swordfish,
            shine,
            expendables,
            ribbon,
            moving_out,
            accelerator,
            remember_me,
            mgom,
        }: BinarySheet,
    ) -> Self {
        Self {
            need_for_speed: need_for_speed.into(),
            black_hole: black_hole.into(),
            distant_thunder: distant_thunder.into(),
            swordfish: swordfish.into(),
            shine: shine.into(),
            expendables: expendables.into(),
            ribbon: ribbon.into(),
            moving_out: moving_out.into(),
            accelerator: accelerator.into(),
            remember_me: remember_me.into(),
            mgom: mgom.into(),
        }
    }
}

impl From<BackgroundMusicSheet> for BinarySheet {
    fn from(
        BackgroundMusicSheet {
            need_for_speed,
            black_hole,
            distant_thunder,
            swordfish,
            shine,
            expendables,
            ribbon,
            moving_out,
            accelerator,
            remember_me,
            mgom,
        }: BackgroundMusicSheet,
    ) -> Self {
        Self {
            need_for_speed: need_for_speed.into(),
            black_hole: black_hole.into(),
            distant_thunder: distant_thunder.into(),
            swordfish: swordfish.into(),
            shine: shine.into(),
            expendables: expendables.into(),
            ribbon: ribbon.into(),
            moving_out: moving_out.into(),
            accelerator: accelerator.into(),
            remember_me: remember_me.into(),
            mgom: mgom.into(),
        }
    }
}
