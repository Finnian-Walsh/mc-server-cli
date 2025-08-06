use dirs::home_dir;
use std::{fs, io, path::{Path, PathBuf}};

pub fn copy_dir(src: &Path, dst: &Path) -> io::Result<()> {
    for entry in fs::read_dir(src) {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(src_path);

        if file_type.is_dir() {
            copy_dir(&src_path, &dst_path);
        } else if file_type.is_file() {
            fs::copy(&src_path, &dst_path);
        }

        // skip symlinks
    }

    Ok(())
}

pub fn backup(server: &str) -> Result<(), io::Error> {
    let Ok(mut path) = home_dir() else {
        
    };

    path.push("Servers");
    path.push(server);

    let mut src_path = path.clone();
    src_path.push("Server");

    let mut dst_path = path;
    dst_path.push("Backups");
    dst_path.push("

    return Err(io::Error::new(io::ErrorKind::NotFound, format!("Directory {} does not exist")));

    Ok(())
}
