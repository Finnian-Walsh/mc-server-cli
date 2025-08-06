mod cli;
mod settings;

use cli::*;
use clap::Parser;
use std::process;
use settings::*;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Attach { session } => {
            let session = session.unwrap_or_else(|| {
                "a".to_string()
            });
        },
        Commands::Backup { server } => {
            let server = server.unwrap_or_else(|| {
                match get_default() {
                    Ok(default) => default,
                    Err(e) => {
                        println!("{}", e);
                        process::exit(1);
                    },
                }
            });

            println!("{}", server);
        },
        _ => { println!("ok"); },
    }
}
