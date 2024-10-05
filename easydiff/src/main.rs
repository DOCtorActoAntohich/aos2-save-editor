mod diff;
mod fmt;

use std::path::PathBuf;

use aos2_env::Settings;
use clap::Parser;
use diff::{BinaryFile, FileDifference};

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
    let settings = Settings::from_env()?;
    let args = Args::parse();

    run(settings, args)
}

fn run(settings: Settings, args: Args) -> anyhow::Result<()> {
    let before = BinaryFile::load(settings.saves_folder.join(args.before))?;
    let after = BinaryFile::load(settings.saves_folder.join(args.after))?;

    println!("{}", FileDifference::between(&before, &after));

    Ok(())
}
