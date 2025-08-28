pub mod fabric;
pub mod purpur;

use clap::ValueEnum;
use std::{io, result};
use thiserror::Error;
use url::{self, Url};

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("Field {field} of type {json_type} not found")]
    JsonFieldNotFound {
        field: String,
        json_type: &'static str,
    },

    #[error("wrong type: expected JSON {expected}")]
    JsonExpectedType { expected: String },

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    ToStr(#[from] reqwest::header::ToStrError),

    #[error(transparent)]
    UrlParse(#[from] url::ParseError),
}

pub type Result<T> = result::Result<T, Error>;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Platform {
    Fabric,
    Forge,
    Neoforge,
    Paper,
    Purpur,
}

pub fn get(platform: Platform, version: Option<String>) -> Result<Url> {
    let download_url = match platform {
        Platform::Fabric => fabric::get(version)?,
        Platform::Forge => todo!(),
        Platform::Neoforge => todo!(),
        Platform::Paper => todo!(),
        Platform::Purpur => purpur::get(version)?,
    };

    Ok(Url::parse(&download_url)?)
}

type JsonArray = Vec<serde_json::Value>;

type JsonObject = serde_json::Map<String, serde_json::Value>;

pub fn get_array<'a>(parent: &'a JsonObject, field: &str) -> Result<&'a JsonArray> {
    parent[field]
        .as_array()
        .ok_or_else(|| Error::JsonFieldNotFound {
            field: field.to_string(),
            json_type: "array",
        })
}

pub fn get_object<'a>(parent: &'a JsonObject, field: &str) -> Result<&'a JsonObject> {
    parent[field]
        .as_object()
        .ok_or_else(|| Error::JsonFieldNotFound {
            field: field.to_string(),
            json_type: "object",
        })
}

pub fn get_str<'a>(parent: &'a JsonObject, field: &str) -> Result<&'a str> {
    parent[field]
        .as_str()
        .ok_or_else(|| Error::JsonFieldNotFound {
            field: field.to_string(),
            json_type: "string",
        })
}

pub fn to_json_object<'a>(value: &'a serde_json::Value) -> Result<&'a JsonObject> {
    value.as_object().ok_or_else(|| Error::JsonExpectedType {
        expected: String::from("object"),
    })
}
