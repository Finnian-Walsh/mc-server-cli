use reqwest::header;
use std::{
    fmt::{self, Display, Formatter},
    io,
    path::PathBuf,
    result,
};
use thiserror::Error;
use url;

#[derive(Debug, Error)]
pub enum Error {
    CommandFailure { code: Option<i32>, stderr: Vec<u8> },
    EmptyFile(Option<PathBuf>),
    InvalidHeaderValue(#[from] header::InvalidHeaderValue),
    Io(#[from] io::Error),
    MissingDirectory(Option<PathBuf>),
    MissingFile(Option<PathBuf>),
    PlatformsNotFound(String),
    Poison(Option<String>),
    Reqwest(#[from] reqwest::Error),
    ToStr(#[from] header::ToStrError),
    UrlParse(#[from] url::ParseError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::CommandFailure { code, stderr } => write!(
                f,
                "Command faile with code {}: {}",
                code.map(|c| c.to_string())
                    .unwrap_or_else(|| String::from("none")),
                String::from_utf8_lossy(stderr)
            ),
            Self::EmptyFile(file) => match file {
                Some(file) => write!(f, "File {} is empty", file.display()),
                None => write!(f, "Empty file"),
            },
            Self::InvalidHeaderValue(err) => write!(f, "{}", err),
            Self::Io(err) => write!(f, "{}", err),
            Self::MissingDirectory(dir) => match dir {
                Some(dir) => write!(f, "Directory {} is missing", dir.display()),
                None => write!(f, "Missing directory"),
            },
            Self::MissingFile(file) => match file {
                Some(file) => write!(f, "File {} is missing", file.display()),
                None => write!(f, "Missing file"),
            },
            Self::PlatformsNotFound(value) => write!(f, "Platforms not found: {}", value),
            Self::Poison(object) => match object {
                Some(object) => write!(f, "{} is poisoned", object),
                None => write!(f, "Poison error"),
            },
            Self::Reqwest(err) => write!(f, "{}", err),
            Self::ToStr(err) => write!(f, "{}", err),
            Self::UrlParse(err) => write!(f, "{}", err),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;
