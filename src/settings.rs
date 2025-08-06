use dirs::home_dir;
use std::{fs, io};

#[macro_export]
macro_rules! get_default_or_exit {
    () => {
        match get_default() {
            Ok(default) => default,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            },
        }
    };
}

pub enum SettingsError {
    Io(io::Error),
    HomeDirFailure,
}

impl From<io::Error> for SettingsError {
    fn from(e: io::Error) -> Self {
        SettingsError::Io(e)
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

pub fn get_default() -> Result<String, String> {
    match get_setting("default") {
        Ok(value) => {
            if value.len() == 0 {
                return Err(".default file is empty!".to_string());
            }

            Ok(value.trim_end().to_string())
        },
        Err(SettingsError::Io(e)) => Err(format!("IO error: {}", e)),
        Err(SettingsError::HomeDirFailure) => Err("Failed to get home dir".to_string()),
    }
}

