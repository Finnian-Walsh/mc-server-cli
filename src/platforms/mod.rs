mod fabric;
mod paper;
pub mod purpur;

use crate::Result;
use clap::ValueEnum;
use url::Url;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Platform {
    Fabric,
    Forge,
    Neoforge,
    Paper,
    Purpur,
}

pub fn get(platform: Platform, version: Option<String>) -> Result<Url> {
    let version = version.filter(|v| v != "latest");

    let download_url = match platform {
        Platform::Fabric => fabric::get(version)?,
        Platform::Forge => todo!(),
        Platform::Neoforge => todo!(),
        Platform::Paper => paper::get(version)?,
        Platform::Purpur => purpur::get(version)?,
    };

    Ok(Url::parse(&download_url)?)
}
