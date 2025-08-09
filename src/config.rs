use crate::home;
use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    fs,
    io::{Error, ErrorKind, Result},
    sync::Mutex
};

static CONFIG: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| Mutex::new(HashMap::new()) );

pub fn get_raw<S: Into<String>>(configuration: S) -> Result<String> {
    let mut config_map = CONFIG.lock().unwrap();

    let configuration: String = configuration.into();

    if let Some(value) = config_map.get(&configuration) {
        return Ok(value.clone())
    }

    let path = home::get()
        .join(".config")
        .join("server")
        .join(&configuration);

    let value = fs::read_to_string(&path)?;

    config_map.insert(configuration, value.clone());
    Ok(value)
}

pub fn get<S: Into<String>>(configuration: S) -> Result<String> {
    Ok(get_raw(configuration)?.trim_end().to_string())
}

pub fn get_default() -> Result<String> {
    let def = get("default")?;

    if def.len() == 0 {
        return Err(Error::new(ErrorKind::UnexpectedEof, ".default file is empty").into());
    }

    Ok(def)
}

pub fn unwrap_or_default(server: Option<String>) -> Result<String> {
    if let Some(server) = server {
        return Ok(server);
    }

    Ok(get_default()?)
}

