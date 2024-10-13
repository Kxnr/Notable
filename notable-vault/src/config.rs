use serde::Deserialize;
use std::{collections::HashMap, fs::read_to_string, path::PathBuf};
use thiserror::Error;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub root_path: PathBuf,
    pub notebooks: HashMap<String, Notebook>,
    pub dockets: HashMap<String, Docket>,
}

#[derive(Deserialize, Clone)]
pub struct Notebook {
    pub location: PathBuf,
    pub template: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct Docket {
    // TODO: allow grouping of dockets into views
    pub location: PathBuf,
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("bad file")]
    BadFile,
    #[error("bad toml")]
    BadToml,
}

impl Config {
    /// ```toml
    /// [notable]
    /// root_path = "/home/kxnr/notable"
    ///
    /// [notebook.journal]
    /// location = "journal" # defaults to the name of the notebook
    /// template = "%m-%d-%&" # template for creating a new note
    ///
    /// [docket.work]
    /// location = "work" # defaults to the name of the docket
    /// ```
    pub fn from_config_file(filename: &str) -> Result<Self, ConfigError> {
        let file_contents = read_to_string(filename).map_err(|_| ConfigError::BadFile)?;
        toml::from_str(&file_contents).map_err(|_| ConfigError::BadToml)
    }
}
