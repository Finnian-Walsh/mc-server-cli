use crate::{
    config,
    error::{Error, Result},
    home,
    platforms::Platform,
};
use reqwest::{blocking, header};
use std::{
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

pub fn remove_server_with_confirmation(name: String) -> Result<()> {
    if loop {
        print!(
            "Enter {} to delete the server or nothing to cancel operation: ",
            name
        );
        io::stdout().flush()?;

        let mut response = String::new();
        io::stdin().read_line(&mut response)?;

        if name == response.trim_end() {
            break true;
        } else if response.is_empty() {
            break false;
        }
    } {
        remove_dir_with_retries(
            home::get()?
                .join(&config::get_expanded_servers_dir()?)
                .join(name),
        )?;
    }
    Ok(())
}

pub fn init(download_url: Url, platform: Platform, name: Option<String>) -> Result<()> {
    let name = name.unwrap_or_else(|| format!("{:?}-server", platform).to_lowercase());
    let servers_dir = &config::get_expanded_servers_dir()?;

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
        .and_then(|slice| slice.split('"').nth(0))
        .unwrap_or("unknown.jar")
        .to_string();

    if let Err(err) = copy_jar(server_dir, file_name, response) {
        remove_dir_with_retries(server_root_dir)?;
        return Err(err.into());
    }

    Ok(())
}

pub fn get_all() -> Result<Vec<String>> {
    let mut servers = vec![];

    let servers_dir = config::get_expanded_servers_dir()?;

    if !servers_dir.exists() || !servers_dir.is_dir() {
        return Err(Error::MissingDirectory(Some(servers_dir.to_path_buf())));
    }

    for entry in fs::read_dir(servers_dir)? {
        let entry = entry?;
        servers.push(entry.file_name().to_string_lossy().into_owned());
    }

    Ok(servers)
}

