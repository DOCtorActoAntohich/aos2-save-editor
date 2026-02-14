use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
#[error("{action} failed:\n- {detail}\n\nFile at: {path}")]
pub struct Error {
    path: PathBuf,
    action: ErroneousAction,
    detail: ErrorDetail,
}

#[derive(Debug, derive_more::Display)]
pub enum ErrorDetail {
    #[display("Filesystem error: {_0}")]
    FileSystem(std::io::Error),
    #[display("Binary format error: {_0}")]
    BinaryFormat(binrw::Error),
    NotFound,
    NoPermission,
}

#[derive(Debug, derive_more::Display)]
pub enum ErroneousAction {
    Reading,
    Writing,
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
        ErrorDetail::BinaryFormat(error)
    }
}
