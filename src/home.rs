use dirs::home_dir;
use once_cell::sync::OnceCell;
use std::{fmt, path::{Path, PathBuf}, result};

#[derive(Debug)]
pub enum Error {
    HomeDirFailure,
    HomeDirAlreadySet,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::HomeDirFailure => write!(f, "Failed to get home directory"),
            Error::HomeDirAlreadySet => write!(f, "HOME_DIR variable has already been set"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = result::Result<T, Error>;

static HOME_DIR: OnceCell<PathBuf> = OnceCell::new();

pub fn init() -> Result<()> {
    let Some(home) = home_dir() else {
        return Err(Error::HomeDirFailure);
    };

    if let Err(_) = HOME_DIR.set(home) {
            return Err(Error::HomeDirAlreadySet);
    }

    Ok(())
}

pub fn get() -> &'static Path {
    HOME_DIR.get().unwrap()
}

