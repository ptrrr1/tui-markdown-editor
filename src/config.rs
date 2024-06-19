use std::path::PathBuf;
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
}