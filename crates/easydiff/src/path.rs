use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use anyhow::Context;

#[derive(Debug, Clone, derive_more::Into, derive_more::Deref, derive_more::AsRef)]
#[as_ref(forward)]
pub struct CanonicalSaveFilePath(PathBuf);

impl CanonicalSaveFilePath {
    pub fn new(saves: impl AsRef<Path>, file_name: impl AsRef<Path>) -> anyhow::Result<Self> {
        match file_name.as_ref().canonicalize() {
            Ok(real) => return Ok(Self(real)),
            Err(err) if err.kind() == ErrorKind::NotFound => (),
            Err(err) => {
                return Err(err).with_context(|| {
                    anyhow::anyhow!(
                        "Failed to canonicalize path: {}",
                        file_name.as_ref().display()
                    )
                });
            }
        };

        let joined = saves.as_ref().join(file_name);
        joined
            .canonicalize()
            .with_context(|| {
                anyhow::anyhow!(
                    "Failed to canonicalize path in AoS2 directory: {}",
                    joined.display()
                )
            })
            .map(Self)
    }
}
