use crate::home;
use std::{
    collections::HashMap,
    fs,
    io::{Error, ErrorKind, Result},
    sync::{Mutex, OnceLock},
};

pub type ConfigType = HashMap<String, String>;

static CONFIG: OnceLock<Mutex<ConfigType>> = OnceLock::new();

pub fn get_raw<S: Into<String>>(configuration: S) -> Result<String> {
    let mut config_map = CONFIG
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .map_err(|err| Error::new(ErrorKind::Other, format!("Config poisoned: {}", err)))?;

    let configuration: String = configuration.into();

    if let Some(value) = config_map.get(&configuration) {
        return Ok(value.clone());
    }

    let path = home::get()?
        .join(".config")
        .join("server")
        .join(&configuration);

    let value = fs::read_to_string(&path)?;
    config_map.insert(configuration.to_string(), value.clone());
    Ok(value)
}

pub fn get<S: Into<String>>(configuration: S) -> Result<String> {
    let mut value = get_raw(configuration)?;
    value.truncate(value.trim_end().len());
    Ok(value)
}

pub fn get_default() -> Result<String> {
    let def = get("default")?;

    if def.len() == 0 {
        return Err(Error::new(ErrorKind::UnexpectedEof, ".default file is empty").into());
    }

    Ok(def)
}

pub fn unwrap_or_default(server: Option<String>) -> Result<String> {
    server.map_or_else(|| get_default(), |val| Ok(val))
}
