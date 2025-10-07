use reqwest::header;
use shellexpand;
use std::{
    env, io,
    path::{self, PathBuf},
    result,
};
use thiserror::Error;
use toml;
use url;

#[derive(Debug, Error)]
pub enum GlobalMutex {
    #[error("CONFIG")]
    Config,
}

#[non_exhaustive]
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

    #[error("Invalid servers directory")]
    InvalidServersDirectory,

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("Missing directory: {}", dir.display())]
    MissingDirectory { dir: PathBuf },

    #[error("Missing file: {}", file.display())]
    MissingFile { file: PathBuf },

    #[error("No server child was given")]
    NoServerChild,

    #[error("No server {0} was found")]
    NoServerFound(String),

    #[error("Platforms not found: {0}")]
    PlatformsNotFound(String),

    #[error("Mutex {0} is poisoned")]
    GlobalMutexPoisoned(GlobalMutex),

    #[error("Mcrcon config is missing for server: {0}")]
    MissingMcrconConfig(String),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    ShellexpandLookup(#[from] shellexpand::LookupError<env::VarError>),

    #[error(transparent)]
    StripPrefix(#[from] path::StripPrefixError),

    #[error("Template {0} already exists")]
    TemplateAlreadyExists(String),

    #[error("Template with the name {0} was not found")]
    TemplateNotFound(String),

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
