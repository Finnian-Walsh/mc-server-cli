use crate::{config, home};
use std::{
    fmt, fs, io,
    path::{Path, PathBuf},
    result,
};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    NoServerDirectory(PathBuf),
    NoJarfile(PathBuf),
    NoJarfileTxt(PathBuf),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(err) => write!(f, "{}", err),
            Error::NoServerDirectory(dir) => write!(
                f,
                "Server directory {} does not exist",
                dir.to_string_lossy()
            ),
            Error::NoJarfile(jarfile) => {
                write!(f, "Jar file {} does not exist", jarfile.to_string_lossy())
            }
            Error::NoJarfileTxt(jarfile_txt) => write!(
                f,
                "{} does not exist, and it is needed to specify the jar path",
                jarfile_txt.to_string_lossy()
            ),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl std::error::Error for Error {}

pub type Result<T> = result::Result<T, Error>;

fn get_server_dir(server: &str) -> Result<PathBuf> {
    let server_dir = home::get()
        .join(config::get("servers")?)
        .join(server)
        .join("Server");

    if !server_dir.is_dir() {
        return Err(Error::NoServerDirectory(server_dir));
    }

    Ok(server_dir)
}

fn get_server_jar_path(server_dir: &Path) -> Result<PathBuf> {
    let jarfile_txt = server_dir.join("jarfile.txt");

    if !jarfile_txt.is_file() {
        return Err(Error::NoJarfileTxt(jarfile_txt));
    }

    let jarfile_path = server_dir.join(fs::read_to_string(jarfile_txt)?.trim_end());

    if !jarfile_path.is_file() {
        return Err(Error::NoJarfile(jarfile_path));
    }

    Ok(jarfile_path)
}

pub fn get_command(server: &str) -> Result<String> {
    let server_dir = get_server_dir(server)?;
    Ok(format!(
        "cd {} && java -jar {} {}",
        server_dir.to_string_lossy(),
        config::get("arguments")?,
        get_server_jar_path(&server_dir)?.to_string_lossy()
    ))
}
