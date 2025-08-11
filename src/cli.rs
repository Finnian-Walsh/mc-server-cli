use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "server", version, about = "Server CLI tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(visible_alias = "att", aliases = &["connect", "con"], about = "Attach to a server session")]
    Attach {
        #[arg()]
        session: Option<String>,
    },

    #[command(visible_alias = "bu", about = "Backup a server")]
    Backup {
        #[arg()]
        server: Option<String>,
    },

    #[command(visible_alias = "def", about = "Set or get the default server")]
    Default {
        #[command(subcommand)]
        action: DefaultCommands,
    },

    #[command(visible_alias = "dpl", visible_aliases = &["start", "st"], about = "Deploy a server")]
    Deploy {
        #[arg()]
        server: Option<String>,
    },

    #[command(visible_alias = "exec", about = "Execute a command on a server")]
    Execute {
        #[arg()]
        command: String,

        #[arg()]
        server: Option<String>,
    },

    #[command(visible_alias = "stp", about = "Stop a server")]
    Stop {
        #[arg()]
        server: Option<String>,
    },

    #[command(visible_alias = "ls", about = "List all, active or inactive servers")]
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
