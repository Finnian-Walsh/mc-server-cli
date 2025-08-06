mod cli;
mod settings;
mod tmux;

use cli::*;
use clap::Parser;
use settings::*;
use std::process;
use tmux::AttachError;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Attach { session } => {
            let session = session.unwrap_or_else(|| get_default_or_exit!());

            let Err(e) = tmux::attach(&session) else {
                return;
            };

            match e {
                AttachError::Io(e) => eprintln!("IO error: {}", e),
                AttachError::SessionInexistent => eprintln!("Session {} does not exist", session),
                AttachError::TmuxFailure(e) => eprintln!("Tmux failure: {}", e),
            };

            process::exit(1);
        },
        Commands::Backup { server } => {
            let server = server.unwrap_or_else(|| get_default_or_exit!());

            println!("Backing up {}", server);

            
        },
        _ => { println!("ok"); },
    }
}
