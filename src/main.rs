mod backup;
mod cli;
mod config;
mod deployment;
mod file_manager;
mod home;
mod repo;
mod tmux;

use clap::Parser;
use cli::*;
use config::unwrap_or_default;
use url::Url;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match args.command {
        Commands::Attach { session } => {
            let session = unwrap_or_default(session)?;
            tmux::attach(&session)?;
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
            tmux::new(Some(&server), Some(&deployment::get_command(&server)?))?;
        }
        Commands::Execute { server, command } => {
            let server = unwrap_or_default(server)?;
            tmux::execute(server, command)?;
        }
        Commands::List { active, inactive } => {
            if active {
                if inactive {
                    eprintln!("Cannot output");
                    return Ok(());
                }

                println!("{}", tmux::get_active_servers()?.join("\n"));
            } else if inactive {
                println!("{}", tmux::get_inactive_servers()?.join("\n"));
            } else {
                println!("{}", tmux::get_servers()?.join("\n"));
            }
        }
        Commands::New {
            loader,
            version,
            name,
        } => {
            use file_manager::*;

            make_server(
                name.unwrap_or_else(|| format!("{:?}-server", loader).to_lowercase()),
                Url::parse(&match loader {
                    Loader::Fabric => fabric::new(version)?,
                    Loader::Purpur => purpur::new(version)?,
                    _ => "unhandled".to_string(),
                })?,
            )?;
        }
        Commands::Stop { server } => {
            let server = unwrap_or_default(server)?;
            tmux::execute(server, "stop")?;
        }
        Commands::Remove { server } => file_manager::remove_server_with_confirmation(server)?,
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
