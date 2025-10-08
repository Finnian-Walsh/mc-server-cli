use crate::{
    config::get_expanded_servers_dir,
    error::{Error, Result},
    server::copy_directory,
};
use std::path::{Path, PathBuf};

static TEMPLATE_SUFFIX: &str = ".template";

pub fn is_template(server: &str) -> bool {
    server.ends_with(TEMPLATE_SUFFIX)
}

pub fn new(server: &str) -> Result<()> {
    if is_template(server) {
        return Err(Error::TemplateUsedForTemplate);
    }
    println!("Creating template using server {server}...");

    let servers_dir = get_expanded_servers_dir()?;

    let server_path = servers_dir.join(server);
    if !server_path.exists() {
        return Err(Error::ServerNotFound(server.to_string()));
    }

    let template_path = servers_dir.join(format!("{server}{TEMPLATE_SUFFIX}"));
    if template_path.exists() {
        return Err(Error::TemplateAlreadyExists(server.to_string()));
    }

    copy_directory(server_path, template_path)?;

    Ok(())
}

fn get_server_path(servers_dir: impl AsRef<Path>, name: &str) -> PathBuf {
    let path = servers_dir.as_ref().join(name);
    if !path.exists() {
        return path;
    }

    let mut number = 2;
    loop {
        let path = servers_dir.as_ref().join(format!("{name}-{number}"));
        if !path.exists() {
            break path;
        }

        number += 1;
    }
}

pub fn from(template: &str, server: Option<&str>) -> Result<()> {
    let servers_dir = get_expanded_servers_dir()?;

    let template_path = if template.ends_with(TEMPLATE_SUFFIX) {
        println!("Creating server from {template}");
        servers_dir.join(template)
    } else {
        let template_name = format!("{template}{TEMPLATE_SUFFIX}");
        println!("Creating server from {template_name}");
        servers_dir.join(template_name)
    };

    if !template_path.exists() {
        return Err(Error::TemplateNotFound(template.to_string()));
    }

    let server_path = match server {
        Some(server) => {
            let path = get_expanded_servers_dir()?.join(server);
            if path.exists() {
                return Err(Error::ServerAlreadyExists(server.to_string()));
            }
            path
        }
        None => get_server_path(servers_dir, template),
    };

    copy_directory(template_path, server_path)?;

    Ok(())
}
