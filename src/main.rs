use clap::{AppSettings, Clap};
use std::error::Error;

type Message = String;
type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

use onelo::files;

/// The build command handles the process of parsing all markdown files.
///
/// TODO: This module should be extracted to its own file.
mod build {
    use super::{files::get_markdown_files, Message, Result};
    use clap::Clap;
    use std::path::PathBuf;

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
            let paths = get_markdown_files(&self.input_path)?;

            dbg!(&paths);
            let message = format!("{} files parsed.", paths.len());

            Ok(message)
        }
    }
}

#[derive(Debug, Clap)]
enum Subcommand {
    Build(build::Cmd),
}

#[derive(Debug, Clap)]
#[clap(name = "onelo", version, global_setting(AppSettings::ColoredHelp))]
struct Cli {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

fn main() {
    let cli: Cli = Cli::parse();

    match cli.subcommand {
        Subcommand::Build(cmd) => match cmd.run() {
            Ok(msg) => {
                println!("{}", msg);
            }
            Err(err) => {
                eprintln!("{:?}", err);
            }
        },
    }
}

