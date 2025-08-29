use super::reqwest_client;
use crate::error::{Error, Result};
use serde::Deserialize;

static BASE_API_URL: &str = "https://api.papermc.io/v2/projects/paper";

static BASE_DOWNLOAD_URL: &str = "https://fill-data.papermc.io/v1/objects";

#[derive(Debug, Deserialize)]
struct ProjectInfo {
    versions: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct BuildsInfo {
    builds: Vec<Build>,
}

#[derive(Debug, Deserialize)]
struct Build {
    downloads: Downloads,
}

#[derive(Debug, Deserialize)]
struct Downloads {
    application: Application,
}

#[derive(Debug, Deserialize)]
struct Application {
    name: String,
    sha256: String,
}

pub fn get(version: Option<String>) -> Result<String> {
    let client = reqwest_client::create()?;

    let version = version.map_or_else(
        || {
            let project_info: ProjectInfo = client.get(BASE_API_URL).send()?.json()?;
            let mut versions = project_info.versions;
            Ok::<_, Error>(versions.pop().unwrap())
        },
        |version| Ok(version),
    )?;

    let builds: Vec<Build> = client
        .get(format!("{}/versions/{}/builds", BASE_API_URL, version))
        .send()?
        .json::<BuildsInfo>()?
        .builds;
    let application = &builds[builds.len() - 1].downloads.application;

    let download_url = format!(
        "{}/{}/{}",
        BASE_DOWNLOAD_URL, application.sha256, application.name
    );

    Ok(download_url)
}
