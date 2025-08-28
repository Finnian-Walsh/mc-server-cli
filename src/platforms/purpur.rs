use super::{Result, get_object, get_str, to_json_object};
use reqwest::{self, blocking};
use serde_json::Value;

static BASE_API_URL: &str = "https://api.purpurmc.org/v2/purpur";

fn get_current_version() -> Result<String> {
    let response_json: Value = blocking::get(BASE_API_URL)?.json()?;
    let response_object = to_json_object(&response_json)?;

    let metadata = get_object(&response_object, "metadata")?;
    let current = get_str(metadata, "current")?;

    Ok(current.to_string())
}

pub fn get(version: Option<String>) -> Result<String> {
    let version = version.map_or_else(get_current_version, |ver| Ok(ver))?;
    let version_url = format!("{}/{}", BASE_API_URL, version);
    let response_json: Value = blocking::get(&version_url)?.json()?;
    let response_object = to_json_object(&response_json)?;

    let builds = get_object(&response_object, "builds")?;
    let latest = get_str(builds, "latest")?;
    let build_url = format!("{}/{}/download", &version_url, &latest);

    println!("Creating purpur server (v{}, build {})", version, latest);

    Ok(build_url)
}
