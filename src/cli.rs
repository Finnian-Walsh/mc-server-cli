use crate::platforms;
use clap::{ArgGroup, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "server", version, about = "Server CLI tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(visible_alias = "a", about = "Attach to a server session")]
    Attach { server: Option<String> },

    #[command(visible_alias = "conf", about = "Query the configuration")]
    Config {
        #[command(subcommand)]
        config_type: ConfigType,
    },

    #[command(visible_alias = "def", about = "Set or get the default server")]
    Default {
        #[command(subcommand)]
        action: DefaultCommands,
    },

    #[command(visible_alias = "dpl", about = "Deploy a server")]
    Deploy { server: Option<String> },

    #[command(visible_alias = "exec", about = "Execute a command on a server")]
    Execute {
        #[arg(short, long)]
        server: Option<String>,

        #[arg(trailing_var_arg = true)]
        commands: Vec<String>,
    },

    #[command(visible_alias = "ls", about = "List all, active or inactive servers")]
    List {
        #[arg(short, long, conflicts_with = "inactive")]
        active: bool,

        #[arg(short, long, conflicts_with = "active")]
        inactive: bool,
    },

    #[command(about = "Interact with a server, using the minecraft remote console")]
    Mcrcon {
        server: Option<String>,

        commands: Vec<String>,
    },

    #[command(about = "Create a new server")]
    New {
        #[clap(value_enum)]
        platform: platforms::Platform,

        #[arg(short, long)]
        name: Option<String>,

        #[arg(short, long)]
        version: Option<String>,
    },

    #[command(visible_alias = "rm", about = "Remove a server")]
    Remove { server: String },

    #[command(about = "Stop a server")]
    Stop { server: Option<String> },

    #[command(visible_alias = "up", about = "Update the server binary",
        group(
                ArgGroup::new("source")
                    .required(true)
                    .args(&["git", "path"])
            )
    )]
    Update {
        #[arg(short, long)]
        git: bool,

        #[arg(short, long)]
        commit: Option<String>,

        #[arg(short, long)]
        path: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
pub enum ConfigType {
    Static,

    Dynamic,
}

#[derive(Subcommand)]
pub enum DefaultCommands {
    Get,

    Set { server: String },
}
