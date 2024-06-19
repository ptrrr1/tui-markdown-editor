use std::path::PathBuf;
use clap::{Parser, Subcommand};

/// Create, Edit and View Markdown notes through your terminal.
#[derive(Debug, Parser)]
#[command(version, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Opens file with the specified name, else creates with <filename> in <directory>
    #[command(short_flag='o')]
    Open { file: String, dir: Option<String> },

    /// Configure the path of the folder to write/read files from
    #[command()]
    Config { path: PathBuf }
}