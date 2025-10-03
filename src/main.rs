mod cli;
mod config;
mod deployment;
mod error;
mod home;
mod platforms;
mod repo;
mod reqwest_client;
mod server;
mod zellij;

use clap::Parser;
use cli::*;
use color_eyre::eyre::{Result, WrapErr};
use config::unwrap_or_default;

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();

    match args.command {
        Commands::Attach { session } => zellij::attach(unwrap_or_def_server!(session)?)
            .wrap_err("Failed to attach to zellij session")?,
        Commands::Config { config_type } => match config_type {
            ConfigType::Static => println!("{:?}", config::get_static()),
            ConfigType::Dynamic => println!("{:?}", config::get()?),
        },
        Commands::Default { action } => match action {
            DefaultCommands::Get => println!("{}", config::get()?.default_server),
            DefaultCommands::Set { server } => config::get()?.default_server = server,
        },
        Commands::Deploy { server } => {
            let server = unwrap_or_def_server!(server)?;
            zellij::new(&server, Some(&deployment::get_command(&server)?))?;
        }
        Commands::Execute { server, commands } => {
            let server = unwrap_or_def_server!(server)?;
            for command in commands {
                zellij::write_line(&server, command)?;
            }
        }
        Commands::List { active, inactive } => {
            let mut servers = server::get_all().wrap_err("Failed to get servers")?;

            if active {
                zellij::retain_active(&mut servers).wrap_err("Failed to retain active servers")?;
            } else if inactive {
                zellij::retain_inactive(&mut servers)
                    .wrap_err("Failed to retain inactive servers")?;
            } else {
                zellij::tag_active(&mut servers).wrap_err("Failed to tag active servers")?;
            }

            println!("{}", servers.join("\n"));
        }
        Commands::New {
            platform,
            version,
            name,
        } => server::init(
            platforms::get(platform, version)
                .wrap_err(format!("Failed to get {:?} download url", platform))?,
            platform,
            name,
        )
        .wrap_err(format!("Failed to initialize {:?} server", platform))?,
        Commands::Stop { server } => {
            let server = unwrap_or_def_server!(server)?;
            zellij::write_line(&server, "stop")
                .wrap_err_with(|| format!("Failed to write stop to server {}", server))?;
        }
        Commands::Remove { server } => {
            server::remove_server_with_confirmation(server).wrap_err("Failed to remove server")?
        }
        Commands::Update { git, commit, path } => {
            if let Some(path) = path {
                repo::update_with_path(&path)
                    .wrap_err(format!("Failed to update package with {}", path.display()))?;
            } else if git {
                repo::update_with_git(commit).wrap_err("Failed to update package with git repo")?;
            } else {
                unreachable!("Clap ensures git or some is provided");
            }
        }
    };

    Ok(())
}
