use chrono::{Local, TimeZone};
use std::env::home_dir;
use std::path::PathBuf;

use clap::{arg, Parser, Subcommand};
use notable_vault::{
    config::Config,
    vault::{TemplateArgs, Vault},
};

fn get_default_config_path() -> PathBuf {
    // This is deprecated, but I don't really care about supporting cygwin and would rather not
    // pull in a crate for this
    let mut dir = home_dir().unwrap();
    dir.push(".config");
    dir.push("notable");
    dir.push("config.toml");
    dir
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value=get_default_config_path().into_os_string())]
    config: PathBuf,

    // make command optional; without it, open to index file
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    // command ideas:
    //    List
    //      Graph
    //      Outline
    //    Move
    //    Delete
    //    Search
    Edit {
        notebook: String,

        #[arg(short, long)]
        name: Option<String>,

        #[arg(short, long)]
        date: Option<String>,
    },
}

fn main() {
    let args = Args::parse();
    let config = Config::from_config_file(&args.config).unwrap();
    let vault = Vault::new(config);

    match args.command {
        Command::Edit {
            notebook,
            name,
            date,
        } => match vault.get_path(
            notebook,
            TemplateArgs {
                name: name.unwrap_or("".to_string()),
                when: date.map_or_else(
                    || Local::now(),
                    |date_str| {
                        fuzzydate::parse(date_str)
                            .unwrap()
                            .and_local_timezone(Local)
                            .single()
                            .unwrap()
                    },
                ),
            },
        ) {
            // TODO: report errors
            Ok(note_path) => _ = edit::edit_file(note_path),
            _ => {}
        },
    }
}
