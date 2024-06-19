use std::path::PathBuf;
use confy::ConfyError;
use serde::{Serialize, Deserialize};

// Options stored in config file
// It's possible to create multiple config files
// So, if i want to have some configuration
// I can always organize it per file
#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub folder_path: Option<PathBuf>,
}

// Default configuration used when
// creating the config file through confy::load
impl Default for Settings {
    fn default() -> Self {
        Settings { folder_path: None }
    }
}

impl Settings {
    pub fn new(path: PathBuf) -> Settings {
        Settings { folder_path: Some(path) }
    }

    pub fn store(self, appname: &str, config_file_name: Option<&str>) {
        // (app name, file to store, configuration)
        let cfg = confy::store(appname, config_file_name, self);

        // Match result
        match cfg {
            Ok(()) => { println!("Config file successfully updated!") },

            Err(e) => { io_err(e) },
        }
    }

    pub fn path_exists(appname: &str, config_file_name: Option<&str>) -> Option<PathBuf> {
        // I don't know if i should use the ? operator, but i don't know a better way
        let cfg: Settings = confy::load(appname, config_file_name).ok()?;

        if let None = cfg.folder_path {
            missing_folder_err();

            return None
        }
        
        return Some(cfg.folder_path?)
    }
}

// TODO: There should be a better way to do this
// I don't want to use panic! because i want consistency (color the messages)
// But as of now i don't know a better way
// Creating a cmd everytime seems like too much
fn io_err(e: ConfyError) {
    let cmd = clap::Command::new("config");

    let err = clap::Error::raw(
        clap::error::ErrorKind::Io, 
        e
        ).with_cmd(&cmd);

    let _ = err.print();
}

pub fn missing_folder_err() {
    let cmd = clap::Command::new("config");

    let err = clap::Error::raw(
        clap::error::ErrorKind::MissingRequiredArgument, 
        "Notes folder not configured. Use 'appname --config <FOLDER_PATH>'."
        ).with_cmd(&cmd);

    let _ = err.print();
}