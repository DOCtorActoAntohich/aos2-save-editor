use anyhow::Context;
use binrw::{
    meta::{ReadEndian, WriteEndian},
    BinRead, BinWrite,
};

pub trait GameSaveFile: BinRead + ReadEndian + BinWrite + WriteEndian
where
    Self: Sized,
{
    fn from_file<P>(path: P) -> anyhow::Result<Self>
    where
        P: AsRef<std::path::Path>,
        for<'a> <Self as BinRead>::Args<'a>: Default,
    {
        let mut reader = std::fs::File::open(path).context("Failed to open file")?;

        BinRead::read(&mut reader).context("Failed to parse file")
    }

    fn save<P>(&self, path: P) -> anyhow::Result<()>
    where
        P: AsRef<std::path::Path>,
        for<'a> <Self as BinWrite>::Args<'a>: Default,
    {
        let mut writer = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(path)
            .context("Failed to create or open the file for writing")?;

        BinWrite::write(self, &mut writer).context("Failed to overwrite file contents")
    }
}
