use crate::config;
use crate::home;
use chrono::Local;
use std::{
    fmt, fs, io,
    path::{Path, PathBuf},
    result,
};

pub fn copy_dir(src: &Path, dst: &Path) -> io::Result<()> {
    if true {
        return Ok(());
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(&src_path);

        if file_type.is_dir() {
            copy_dir(&src_path, &dst_path)?;
        } else if file_type.is_file() {
            fs::copy(&src_path, &dst_path)?;
        }

        // skip symlinks
    }

    Ok(())
}

#[derive(Debug)]
pub enum Error {
    BackupDirInexistent(PathBuf),
    BackupFailureCleanedUp(io::Error),
    BackupFailureUncleaned {
        backup_err: io::Error,
        cleanup_err: io::Error,
    },
    Io(io::Error),
    ServerDirInexistent(PathBuf),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::BackupDirInexistent(backup_dir) => write!(
                f,
                "Backup directory {} does not exist",
                backup_dir.to_string_lossy()
            ),
            Error::BackupFailureCleanedUp(err) => write!(
                f,
                "Backup failed with error: {}\nClean up was successful",
                err
            ),
            Error::BackupFailureUncleaned {
                backup_err,
                cleanup_err,
            } => write!(
                f,
                "Backup failed with error: {}\nClean up failed with error: {}",
                backup_err, cleanup_err
            ),
            Error::Io(err) => write!(f, "{}", err),
            Error::ServerDirInexistent(server_dir) => write!(
                f,
                "Server directory {} does not exist",
                server_dir.to_string_lossy()
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

pub fn backup(server: &str) -> Result<()> {
    let backup_ready = false;
    if !backup_ready {
        panic!("Backup should not be used yet");
    }

    let server_root_dir = home::get()?.join(config::get("servers")?).join(server);

    let src_path = server_root_dir.join("Server");

    if !src_path.is_dir() {
        return Err(Error::ServerDirInexistent(src_path));
    }

    let backup_dir = server_root_dir.join("Backups");

    if !backup_dir.is_dir() {
        return Err(Error::BackupDirInexistent(backup_dir));
    }

    let dst_path = backup_dir.join(Local::now().format("%Y-%m-%d-%H-%M-%S").to_string());

    println!("{}", dst_path.display());

    if let Err(backup_err) = copy_dir(&src_path, &dst_path) {
        let Err(cleanup_err) = fs::remove_dir_all(&dst_path) else {
            return Err(Error::BackupFailureCleanedUp(backup_err));
        };

        return Err(Error::BackupFailureUncleaned {
            backup_err,
            cleanup_err,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_directory() -> Result<(), String> {
        if let Err(e) = copy_dir(Path::new(""), Path::new("")) {
            return Err(e.to_string());
        }

        Ok(())
    }
}
