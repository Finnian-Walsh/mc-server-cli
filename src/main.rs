mod cli;
mod settings;

use cli::*;
use clap::Parser;
use std::process;
use settings::{get_setting, SettingsError};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Backup { server } => {
            let server = server.unwrap_or_else(|| {
                match get_setting("default") {
                    Ok(value) => {
                        if value.len() == 0 {
                            println!(".default file is empty!");
                            process::exit(1);
                        }

                        value
                    },
                    Err(SettingsError::Io(e)) => {
                        eprintln!("IO error: {}", e);
                        process::exit(1);
                    },
                    Err(SettingsError::HomeDirFailure) => {
                        eprintln!("Failed to get home directory");
                        process::exit(1);
                    },
                }
            });

            println!("Ok");
        },

        _ => { println!("ok"); },
    }
}
