mod diff;
mod hexy;

use std::path::PathBuf;

use aos2_save_editor::settings::Settings;
use clap::Parser;
use diff::{EpicFile, FileDifference};
use hexy::HexyNumber;

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
    let before = EpicFile::load(settings.saves_folder.join(args.before))?;
    let after = EpicFile::load(settings.saves_folder.join(args.after))?;

    match after.diff_from(&before) {
        Some(FileDifference::Size { previous, current }) => println!(
            "Different file size: {} (before) vs {} (after)",
            HexyNumber::from(previous),
            HexyNumber::from(current)
        ),
        Some(FileDifference::Bytes(diffs)) => {
            println!("Same file size but different contents");
            println!("Position\tBefore\t\tAfter");
            diffs.into_iter().for_each(|diff| {
                println!(
                    "{}\t{}\t{}",
                    HexyNumber::from(diff.position),
                    HexyNumber::from(diff.previous),
                    HexyNumber::from(diff.current)
                );
            })
        }
        None => println!("Identical files"),
    }

    Ok(())
}
