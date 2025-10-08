mod cli;
mod config;
mod deployment;
mod error;
mod mcrcon;
mod platforms;
mod repo;
mod reqwest_client;
mod server;
mod template;
mod zellij;

use clap::Parser;
use cli::*;
use color_eyre::eyre::{Result, WrapErr};

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();

    match args.command {
        Commands::Attach { server } => zellij::attach(handle_server_arg!(server))
            .wrap_err("Failed to attach to zellij session")?,
        Commands::Config { config_type } => match config_type {
            ConfigType::Static => println!("{:#?}", config::get_static()),
            ConfigType::Dynamic => println!("{:#?}", config::get()?),
        },
        Commands::Default { action } => match action {
            DefaultCommands::Get => println!("{}", config::get()?.default_server),
            DefaultCommands::Set { server } => config::get()?.default_server = server,
        },
        Commands::Deploy { server } => {
            let server = handle_server_arg!(server);
            zellij::new(&server, Some(&deployment::get_command(&server)?))?;
        }
        Commands::Execute { server, commands } => {
            let server = handle_server_arg!(server);
            for command in commands {
                zellij::write_line(&server, command)?;
            }
        }
        Commands::List { active, inactive } => {
            let mut servers = vec![];
            server::for_each(|s| servers.push(s)).wrap_err("Failed to get servers")?;

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
        Commands::Mcrcon { server, commands } => mcrcon::run(&handle_server_arg!(server), commands)
            .wrap_err("Failed to run mcrcon command")?,
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
        Commands::Remove { servers, force } => if force {
            server::remove_servers(servers)
        } else {
            server::remove_servers_with_confirmation(servers)
        }
        .wrap_err("Failed to remove server")?,
        Commands::Stop { server } => {
            let server = handle_server_arg!(server);
            mcrcon::run(&server, vec!["stop"])
                .wrap_err_with(|| format!("Failed to stop server {}", &server))?;
        }
        Commands::Template { action } => match action {
            TemplateCommands::New { server } => template::new(&server)
                .wrap_err_with(|| format!("Failed to create template with server {server}"))?,
            TemplateCommands::From { template, server } => {
                template::from(&template, server.as_deref())
                    .wrap_err_with(|| format!("Failed to use template {template}"))?
            }
        },
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

    config::CONFIG.ensure_written()?;

    Ok(())
}
