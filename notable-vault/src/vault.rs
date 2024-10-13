use std::{
    collections::{HashMap, HashSet},
    fs::read_dir,
    path::PathBuf,
};
use thiserror::Error;

use crate::config::Config;
use crate::notebook::Notebook;
use crate::tasks::Docket;

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
    dockets: HashMap<Name, Docket>,
}

impl Vault {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.to_owned(),
            notebooks: config
                .notebooks
                .iter()
                .map(|(k, v)| (k.to_owned(), load_notebook(&v.location)))
                .filter(|(_, v)| v.is_ok())
                .map(|(k, v)| (k, v.expect("Err filtered out")))
                .collect(),
            dockets: config
                .dockets
                .iter()
                .map(|(k, v)| (k.to_owned(), load_docket(&v.location)))
                .filter(|(_, v)| v.is_ok())
                .map(|(k, v)| (k, v.expect("Err filtered out")))
                .collect(),
        }
    }
}

fn list_files(dir: &PathBuf) -> Result<HashSet<PathBuf>, VaultError> {
    Ok(read_dir(dir)
        .map_err(|_| VaultError::ListFilesError)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|entry| !entry.is_dir())
        .collect())
}

fn load_notebook(path: &PathBuf) -> Result<Notebook, VaultError> {
    list_files(path).map_err(|_| VaultError::BadNotebook)
}

fn load_docket(path: &PathBuf) -> Result<Docket, VaultError> {
    let mut open_path = path.to_owned();
    open_path.push("open");
    let mut closed_path = path.to_owned();
    closed_path.push("closed");

    Ok(Docket {
        open: list_files(&open_path).map_err(|_| VaultError::BadDocket)?,
        closed: list_files(&closed_path).map_err(|_| VaultError::BadDocket)?,
    })
}
