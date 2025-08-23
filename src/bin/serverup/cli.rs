use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "serverup", version, about = "Server binary manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "self")]
    SelfCmd {
        #[command(subcommand)]
        command: SelfCommands,
    },

    #[command()]
    Update {
        #[arg(short, long)]
        local: bool,
    },
}

#[derive(Subcommand)]
pub enum SelfCommands {
    #[command()]
    Update {
        #[arg(short, long)]
        local: bool,
    },
}
