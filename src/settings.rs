use dirs::home_dir;
use std::{fs, io::{self}};

pub enum SettingsError {
    Io(io::Error),
    HomeDirFailure,
}

impl From<io::Error> for SettingsError {
    fn from(err: io::Error) -> Self {
        SettingsError::Io(err)
    }
}

pub fn get_setting(setting: &str) -> Result<String, SettingsError> {
    let Some(mut path) = home_dir() else {
        return Err(SettingsError::HomeDirFailure);
    };

    path.push("Servers");
    path.push(format!(".{}", setting));

    Ok(fs::read_to_string(&path)?)
}

