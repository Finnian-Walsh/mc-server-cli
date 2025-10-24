use crate::{
    error::{Error, Result},
    reqwest_client,
};
use serde::Deserialize;

const BASE_API_URL: &str = "https://api.papermc.io/v2/projects/paper";

const BASE_DOWNLOAD_URL: &str = "https://fill-data.papermc.io/v1/objects";

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
        Ok,
    )?;

    let builds: Vec<Build> = client
        .get(format!("{BASE_API_URL}/versions/{version}/builds"))
        .send()?
        .json::<BuildsInfo>()?
        .builds;
    let application = &builds[builds.len() - 1].downloads.application;

    let download_url = format!(
        "{BASE_DOWNLOAD_URL}/{}/{}",
        application.sha256, application.name
    );

    Ok(download_url)
}
