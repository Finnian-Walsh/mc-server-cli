mod backup;
mod cli;
mod config;
mod deployment;
mod error;
mod home;
mod platforms;
mod repo;
mod server;
mod tmux_interactor;

use clap::Parser;
use cli::*;
use color_eyre::eyre::Result;
use config::unwrap_or_default;

fn handle(args: Cli) -> Result<()> {
    match args.command {
        Commands::Attach { session } => {
            let session = unwrap_or_default(session)?;
            tmux_interactor::attach(&session)?;
        }
        Commands::Backup { server } => {
            let server = unwrap_or_default(server)?;
            println!("Attempting to back up {}", server);
            backup::backup(&server)?;
        }
        Commands::Default { action } => match action {
            DefaultCommands::Get => println!("{}", config::get_default()?),
            DefaultCommands::Set { server } => config::set("default", server)?,
        },
        Commands::Deploy { server } => {
            let server = unwrap_or_default(server)?;
            tmux_interactor::new(Some(&server), Some(&deployment::get_command(&server)?))?;
        }
        Commands::Execute { server, command } => {
            let server = unwrap_or_default(server)?;
            tmux_interactor::execute(server, command)?;
        }
        Commands::List { active, inactive } => {
            if active {
                if inactive {
                    eprintln!("Cannot output");
                    return Ok(());
                }

                println!("{}", tmux_interactor::get_active_servers()?.join("\n"));
            } else if inactive {
                println!("{}", tmux_interactor::get_inactive_servers()?.join("\n"));
            } else {
                println!("{}", tmux_interactor::get_servers()?.join("\n"));
            }
        }
        Commands::New {
            platform,
            version,
            name,
        } => {
            server::init(platforms::get(platform, version)?, platform, name)?;
        }
        Commands::Stop { server } => {
            let server = unwrap_or_default(server)?;
            tmux_interactor::execute(server, "stop")?;
        }
        Commands::Remove { server } => server::remove_server_with_confirmation(server)?,
        Commands::Update { git, commit, path } => {
            if let Some(path) = path {
                repo::update_with_path(path)?;
            } else if git {
                repo::update_with_git(commit)?;
            } else {
                unreachable!("Clap ensures git or some is provided");
            }
        }
    };

    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();
    color_eyre::install()?;

    if let Err(err) = handle(args) {
        eprintln!("{}", err);
    }

    Ok(())
}
