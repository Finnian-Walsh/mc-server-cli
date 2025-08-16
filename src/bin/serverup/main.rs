mod cli;
mod updater;

use cli::*;
use git2::Repository;
use server::{config, home};
use std::{fmt, fs, io, process::Command, result};

fn main() -> updater::Result<()> {
    home::init()?;

    let cli = Cli::parse();

    match cli.command {
        Commands::SelfCmd { command } => match command {
            SelfCommands::Update { local } => {
                if local {
                    updater::self_local_update()?;
                } else {
                    updater::self_remote_update()?;
                }
            }
        }
        Commands::Update { local } => {
            if local {
                updater::local_update()?;
            } else {
                updater::remote_update()?;
            }
        }
    }
    

    Ok(())
}
