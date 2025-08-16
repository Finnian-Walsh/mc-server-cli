
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "serverup", version, about = "Server binary manager")]
pub struct Cli {
    #[subcommand()]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "self")]
    SelfCmd {
        #[subcommand()]
        command: SelfCommands,
    },

    Update {
        #[arg(short, long)]
        local: bool,
    },
}

#[derive(Subcommand)]
pub enum SelfCommands {
    Update {
        #[arg(short, bool)]
        local: bool,
    },
}

