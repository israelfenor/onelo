use clap::{AppSettings, Clap};
use onelo_backend::cli::build;

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
