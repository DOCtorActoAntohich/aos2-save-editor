use std::fmt::Display;

use crate::diff::{ByteDifferenceTable, SizeDifferenceTable};

use super::{BinaryFile, ByteDifference, SizeDifference};

#[derive(Debug, Clone)]
pub enum FileDifference {
    Identical,
    SameSizeOnlyBytes(Vec<ByteDifference>),
    SizeInnerDiff {
        size_diff: SizeDifference,
        first_byte_diff: ByteDifference,
    },
    SizeOuterDiff(SizeDifference),
}

enum FilesPair<'a> {
    SameSize(EqualSizeFiles<'a>),
    DifferentSize(UnequalSizeFiles<'a>),
}

#[derive(Debug)]
struct EqualSizeFiles<'a> {
    previous: &'a BinaryFile,
    current: &'a BinaryFile,
}

struct UnequalSizeFiles<'a> {
    previous: &'a BinaryFile,
    current: &'a BinaryFile,
}

impl FileDifference {
    pub fn between(previous: &BinaryFile, current: &BinaryFile) -> Self {
        let files = if previous.content().len() == current.content().len() {
            FilesPair::SameSize(EqualSizeFiles { previous, current })
        } else {
            FilesPair::DifferentSize(UnequalSizeFiles { previous, current })
        };
        files.difference()
    }
}

impl Display for FileDifference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileDifference::Identical => writeln!(f, "Identical files"),
            FileDifference::SameSizeOnlyBytes(diffs) => {
                writeln!(f, "Same file size but different contents:")?;
                write!(f, "{}", ByteDifferenceTable::from(diffs.as_ref()))
            }
            FileDifference::SizeInnerDiff {
                size_diff,
                first_byte_diff,
            } => {
                writeln!(f, "{}", SizeDifferenceTable::from(size_diff))?;
                writeln!(f, "First change in overlapping bytes:")?;
                write!(
                    f,
                    "{}",
                    ByteDifferenceTable::from(vec![*first_byte_diff].as_ref())
                )
            }
            FileDifference::SizeOuterDiff(size_diff) => {
                write!(f, "{}", SizeDifferenceTable::from(size_diff))
            }
        }
    }
}

impl FilesPair<'_> {
    pub fn difference(self) -> FileDifference {
        match self {
            FilesPair::SameSize(same_size_files) => same_size_files.difference(),
            FilesPair::DifferentSize(different_size_files) => different_size_files.difference(),
        }
    }
}

impl EqualSizeFiles<'_> {
    pub fn difference(self) -> FileDifference {
        let diffs: Vec<ByteDifference> = self
            .previous
            .content()
            .iter()
            .zip(self.current.content())
            .enumerate()
            .filter_map(|(index, (&previous_byte, &current_byte))| {
                (previous_byte != current_byte).then_some(ByteDifference {
                    position: index,
                    previous: previous_byte,
                    current: current_byte,
                })
            })
            .collect();

        if diffs.is_empty() {
            FileDifference::Identical
        } else {
            FileDifference::SameSizeOnlyBytes(diffs)
        }
    }
}

impl UnequalSizeFiles<'_> {
    pub fn difference(self) -> FileDifference {
        let inner_diff_opt = self
            .previous
            .content()
            .iter()
            .zip(self.current.content())
            .enumerate()
            .find_map(|(index, (&previous_byte, &current_byte))| {
                (previous_byte != current_byte).then_some(ByteDifference {
                    position: index,
                    previous: previous_byte,
                    current: current_byte,
                })
            });

        let size_diff = SizeDifference {
            previous: self.previous.content().len(),
            current: self.current.content().len(),
        };
        match inner_diff_opt {
            Some(inner_diff) => FileDifference::SizeInnerDiff {
                size_diff,
                first_byte_diff: inner_diff,
            },
            None => FileDifference::SizeOuterDiff(size_diff),
        }
    }
}
