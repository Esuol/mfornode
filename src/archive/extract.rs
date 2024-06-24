use std::error::Error as StdError;
use std::path::Path;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ZipError(zip::result::ZipError),
    HttpError(crate::http::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoError(x) => x.fmt(f),
            Self::ZipError(x) => x.fmt(f),
            Self::HttpError(x) => x.fmt(f),
        }
    }
}
