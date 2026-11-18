use std::io;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io: {0}")]
    Io(#[from] io::Error),
    #[error("the file is not a valid .reiwand file: {0}")]
    Invalid(String),
    #[error(
        "the file has format {found}, newer than the format {known} this program reads; update the program"
    )]
    Future { found: u32, known: u32 },
    #[error(
        "the file has format {found}, and this program reads format {known}; convert it with `reiwand-upgrade export FILE` on a computer (an encrypted file stays encrypted and asks for its password), then use the converted file"
    )]
    Old { found: u32, known: u32 },
    #[error("{0}")]
    Field(#[from] reiwand_core::explicit::ReadError),
    #[error("the file is encrypted and needs its password")]
    NeedsPassword,
    #[error("wrong password, or the file is damaged")]
    WrongPassword,
    #[error("the file is cut short")]
    Truncated,
    #[error("the file is too large to be a real export: {0}")]
    TooLarge(String),
    #[error("unsafe entry in the file: {0}")]
    Unsafe(String),
    #[error("a stored sum does not match: {0}")]
    Sum(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<zip::result::ZipError> for Error {
    fn from(error: zip::result::ZipError) -> Self {
        match error {
            zip::result::ZipError::Io(io) => Error::Io(io),
            other => Error::Invalid(other.to_string()),
        }
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(error: std::str::Utf8Error) -> Self {
        Error::Invalid(error.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Invalid(error.to_string())
    }
}
