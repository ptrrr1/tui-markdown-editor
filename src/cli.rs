use std::path::PathBuf;
use clap::{Args, Parser};

/// Create, Edit and View Markdown notes through your terminal.
#[derive(Debug, Parser)]
#[command(version, long_about = None)]
pub struct Cli {
    /// Specify the full path of the folder to write/read files from
    #[arg(long="config")]
    pub folder_path: Option<PathBuf>,

    /// Open the file
    #[command(flatten)]
    pub file: Option<File>,
}

#[derive(Debug, Args)]
pub struct File {
    /// Specify the subdirectory to look for or add the file to
    #[arg(short='d', long="dir")]
    pub dir: Option<String>,

    /// Opens or creates file with the specified name
    #[arg(short='o', long="open")]
    pub filename: Option<String>,   
}