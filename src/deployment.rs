use crate::{
    config,
    error::{Error, Result},
    template::is_template,
};
use std::{
    fs,
    path::{Path, PathBuf},
};

fn get_server_dir(server: &str) -> Result<PathBuf> {
    let server_dir = config::get_expanded_servers_dir()?
        .join(server)
        .join("Server");

    if !server_dir.is_dir() {
        return Err(Error::MissingDirectory { dir: server_dir });
    }

    Ok(server_dir)
}

fn get_server_jar_path(server_dir: &Path) -> Result<PathBuf> {
    let jarfile_txt = server_dir.join("jarfile.txt");

    if !jarfile_txt.is_file() {
        return Err(Error::MissingFile { file: jarfile_txt });
    }

    let jarfile_path = server_dir.join(fs::read_to_string(jarfile_txt)?.trim_end());

    if !jarfile_path.is_file() {
        return Err(Error::MissingFile { file: jarfile_path });
    }

    Ok(jarfile_path)
}

pub fn get_command(server: &str) -> Result<String> {
    if is_template(server) {
        return Err(Error::TemplateDeployed);
    }

    let server_dir = get_server_dir(server)?;
    let config = &config::get()?;
    Ok(format!(
        "cd {} && java -jar {} {}{} && zellij kill-session $ZELLIJ_SESSION_NAME",
        server_dir.to_string_lossy(),
        config.default_java_args,
        get_server_jar_path(&server_dir)?.to_string_lossy(),
        if config.nogui { " nogui" } else { "" }
    ))
}
