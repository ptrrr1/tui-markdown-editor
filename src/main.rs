// 
// TODO: ADD CLAP
// Clap is going to be used to pass in the file to look for or create
// 
// Commands:
// -d <dir> --new, -n <name>  creates a file in the specified dir
// -d <dir> --open, -o <name> opens a file / creates if it doesnt exist
// --dir, -d <dir>            creates a dir
//
pub mod cli;
use cli::Cli;

pub mod config;
use config::Settings;

use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let appname = "md-tui";
    let config_file_name = "config";

    // Store path if there is one
    if let Some(path) = args.folder_path {
        let settings = Settings::new(path);
        // (app name, file to store, configuration)
        confy::store(appname, config_file_name, settings)?;
    }

    // Read path
    // (app name, file to read)
    let cfg: Settings = confy::load(appname, config_file_name)?;

    // Check if folder has been configured
    match cfg.folder_path {
        Some(_path) => {
            // If Some(args.dir) then open it

            // Open file to edit
        },

        None => { missing_folder_err() },
    }

    Ok(())
}

fn missing_folder_err() {
    let cmd = clap::Command::new("config");

    let err = clap::Error::raw(
        clap::error::ErrorKind::MissingRequiredArgument, 
        "Notes folder not configured. Use 'appname --config <FOLDER_PATH>'."
        ).with_cmd(&cmd);

    let _ = err.print();

    /*
    let mut err = clap::Error::new(
        clap::error::ErrorKind::MissingRequiredArgument
        ).with_cmd(&cmd);

    err.insert(
        clap::error::ContextKind::SuggestedArg, 
        clap::error::ContextValue::String("--config <FOLDER_PATH>".to_owned())
    );

    err.insert(
        clap::error::ContextKind::Custom, 
        clap::error::ContextValue::String("TEST".to_owned())
    );
    */        

    //panic!("Notes folder not configured. Use '{} --config <FOLDER_PATH>'.", appname);
}