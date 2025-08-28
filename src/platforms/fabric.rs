use super::{Result, get_array, get_str, to_json_object};
use reqwest::{self, blocking};
use serde_json::Value;

static BASE_API_URL: &str = "https://meta.fabricmc.net/v2/versions";

pub fn get(game_version: Option<String>) -> Result<String> {
    let response_json: Value = blocking::get(BASE_API_URL)?.json()?;
    let response_object = to_json_object(&response_json)?;

    let game_version = game_version.map_or_else::<Result<String>, _, _>(
        || {
            let game_versions_array = get_array(&response_object, "game")?;
            let stable_entry = game_versions_array
                .iter()
                .find(|game_entry| game_entry["stable"] == true)
                .unwrap();

            Ok(get_str(to_json_object(&stable_entry)?, "version")?.to_string())
        },
        |game_version| Ok(game_version),
    )?;

    let loader = response_json["loader"]
        .as_array()
        .expect("Expected loader array in response")
        .iter()
        .find(|loader_entry| loader_entry["stable"] == true)
        .expect("Expected a stable loader")
        .as_object()
        .expect("Expected loader entry to be an object")["version"]
        .as_str()
        .expect("Expected version field of loader to be a string");

    let installer = response_json["installer"]
        .as_array()
        .expect("Expected installer array in response")
        .iter()
        .find(|installer_entry| installer_entry["stable"] == true)
        .expect("Expected a stable installer")
        .as_object()
        .expect("Expected installer entry to be an object")["version"]
        .as_str()
        .expect("Expected version field of installer to be a string");

    println!(
        "Installing fabric server (v{}, loader {}, installer {})",
        game_version, loader, installer
    );

    Ok(format!(
        "{}/loader/{}/{}/{}/server/jar",
        BASE_API_URL, game_version, loader, installer
    ))
}
