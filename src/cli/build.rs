use crate::context::{Message, Result};
use crate::filesystem;
use clap::Clap;
use std::path::PathBuf;

/// Builds the onelo store.
#[derive(Debug, Clap)]
pub struct Cmd {
    /// Input path
    #[clap(long, short = 'i', value_name = "path", default_value = "./test/files")]
    input_path: PathBuf,
    /// Cache path
    #[clap(long, short = 'c', value_name = "path", default_value = "./onelo.db")]
    cache_path: PathBuf,
}

impl Cmd {
    pub fn run(&self) -> Result<Message> {
        let paths = filesystem::get_files(&self.input_path)?;

        dbg!(&paths);
        let message = format!("{} files parsed.", paths.len());

        Ok(message)
    }
}
