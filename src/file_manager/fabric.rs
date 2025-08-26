use reqwest::{self, blocking};
use serde_json::Value;
use std::{fmt, io, result};

static BASE_FABRIC_URL: &str = "https://meta.fabricmc.net/v2/versions";

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Reqwest(reqwest::Error),
    StableInstallerNotFound,
    StableLoaderNotFound,
    StableVersionNotFound,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "{}", err),
            Self::Reqwest(err) => write!(f, "{}", err),
            Self::StableInstallerNotFound => write!(f, "No stable fabric installer could be found"),
            Self::StableLoaderNotFound => write!(f, "No stable fabric loader could be found"),
            Self::StableVersionNotFound => write!(f, "No stable fabric version could be found"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
    }
}

impl std::error::Error for Error {}

pub type Result<T> = result::Result<T, Error>;

pub fn new(game_version: Option<String>) -> Result<String> {
    let response_json: Value = blocking::get(BASE_FABRIC_URL)?.json()?;

    let game_version = game_version.map_or_else(
        || {
            let latest_stable_entry = response_json["game"]
                .as_array()
                .ok_or(Error::StableVersionNotFound)?
                .iter()
                .find(|game_version_entry| game_version_entry["stable"] == true)
                .ok_or(Error::StableVersionNotFound)?
                .as_object()
                .ok_or(Error::StableVersionNotFound)?;

            latest_stable_entry["version"]
                .as_str()
                .map(|ver| ver.to_string())
                .ok_or(Error::StableVersionNotFound)
        },
        |ver| Ok(ver),
    )?;

    let loader = response_json["loader"]
        .as_array()
        .ok_or(Error::StableLoaderNotFound)?
        .iter()
        .find(|loader_entry| loader_entry["stable"] == true)
        .ok_or(Error::StableLoaderNotFound)?
        .as_object()
        .ok_or(Error::StableLoaderNotFound)?["version"]
        .as_str()
        .ok_or(Error::StableLoaderNotFound)?;

    let installer = response_json["installer"]
        .as_array()
        .ok_or(Error::StableInstallerNotFound)?
        .iter()
        .find(|installer_entry| installer_entry["stable"] == true)
        .ok_or(Error::StableInstallerNotFound)?["version"]
        .as_str()
        .ok_or(Error::StableInstallerNotFound)?;

    println!(
        "Installing fabric server (v{}, loader {}, installer {}",
        game_version, loader, installer
    );

    Ok(format!(
        "{}/loader/{}/{}/{}/server/jar",
        BASE_FABRIC_URL, game_version, loader, installer
    ))
}
