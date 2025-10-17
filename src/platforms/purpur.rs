use crate::error::Result;
use reqwest::{self, blocking};
use serde::Deserialize;

static BASE_API_URL: &str = "https://api.purpurmc.org/v2/purpur";

#[derive(Debug, Deserialize)]
struct ProjectInfo {
    metadata: Metadata,
}

#[derive(Debug, Deserialize)]
struct Metadata {
    current: String,
}

#[derive(Debug, Deserialize)]
struct VersionInfo {
    builds: Builds,
}

#[derive(Debug, Deserialize)]
struct Builds {
    latest: String,
}

fn get_current_version() -> Result<String> {
    let project_info: ProjectInfo = blocking::get(BASE_API_URL)?.json()?;
    Ok(project_info.metadata.current)
}

pub fn get(version: Option<String>) -> Result<String> {
    let version = version.map_or_else(get_current_version, Ok)?;
    let version_url = format!("{}/{}", BASE_API_URL, version);
    let version_info: VersionInfo = blocking::get(&version_url)?.json()?;

    let latest = version_info.builds.latest;
    println!("Creating purpur server (v{}, build {})", version, latest);

    let download_url = format!("{}/{}/download", version_url, latest);
    Ok(download_url)
}
