use crate::{config, home};
use std::{
    fs, io,
    path::{Path, PathBuf},
    result,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Io(#[from] io::Error),

    #[error("Server directory {0} does not exist")]
    NoServerDirectory(PathBuf),

    #[error("Jar file {0} does not exist")]
    NoJarfile(PathBuf),

    #[error("{0} does not exist, and it is needed to specify the jar path")]
    NoJarfileTxt(PathBuf),
}

pub type Result<T> = result::Result<T, Error>;

fn get_server_dir(server: &str) -> Result<PathBuf> {
    let server_dir = home::get()?
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
