use std::{fs::File, io::Read, path::Path};

use anyhow::Context;

#[derive(Debug, Clone)]
pub struct BinaryFile(Vec<u8>);

impl BinaryFile {
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

    pub fn content(&self) -> &[u8] {
        &self.0
    }
}
