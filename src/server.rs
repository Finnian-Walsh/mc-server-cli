use crate::{
    config::{get_current_server_directory, get_expanded_servers_dir},
    error::{Error, Result},
    platforms::Platform,
    template,
};
use reqwest::{blocking, header};
use std::{
    collections::HashSet,
    env,
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
};
use url::Url;

fn copy_jar<J: io::Read>(server_dir: PathBuf, file_name: String, mut jar: J) -> Result<()> {
    env::set_current_dir(server_dir)?;

    let mut jar_file = File::create(&file_name)?;
    io::copy(&mut jar, &mut jar_file)?;

    let mut jarfile_txt = File::create("jarfile.txt")?;
    writeln!(jarfile_txt, "{}", file_name)?;

    Ok(())
}

pub fn copy_directory(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            copy_directory(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}

pub fn remove_dir_with_retries<P: AsRef<Path>>(dir: P) -> Result<()> {
    const ATTEMPTS: u8 = 10;

    for i in 1..=ATTEMPTS {
        if let Err(err) = fs::remove_dir_all(&dir) {
            if i == ATTEMPTS {
                return Err(Error::Io(err));
            }
        } else {
            return Ok(());
        }
    }

    unreachable!("Code returns before the for loop ends")
}

fn remove_server(server: String) -> Result<()> {
    remove_dir_with_retries(get_expanded_servers_dir()?.join(server))?;
    Ok(())
}

pub fn remove_servers(servers: Vec<String>) -> Result<()> {
    let all_servers = get_all_hashed()?;

    for server in servers {
        let server = if server == "." {
            get_current_server_directory()?
        } else {
            server
        };

        if all_servers.get(&server).as_ref().is_none() {
            return Err(Error::ServerNotFound(server));
        }

        remove_server(server)?;
    }

    Ok(())
}

pub fn remove_servers_with_confirmation(servers: Vec<String>) -> Result<()> {
    let all_servers = get_all_hashed()?;

    for server in servers {
        if all_servers.get(&server).as_ref().is_none() {
            return Err(Error::ServerNotFound(server));
        }

        if loop {
            print!(
                "Enter `{}` to delete the server or nothing to cancel operation: ",
                server
            );
            io::stdout().flush()?;

            let mut response = String::new();
            io::stdin().read_line(&mut response)?;

            if server == response.trim_end() {
                break true;
            } else if response.is_empty() {
                break false;
            }
        } {
            remove_server(server)?;
            println!("Server successfully removed");
        } else {
            println!("Operation canceled");
        }
    }

    Ok(())
}

pub fn init(download_url: Url, platform: Platform, name: Option<String>) -> Result<()> {
    let name = name.unwrap_or_else(|| format!("{:?}-server", platform).to_lowercase());
    let servers_dir = &get_expanded_servers_dir()?;

    let mut server_root_dir = servers_dir.join(&name);

    if server_root_dir.exists() {
        let mut number = 2;

        server_root_dir = loop {
            let dir = servers_dir.join(format!("{}-{}", &name, number));

            if !dir.exists() {
                break dir;
            }
            number += 1;
        }
    }

    let server_dir = server_root_dir.join("Server");
    fs::create_dir_all(&server_dir)?;

    println!("Downloading from {}...", download_url);
    let response = blocking::get(download_url)?;

    let file_name = response
        .headers()
        .get(header::CONTENT_DISPOSITION)
        .map(|disposition| disposition.to_str())
        .transpose()?
        .and_then(|cd| cd.split("filename=\"").nth(1))
        .and_then(|slice| slice.split('"').next())
        .unwrap_or("unknown.jar")
        .to_string();

    if let Err(err) = copy_jar(server_dir, file_name, response) {
        remove_dir_with_retries(server_root_dir)?;
        return Err(err);
    }

    Ok(())
}

pub fn for_each<F: FnMut(String)>(mut f: F) -> Result<()> {
    let servers_dir = get_expanded_servers_dir()?;

    if !servers_dir.exists() || !servers_dir.is_dir() {
        return Err(Error::MissingDirectory {
            dir: servers_dir.to_path_buf(),
        });
    }

    for entry in fs::read_dir(servers_dir)? {
        let entry = entry?;
        let file_name = entry.file_name().to_string_lossy().to_string();
        f(file_name);
    }

    Ok(())
}

pub fn get_all_hashed() -> Result<HashSet<String>> {
    let mut servers = HashSet::new();
    for_each(|s| {
        servers.insert(s);
    })?;
    Ok(servers)
}
