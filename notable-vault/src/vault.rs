use chrono::{DateTime, Local};
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;
use toml::value::Datetime;

use crate::config::Config;
use crate::notebook::{NoteTemplate, Notebook};

#[derive(Error, Debug)]
pub enum VaultError {
    #[error("bad notebook")]
    BadNotebook,
}

type Name = String;

pub struct Template {
    date_format: String,
    note_format: String,
}
pub struct TemplateArgs {
    pub name: String,
    pub when: DateTime<Local>,
}

impl NoteTemplate for Template {
    type Args = TemplateArgs;

    fn format_note_name(&self, args: &Self::Args) -> String {
        // TODO: make args owned
        let formatted_date = args.when.format(&self.date_format);
        // FIXME: check if template is valid
        let template = leon::Template::parse(&self.note_format).unwrap();
        template
            .render(&[("name", &args.name), ("date", &formatted_date.to_string())])
            .unwrap()
    }
}

pub struct Vault {
    // TODO: index files
    // TODO: handle path relativity; add get file method for notebook
    // TODO: document and define notebook nesting; should be disallowed?
    pub root: PathBuf,
    pub notebooks: HashMap<Name, Notebook<Template>>,
}

impl Vault {
    pub fn new(config: Config) -> Self {
        Self {
            root: config.root_path,
            notebooks: config
                .notebooks
                .into_iter()
                .map(|(k, v)| {
                    (
                        k.to_owned(),
                        Notebook::new(
                            &v.location,
                            Template {
                                date_format: v.date_format.unwrap_or("%Y-%m-%d".to_string()),
                                note_format: v.note_template.unwrap_or("{ name }".to_string()),
                            },
                        ),
                    )
                })
                .collect(),
        }
    }

    pub fn get_path(
        &self,
        notebook_name: String,
        args: <Template as NoteTemplate>::Args,
    ) -> Result<PathBuf, VaultError> {
        let mut new_path = self.root.to_owned();
        let notebook = self
            .notebooks
            .get(&notebook_name)
            .ok_or(VaultError::BadNotebook)?;
        new_path.push(notebook_name);
        new_path.push(notebook.get_name(&args));
        new_path.set_extension("md");
        Ok(new_path)
    }
}
