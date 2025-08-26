pub mod fabric;
pub mod purpur;

use crate::{config, home};
use clap::ValueEnum;
use reqwest::{blocking, header};
use std::{
    env, fmt,
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
};
use url::Url;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Loader {
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

pub fn remove_server_with_confirmation<S>(server_name: S) -> io::Result<()>
where
    S: AsRef<str> + AsRef<Path> + for<'a> PartialEq<&'a str> + fmt::Display,
{
    if loop {
        print!(
            "Enter {} to delete the server or enter \"\" to cancel operation",
            server_name
        );
        io::stdout().flush()?;

        let mut response = String::new();
        io::stdin().read_line(&mut response)?;

        match response.as_str() {
            ref s if server_name == *s => break true,
            "" => break false,
            _ => continue,
        };
    } {
        remove_dir_with_retries(home::get()?.join(config::get("servers")?).join(server_name))?;
    }
    Ok(())
}

pub fn make_server(
    server_name: String,
    download_url: Url,
) -> Result<(), Box<dyn std::error::Error>> {
    let server_root_dir = home::get()?.join(config::get("servers")?).join(server_name);

    let server_dir = server_root_dir.join("Server");

    fs::create_dir_all(&server_dir)?;

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
        return Err(Box::new(err));
    }

    Ok(())
}
