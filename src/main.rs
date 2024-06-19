pub mod cli;
use cli::{Cli, Commands};

pub mod config;
use config::Settings;

use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let appname = "md-tui";
    let config_file_name = "config";

    match args.command {
        Commands::Config { path } => {
            // Stores/Updates path in config file
            Settings::new(path).store(appname, Some(config_file_name))
        },

        Commands::Open { file, dir } => {
            // Check if folder has been configured and returns path
            if let Some(mut filepath) = Settings::path_exists(appname, Some(config_file_name)) {

                // Append dir to path if the arg exists
                if let Some(folder) = dir {
                    filepath.push(folder);
                }

                // Append file to path
                filepath.push(file);

            }
        },
    }

    Ok(())
}