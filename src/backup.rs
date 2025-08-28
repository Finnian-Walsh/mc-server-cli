use crate::home;
use crate::{
    config,
    error::{Error, Result},
};
use chrono::Local;
use std::{fs, io, path::Path};

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

pub fn backup(server: &str) -> Result<()> {
    let backup_ready = false;
    if !backup_ready {
        panic!("Backup should not be used yet");
    }

    let server_root_dir = home::get()?.join(config::get("servers")?).join(server);

    let src_path = server_root_dir.join("Server");

    if !src_path.is_dir() {
        return Err(Error::MissingDirectory(Some(src_path)));
    }

    let backup_dir = server_root_dir.join("Backups");

    if !backup_dir.is_dir() {
        return Err(Error::MissingDirectory(Some(backup_dir)));
    }

    let dst_path = backup_dir.join(Local::now().format("%Y-%m-%d-%H-%M-%S").to_string());

    println!("{}", dst_path.display());

    if let Err(backup_err) = copy_dir(&src_path, &dst_path) {
        fs::remove_dir_all(&dst_path)?;
        return Err(Error::Io(backup_err));
    }

    Ok(())
}
