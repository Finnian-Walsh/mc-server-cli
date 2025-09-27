use reqwest::header;
use shellexpand;
use std::{env, io, path::PathBuf, result};
use thiserror::Error;
use toml;
use url;

#[derive(Debug, Error)]
pub enum GlobalMutex {
    #[error("CONFIG")]
    Config,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(
        "Command failed with code {}{}",
        code.map(|c| c.to_string()).as_deref().unwrap_or("none"),
        stderr
            .as_ref()
            .map(|err| format!(": {}", String::from_utf8_lossy(err)))
            .as_deref()
            .unwrap_or("")
    )]
    CommandFailure {
        code: Option<i32>,
        stderr: Option<Vec<u8>>,
    },

    #[error(transparent)]
    InvalidHeaderValue(#[from] header::InvalidHeaderValue),

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("Missing directory: {}", dir.display())]
    MissingDirectory { dir: PathBuf },

    #[error("Missing file: {}", file.display())]
    MissingFile { file: PathBuf },

    #[error("Platforms not found: {0}")]
    PlatformsNotFound(String),

    #[error("Mutex {0} is poisoned")]
    GlobalMutexPoisoned(GlobalMutex),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    ShellexpandLookup(#[from] shellexpand::LookupError<env::VarError>),

    #[error(transparent)]
    TomlDeserialize(#[from] toml::de::Error),

    #[error(transparent)]
    TomlSerialize(#[from] toml::ser::Error),

    #[error(transparent)]
    ToStr(#[from] header::ToStrError),

    #[error(transparent)]
    UrlParse(#[from] url::ParseError),
}

pub type Result<T> = result::Result<T, Error>;
