mod backup;
mod cli;
mod config;
mod deployment;
mod home;
mod tmux;

use cli::*;
use clap::Parser;
use config::unwrap_or_default;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    home::init()?;

    match cli.command {
        Commands::Attach { session } => {
            let session = unwrap_or_default(session)?;
            tmux::attach(&session)?;
        },
        Commands::Backup { server } => {
            let server = unwrap_or_default(server)?;
            println!("Attempting to back up {}", server);
            backup::backup(&server)?;
        },
        Commands::Deploy { server } => {
            let server = unwrap_or_default(server)?;
            tmux::new(Some(&server), Some(&deployment::get_command(&server)?))?;
        },
        _ => { println!("not implemented yet"); },
    };

    Ok(())
}
