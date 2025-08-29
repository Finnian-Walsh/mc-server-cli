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

    let home = home_dir()
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "Failed to get home directory"))?;
    Ok(HOME_DIR.get_or_init(|| home).as_path())
}
