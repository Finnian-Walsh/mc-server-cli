use crate::home;
use std::{
    collections::HashMap,
    fs,
    io::{Error, ErrorKind, Result},
    path::PathBuf,
    sync::{Mutex, MutexGuard, OnceLock},
};

pub type ConfigType = HashMap<String, String>;

static CONFIG: OnceLock<Mutex<ConfigType>> = OnceLock::new();

fn get_server_config_path() -> Result<PathBuf> {
    Ok(home::get()?.join(".config").join("server"))
}

fn get_config_map() -> Result<MutexGuard<'static, ConfigType>> {
    Ok(CONFIG
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .map_err(|_| Error::new(ErrorKind::Other, "CONFIG map is poisoned"))?)
}

pub fn get_raw<T: AsRef<str> + Into<String>>(configuration: T) -> Result<String> {
    let mut config_map = get_config_map()?;

    if let Some(value) = config_map.get(configuration.as_ref()) {
        return Ok(value.clone());
    }

    let path = get_server_config_path()?.join(configuration.as_ref());

    let value = fs::read_to_string(&path)?;
    config_map.insert(configuration.into(), value.clone());
    Ok(value)
}

pub fn get<T: AsRef<str> + Into<String>>(configuration: T) -> Result<String> {
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

pub fn set<T: AsRef<str> + Into<String>>(configuration: T, value: String) -> Result<()> {
    let path = get_server_config_path()?.join(configuration.as_ref());
    fs::write(path, &value)?;

    let mut config_map = get_config_map()?;
    config_map.insert(configuration.into(), value);

    Ok(())
}
