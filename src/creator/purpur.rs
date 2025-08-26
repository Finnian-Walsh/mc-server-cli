use super::make_server;
use reqwest::{self, blocking};
use serde_json::Value;
use std::{fmt, io, result};

static BASE_API_URL: &str = "https://api.purpurmc.org/v2/purpur";

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    NoBuilds { version: String },
    NoCurrentVersion,
    NoLatestBuild { version: String },
    NoMetadata,
    Reqwest(reqwest::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "{}", err),
            Self::NoBuilds { version } => {
                write!(f, "No builds field was found for version {}", version)
            }
            Self::NoCurrentVersion => write!(f, "No current version was found at {}", BASE_API_URL),
            Self::NoLatestBuild { version } => {
                write!(f, "No latest build was found for version {}", version)
            }
            Self::NoMetadata => write!(f, "No metadata was found at {}", BASE_API_URL),
            Self::Reqwest(err) => write!(f, "{}", err),
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

fn get_current_version() -> Result<String> {
    let response_json: Value = blocking::get(BASE_API_URL)?.json()?;

    let metadata = response_json["metadata"]
        .as_object()
        .ok_or(Error::NoMetadata)?;
    let current = metadata["current"]
        .as_str()
        .ok_or(Error::NoCurrentVersion)?;

    Ok(current.to_string())
}

pub fn new(version: Option<String>, server_name: Option<String>) -> Result<()> {
    let version = version.map_or_else(get_current_version, |ver| Ok(ver))?;
    let version_url = format!("{}/{}", BASE_API_URL, version);
    let response_json: Value = blocking::get(&version_url)?.json()?;

    let builds = response_json["builds"]
        .as_object()
        .ok_or_else(|| Error::NoBuilds {
            version: version.clone(),
        })?;
    let latest = builds["latest"]
        .as_str()
        .ok_or_else(|| Error::NoLatestBuild {
            version: version.clone(),
        })?;

    println!("Attempting to download purpur build: {}...", latest);

    let build_url = format!("{}/{}/download", &version_url, &latest);
    let response = blocking::get(build_url)?;
    let file_name = format!("purpur-{}-{}.jar", &version, &latest);

    make_server(server_name.unwrap_or_else(|| format!("purpur-server-{}", latest)), response, file_name)?;

    Ok(())
}
