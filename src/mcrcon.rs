use crate::{
    config,
    error::{Error, Result},
};
use std::process::Command;

pub fn run(server: String) -> Result<()> {
    let config = config::get()?;
    let mcrcon_config = &config.mcrcon;

    let server_mcrcon_config = mcrcon_config
        .get(&server)
        .ok_or_else(|| Error::MissingMcrconConfig(server))?;

    let mut command = Command::new("mcrcon");

    if let Some(server_address) = &server_mcrcon_config.server_address {
        command.arg("-H");
        command.arg(server_address);
    }

    if let Some(port) = &server_mcrcon_config.port {
        command.arg("-P");
        command.arg(port.to_string());
    }

    if let Some(password) = &server_mcrcon_config.password {
        command.arg("-p");
        command.arg(password);
    }

    command.status()?;

    Ok(())
}
