use crate::config;
use crate::home;
use chrono::Local;
use std::{
    fs, io,
    path::{Path, PathBuf},
    result,
};
use thiserror::Error;

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

#[derive(Debug, Error)]
pub enum Error {
    #[error("Backup directory {0} does not exist")]
    BackupDirInexistent(PathBuf),

    #[error("Backup failed with error: {0}\nClean up was successful")]
    BackupFailureCleanedUp(io::Error),

    #[error("Backup failed with error: {backup_err}\nClean up failed with error: {cleanup_err}")]
    BackupFailureUncleaned {
        backup_err: io::Error,
        cleanup_err: io::Error,
    },

    #[error("{0}")]
    Io(#[from] io::Error),

    #[error("Server directory {0} does not exist")]
    ServerDirInexistent(PathBuf),
}

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
