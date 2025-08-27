use reqwest::{self, blocking};
use serde_json::Value;
use std::{io, result};
use thiserror::Error;

static BASE_API_URL: &str = "https://api.purpurmc.org/v2/purpur";

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Io(#[from] io::Error),

    #[error("No builds field was found for version {version}")]
    NoBuilds { version: String },

    #[error("No current version was found")]
    NoCurrentVersion,

    #[error("No latest build was found for version {version}")]
    NoLatestBuild { version: String },

    #[error("No metadata was found")]
    NoMetadata,

    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
}

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

pub fn new(version: Option<String>) -> Result<String> {
    let version = version.map_or_else(get_current_version, |ver| Ok(ver))?;
    let version_url = format!("{}/{}", BASE_API_URL, version);
    let response_json: Value = blocking::get(&version_url)?.json()?;

    let builds = response_json["builds"]
        .as_object()
        .ok_or_else(|| Error::NoBuilds {
            version: version.clone(),
        })?;
    let latest_build = builds["latest"]
        .as_str()
        .ok_or_else(|| Error::NoLatestBuild {
            version: version.clone(),
        })?;

    let build_url = format!("{}/{}/download", &version_url, &latest_build);

    println!(
        "Creating purpur server (v{}, build {})",
        version, latest_build
    );

    Ok(build_url)
}
