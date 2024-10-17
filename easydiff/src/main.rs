mod diff;
mod fmt;
mod path;

use std::path::PathBuf;

use aos2_env::AoS2Paths;
use clap::Parser;

use crate::{
    diff::{BinaryFile, FileDifference},
    path::CanonicalSaveFilePath,
};

/// Looks for files in AoS2 saves folder and shows a simple difference.
#[derive(Debug, Parser)]
struct Args {
    /// The original (unchanged) file.
    #[arg(value_name = "ORIGINAL")]
    before: PathBuf,
    /// The other (modified) file.
    #[arg(value_name = "MODIFIED")]
    after: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let settings = AoS2Paths::from_env()?;
    let args = Args::parse();

    run(settings, args)
}

fn run(settings: AoS2Paths, args: Args) -> anyhow::Result<()> {
    let before_path = CanonicalSaveFilePath::new(&settings.saves_folder, &args.before)?;
    let after_path = CanonicalSaveFilePath::new(&settings.saves_folder, &args.after)?;

    let before = BinaryFile::load(&before_path)?;
    let after = BinaryFile::load(&after_path)?;

    println!(
        "Working with files:\nBefore: {}\nAfter: {}\n",
        before_path.display(),
        after_path.display(),
    );

    println!("{}", FileDifference::between(&before, &after));

    Ok(())
}
