pub mod fabric;
pub mod purpur;

use crate::{config, home};
use clap::ValueEnum;
use reqwest::{blocking, header};
use std::{
    env,
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Platform {
    Fabric,
    Forge,
    Neoforge,
    Paper,
    Purpur,
}

fn copy_jar<N, J: io::Read>(server_dir: PathBuf, file_name: N, mut jar: J) -> io::Result<()>
where
    N: AsRef<[u8]> + AsRef<Path>,
    J: io::Read,
{
    env::set_current_dir(server_dir)?;

    let mut jar_file = File::create(&file_name)?;
    io::copy(&mut jar, &mut jar_file)?;

    fs::write("jarfile.txt", file_name)?;

    Ok(())
}

pub fn remove_dir_with_retries<P: AsRef<Path>>(dir: P) -> io::Result<()> {
    const ATTEMPTS: u8 = 10;

    for i in 1..=ATTEMPTS {
        if let Err(err) = fs::remove_dir_all(&dir) {
            if i == ATTEMPTS {
                return Err(err);
            }
        } else {
            return Ok(());
        }
    }

    unreachable!("Code returns before the for loop ends")
}

pub fn remove_server_with_confirmation(name: String) -> io::Result<()> {
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
        remove_dir_with_retries(home::get()?.join(config::get("servers")?).join(name))?;
    }
    Ok(())
}

pub fn make_server(
    platform: Platform,
    version: Option<String>,
    name: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let download_url = match platform {
        Platform::Fabric => fabric::new(version)?,
        Platform::Forge => todo!(),
        Platform::Neoforge => todo!(),
        Platform::Paper => todo!(),
        Platform::Purpur => purpur::new(version)?,
    };

    println!("Making server...");

    let name = name.unwrap_or_else(|| format!("{:?}-server", platform).to_lowercase());

    let server_root_dir = home::get()?.join(config::get("servers")?).join(name);
    let server_dir = server_root_dir.join("Server");

    fs::create_dir_all(&server_dir)?;

    println!("{}", download_url);

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

    println!("{}", file_name);

    if let Err(err) = copy_jar(server_dir, file_name, response) {
        remove_dir_with_retries(server_root_dir)?;
        return Err(err.into());
    }

    Ok(())
}
