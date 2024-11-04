use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;

use crate::config::Config;
use crate::notebook::Notebook;

#[derive(Error, Debug)]
pub enum VaultError {
    #[error("bad notebook")]
    BadNotebook,
    #[error("bad docket")]
    BadDocket,
    #[error("Could not list files")]
    ListFilesError,
}

type Name = String;

pub struct Vault {
    root: PathBuf,
    notebooks: HashMap<Name, Notebook>,
}

impl Vault {
    pub fn new(config: Config) -> Self {
        Self {
            root: config.root_path,
            notebooks: config
                .notebooks
                .iter()
                .map(|(k, v)| (k.to_owned(), Notebook::new(v)))
                .collect(),
        }
    }
}
