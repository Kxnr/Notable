use serde::Deserialize;
use std::{collections::HashMap, fs::read_to_string, path::PathBuf};
use thiserror::Error;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub root_path: PathBuf,
    pub notebooks: HashMap<String, Notebook>,
}

#[derive(Deserialize, Clone)]
pub struct Notebook {
    // TODO: require this location to be relative
    pub location: PathBuf,
    pub index_file: PathBuf,
    pub template: Option<String>,
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
    /// root_path = "/home/kxnr/notable"
    ///
    /// [notebook.journal]
    /// location = "journal" # defaults to the name of the notebook
    /// template = "%m-%d-%Y" # template for creating a new note
    /// index_file = "index.md"
    /// ```
    pub fn from_config_file(filename: &PathBuf) -> Result<Self, ConfigError> {
        let file_contents = read_to_string(filename).map_err(|_| ConfigError::BadFile)?;
        Self::from_string(&file_contents)
    }

    pub fn from_string(config: &str) -> Result<Self, ConfigError> {
        toml::from_str(&config).map_err(|_| ConfigError::BadToml)
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn example_config() {
        let config_data = "\
             root_path = \"/home/kxnr/notable\"
    
             [notebooks]
             [notebooks.journal]
             location = \"journal\" # defaults to the name of the notebook
             template = \"%m-%d-%Y\" # template for creating a new note
             index_file = \"index.md\"";

        assert!(
            Config::from_string(config_data).is_ok(),
            "failed to parse config"
        );
    }
}
