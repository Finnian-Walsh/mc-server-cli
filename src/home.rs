use dirs::home_dir;
use std::{
    io::{Error, ErrorKind, Result},
    path::{Path, PathBuf},
    sync::OnceLock,
};

static HOME_DIR: OnceLock<PathBuf> = OnceLock::new();

pub fn get() -> Result<&'static Path> {
    // use get_or_try_init

    if let Some(home) = HOME_DIR.get() {
        return Ok(home.as_path());
    }

    match home_dir() {
        Some(home) => {
            HOME_DIR.set(home).unwrap();
            Ok(HOME_DIR.get().unwrap())
        }
        None => Err(Error::new(ErrorKind::NotFound, "Failed to get home directory")),
    }
}

