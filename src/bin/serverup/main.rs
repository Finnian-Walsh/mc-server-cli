mod cli;
mod updater;

use clap::Parser;
use cli::*;

fn main() -> updater::Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::SelfCmd { command } => match command {
            SelfCommands::Update { local } => {
                if local {
                    updater::local_self_update()?;
                } else {
                    updater::remote_self_update()?;
                }
            }
        },
        Commands::Update { local } => {
            if local {
                updater::local_update()?;
            } else {
                updater::remote_update()?;
            }

            println!("Updated server");
        }
    }

    Ok(())
}
