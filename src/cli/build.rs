use clap::Clap;
use std::path::PathBuf;
use std::error::Error;
use crate::files;

type Message = String;
type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

/// Builds the onelo store.
#[derive(Debug, Clap)]
pub struct Cmd {
    /// Input path
    #[clap(long, short = "i", value_name = "path", default_value = "./test/files")]
    input_path: PathBuf,
    /// Cache path
    #[clap(long, short = "c", value_name = "path", default_value = "./onelo.db")]
    cache_path: PathBuf,
}

impl Cmd {
    pub fn run(&self) -> Result<Message> {
        let paths = files::get_markdown_files(&self.input_path)?;

        dbg!(&paths);
        let message = format!("{} files parsed.", paths.len());

        Ok(message)
    }
}