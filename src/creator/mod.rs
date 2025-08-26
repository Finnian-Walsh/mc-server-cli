pub mod fabric;
pub mod purpur;

use crate::{config, home};
use clap::ValueEnum;
use std::{
    env,
    fs::{self, File},
    io,
    path::PathBuf,
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Loader {
    Fabric,
    Forge,
    Neoforge,
    Paper,
    Purpur,
}

pub fn make_server<T>(server_name: String, mut jar: T, jar_file_name: String) -> io::Result<()>
where
    T: io::Read,
{
    let server_dir = PathBuf::new()
        .join(home::get()?)
        .join(config::get("servers")?)
        .join(server_name)
        .join("Server");

    fs::create_dir_all(&server_dir)?;
    env::set_current_dir(server_dir)?;

    let mut jar_file = File::create(jar_file_name)?;
    io::copy(&mut jar, &mut jar_file)?;
    Ok(())
}
