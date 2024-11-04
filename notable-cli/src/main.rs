use std::path::PathBuf;

use clap::{arg, Parser, Subcommand};
use notable_vault::{config::Config, vault::Vault};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config: PathBuf,

    // make command optional; without it, open to index file
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Edit {
        #[arg(short, long)]
        notebook: String,

        #[arg(short, long)]
        command: String,
    },
}

fn main() {
    let args = Args::parse();
    let config = Config::from_config_file(&args.config).unwrap();
    let vault = Vault::new(config);

    match args.command {
        Command::Edit { notebook, command } => {
            _ = edit::edit_file("abc.txt");
        }
    };
}
