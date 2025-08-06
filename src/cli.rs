use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "server", version, about = "Server CLI tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(visible_alias = "att", aliases = &["connect", "con"])]
    Attach {
        #[arg()]
        session: Option<String>,
    },

    #[command(visible_alias = "bu")]
    Backup {
        #[arg()]
        server: Option<String>,
    },

    #[command(visible_alias = "def")]
    Default {
        #[command(subcommand)]
        action: DefaultCommands,
    },

    #[command(visible_alias = "dpl", visible_aliases = &["start", "st"])]
    Deploy {
        #[arg()]
        server: Option<String>,
    },

    #[command(visible_alias = "stp")]
    Stop {
        #[arg()]
        server: Option<String>,
    },

    #[command(visible_alias = "ls")]
    List {
        #[arg(short, long)]
        active: bool,

        #[arg(short, long)]
        inactive: bool,
    },
}

#[derive(Subcommand)]
pub enum DefaultCommands {
    #[command()]
    Get,

    #[command()]
    Set {
        #[arg()]
        server: String,
    },
}

