use crate::error::{Error, Result};
use reqwest::{self, blocking};
use serde::Deserialize;

const BASE_API_URL: &str = "https://meta.fabricmc.net/v2/versions";

#[derive(Debug, Deserialize)]
struct Versions {
    game: Vec<Entry>,
    loader: Vec<Entry>,
    installer: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
struct Entry {
    version: String,
    stable: bool,
}

fn first_stable(entries: Vec<Entry>) -> Option<Entry> {
    entries.into_iter().find(|entry| entry.stable)
}

pub fn get(game_version: Option<String>) -> Result<String> {
    let versions: Versions = blocking::get(BASE_API_URL)?.json()?;

    let game_version = game_version.map_or_else(
        || {
            first_stable(versions.game)
                .map(|e| e.version)
                .ok_or_else(|| Error::PlatformsNotFound(String::from("stable game version")))
        },
        Ok,
    )?;
    let loader_version = first_stable(versions.loader)
        .ok_or_else(|| Error::PlatformsNotFound(String::from("stable loader")))?
        .version;
    let installer_version = first_stable(versions.installer)
        .ok_or_else(|| Error::PlatformsNotFound(String::from("stable installer")))?
        .version;

    Ok(format!(
        "{BASE_API_URL}/loader/{game_version}/{loader_version}/{installer_version}/server/jar",
    ))
}
