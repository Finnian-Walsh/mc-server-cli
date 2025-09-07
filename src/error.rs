use reqwest::header;
use shellexpand;
use std::{
    env,
    fmt::{self, Display, Formatter},
    io,
    path::PathBuf,
    result,
};
use thiserror::Error;
use toml;
use url;

#[derive(Debug, Error)]
pub enum Mutexes {
    #[error("CONFIG")]
    Config,
}

#[derive(Debug, Error)]
pub enum Error {
    CommandFailure {
        code: Option<i32>,
        stderr: Option<Vec<u8>>,
    },
    InvalidHeaderValue(#[from] header::InvalidHeaderValue),
    Io(#[from] io::Error),
    MissingDirectory(Option<PathBuf>),
    MissingFile(Option<PathBuf>),
    PlatformsNotFound(String),
    Poison(Mutexes),
    Reqwest(#[from] reqwest::Error),
    ShellexpandLookup(#[from] shellexpand::LookupError<env::VarError>),
    TomlDeserialize(#[from] toml::de::Error),
    TomlSerialize(#[from] toml::ser::Error),
    ToStr(#[from] header::ToStrError),
    UrlParse(#[from] url::ParseError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::CommandFailure { code, stderr } => write!(
                f,
                "Command failed with code {}{}",
                code.map(|c| c.to_string()).as_deref()
                    .unwrap_or("none"),
                stderr
                    .as_ref()
                    .map(|err| format!(": {}", String::from_utf8_lossy(err)))
                    .as_deref()
                    .unwrap_or("")
            ),
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
            Self::Poison(mutex) => write!(f, "Mutex {} is poisoned", mutex),
            Self::Reqwest(err) => write!(f, "{}", err),
            Self::ShellexpandLookup(err) => write!(f, "{}", err),
            Self::TomlDeserialize(err) => write!(f, "{}", err),
            Self::TomlSerialize(err) => write!(f, "{}", err),
            Self::ToStr(err) => write!(f, "{}", err),
            Self::UrlParse(err) => write!(f, "{}", err),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;
