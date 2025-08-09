mod home;

use std::{fmt, fs, io, process::Command, result};

#[derive(Debug)]
pub enum Error {
    BuildFailure,
    Home(home::Error),
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::BuildFailure => write!(f, "cargo build failed"),
            Error::Home(err) => write!(f, "{}", err),
            Error::Io(e) => write!(f, "{}", e),
        }
    }
}

impl From<home::Error> for Error {
    fn from(err: home::Error) -> Self {
        Error::Home(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl std::error::Error for Error {}

pub type Result<T> = result::Result<T, Error>;

pub fn rebuild() -> Result<()> {
    let mut child = Command::new("cargo")
        .arg("build")
        .spawn()?;

    let status = child.wait()?;

    if !status.success() {
        return Err(Error::BuildFailure);
    }

    let mut child = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .spawn()?;

    let status = child.wait()?;

    if !status.success() {
        return Err(Error::BuildFailure);
    }

    fs::copy(
        "target/release/server",
        home::get()
                  .join(".local")
                  .join("bin")
                  .join("server")
    )?;

    Ok(())
}

fn main() -> Result<()> {
    home::init()?;
    rebuild()?;
    Ok(())
}
