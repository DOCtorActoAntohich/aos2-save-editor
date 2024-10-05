use std::{fs::File, io::Read, path::Path};

use anyhow::Context;

#[derive(Debug, Clone)]
pub struct EpicFile(Vec<u8>);

#[derive(Debug, Clone)]
pub enum FileDifference {
    Size { previous: usize, current: usize },
    Bytes(Vec<ByteDifference>),
}

#[derive(Debug, Clone)]
pub struct ByteDifference {
    pub position: usize,
    pub previous: u8,
    pub current: u8,
}

impl EpicFile {
    pub fn load<P>(path: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let file = File::open(path.as_ref())
            .with_context(|| format!("Failed to open file: {}", path.as_ref().display()))?;

        let contents = file
            .bytes()
            .collect::<Result<Vec<u8>, _>>()
            .context("Failed to process file")?;

        Ok(Self(contents))
    }

    pub fn diff_from(&self, previous: &Self) -> Option<FileDifference> {
        if self.0.len() != previous.0.len() {
            return Some(FileDifference::Size {
                previous: previous.0.len(),
                current: self.0.len(),
            });
        }

        let diffs: Vec<_> = previous
            .0
            .iter()
            .zip(self.0.iter())
            .enumerate()
            .filter_map(|(index, (&previous_byte, &current_byte))| {
                if previous_byte != current_byte {
                    Some(ByteDifference {
                        position: index,
                        previous: previous_byte,
                        current: current_byte,
                    })
                } else {
                    None
                }
            })
            .collect();

        if diffs.is_empty() {
            None
        } else {
            Some(FileDifference::Bytes(diffs))
        }
    }
}
