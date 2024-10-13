use std::{
    collections::{HashMap, HashSet},
    fs::read_dir,
    path::PathBuf,
};
use thiserror::Error;

use crate::config::Config;
use crate::notebook::Notebook;
// use crate::tasks::Docket;

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
    config: Config,
    notebooks: HashMap<Name, Notebook>,
    // dockets: HashMap<Name, Docket>,
}

impl Vault {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.to_owned(),
            notebooks: config
                .notebooks
                .iter()
                .map(|(k, v)| (k.to_owned(), Notebook::new(v.location.to_owned())))
                .collect(),
        }
    }
}
