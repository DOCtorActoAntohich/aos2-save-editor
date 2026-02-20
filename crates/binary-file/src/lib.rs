#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

use std::path::PathBuf;

/// Means the purpose of the field is unknown.
///
/// "Explicit is better than implicit".
#[binrw::binrw]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, derive_more::From)]
#[brw(little)]
pub struct UnknownU8(u8);

impl UnknownU8 {
    #[must_use]
    pub const fn new(value: u8) -> Self {
        Self(value)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("{action} failed:\n- {detail}\n\nFile at: {path}")]
pub struct Error {
    pub path: PathBuf,
    pub action: ErroneousAction,
    pub detail: ErrorDetail,
}

#[derive(Debug, derive_more::Display)]
pub enum ErrorDetail {
    #[display("Not found")]
    NotFound,
    #[display("No permission")]
    NoPermission,
    UnsupportedVersion(UnsupportedVersion),
    #[display("Filesystem error: {_0}")]
    FileSystem(std::io::Error),
    #[display("Binary format error: {_0}")]
    BinaryFormat(binrw::Error),
    #[display("Weird unchecked error (blame the developers!!): {_0}")]
    Weird(Box<dyn binrw::error::CustomError>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, derive_more::Display, derive_more::FromStr)]
pub enum ErroneousAction {
    Reading,
    Writing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("Wrong file version (expected: `{expected:04x}`, actual: `{actual:04x}`")]
pub struct UnsupportedVersion {
    pub expected: u16,
    pub actual: u16,
}

impl Error {
    pub fn reading_file(path: impl Into<PathBuf>, err: std::io::Error) -> Self {
        Self {
            path: path.into(),
            action: ErroneousAction::Reading,
            detail: err.into(),
        }
    }

    pub fn writing_file(path: impl Into<PathBuf>, err: std::io::Error) -> Self {
        Self {
            path: path.into(),
            action: ErroneousAction::Writing,
            detail: err.into(),
        }
    }

    pub fn reading_binary(path: impl Into<PathBuf>, err: binrw::Error) -> Self {
        Self {
            path: path.into(),
            action: ErroneousAction::Reading,
            detail: err.into(),
        }
    }

    pub fn writing_binary(path: impl Into<PathBuf>, err: binrw::Error) -> Self {
        Self {
            path: path.into(),
            action: ErroneousAction::Writing,
            detail: err.into(),
        }
    }
}

impl From<std::io::Error> for ErrorDetail {
    fn from(error: std::io::Error) -> Self {
        match error.kind() {
            std::io::ErrorKind::NotFound => ErrorDetail::NotFound,
            std::io::ErrorKind::PermissionDenied => ErrorDetail::NoPermission,
            _ => ErrorDetail::FileSystem(error),
        }
    }
}

impl From<binrw::Error> for ErrorDetail {
    fn from(error: binrw::Error) -> Self {
        let err: Box<dyn binrw::error::CustomError> = match error {
            binrw::Error::Custom { pos: _, err } => err,
            // Rage consumes me :rage:. Who made it this way...
            binrw::Error::Backtrace(backtrace) => {
                let error = *backtrace.error;
                return error.into();
            }
            other => return ErrorDetail::BinaryFormat(other),
        };

        let err = match err.downcast::<UnsupportedVersion>() {
            Ok(unsupported) => return ErrorDetail::UnsupportedVersion(*unsupported),
            Err(err) => err,
        };

        ErrorDetail::Weird(err)
    }
}
