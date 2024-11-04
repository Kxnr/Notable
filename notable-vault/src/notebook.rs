use std::{collections::HashMap, fs, path::PathBuf};

use chrono::{DateTime, Local};
use thiserror::Error;

use crate::config::Notebook as NotebookConfig;

#[derive(Error, Debug)]
pub enum NotebookError {
    #[error("bad notebook")]
    ErrorParsingNote,
    #[error("unsupported platform")]
    UnsupportedPlatform,
}

#[derive(Clone)]
struct TextPoint {
    line: u32,
    column: u32,
}

#[derive(Clone)]
struct TextRange {
    start: TextPoint,
    end: TextPoint,
}

impl From<markdown::unist::Position> for TextRange {
    fn from(item: markdown::unist::Position) -> Self {
        TextRange {
            start: TextPoint {
                line: item
                    .start
                    .line
                    .try_into()
                    .expect("How big are your files!?"),
                column: item
                    .start
                    .column
                    .try_into()
                    .expect("How long are your lines!?"),
            },
            end: TextPoint {
                line: item.end.line.try_into().expect("How big are your files!?"),
                column: item
                    .end
                    .column
                    .try_into()
                    .expect("How long are your lines!?"),
            },
        }
    }
}

pub struct Heading {
    text: String,
    range: TextRange,
}

pub struct Link {
    text: String,
    target: String,
    range: TextRange,
}

pub struct BackLink {
    text: String,
    source: String,
    range: TextRange,
}

impl From<(&PathBuf, &Link)> for BackLink {
    fn from(item: (&PathBuf, &Link)) -> Self {
        BackLink {
            text: item.1.text.to_owned(),
            // TODO: another one of those pesky normalization cases, could store a ref to a
            // TODO: static map of names to keys instead of using names as keys
            source: item.0.to_string_lossy().to_string(),
            range: item.1.range.to_owned(),
        }
    }
}

pub struct Note {
    headings: Vec<Heading>,
    links: Vec<Link>,
}

pub struct Notebook {
    config: NotebookConfig,
    notes: HashMap<PathBuf, Note>,
    backlinks: HashMap<String, Vec<BackLink>>,
}

impl Note {
    // TODO: create new from file contents contents

    pub fn parse_file(path: &PathBuf) -> Result<Self, NotebookError> {
        // TODO: check if is dir
        // TODO: handle ref links
        let ast = markdown::to_mdast(
            &fs::read_to_string(path).map_err(|_| NotebookError::ErrorParsingNote)?,
            &markdown::ParseOptions::default(),
        )
        .map_err(|_| NotebookError::ErrorParsingNote)?;

        let mut node_queue: Vec<markdown::mdast::Node> = Vec::new();
        let mut maybe_node = Some(ast);
        let mut headings = Vec::new();
        let mut links = Vec::new();

        while let Some(mut node) = maybe_node {
            match node {
                // seems like I either have to clone the node or the fields to both get the node
                // as a str and unpack the tuple variant
                // TODO: check if is local link
                // TODO: check if link link is within notebook
                markdown::mdast::Node::Link(ref link) => {
                    links.push(Link {
                        text: node.to_string(),
                        target: link.url.to_owned(),
                        // FIXME: handle missing position info
                        range: link
                            .position
                            .clone()
                            .expect("Missing position info.")
                            .into(),
                    });
                }
                markdown::mdast::Node::Heading(ref heading) => {
                    // A heading isn't a block, so the children are the header text, not the content
                    // after the heading
                    headings.push(Heading {
                        text: node.to_string(),
                        // FIXME: handle missing position info
                        range: heading
                            .position
                            .clone()
                            .expect("Missing position info")
                            .into(),
                    });
                }
                _ => {
                    if let Some(children) = node.children_mut() {
                        node_queue.append(children);
                    }
                }
            }
            maybe_node = node_queue.pop();
        }

        Ok(Note { headings, links })
    }
}

impl Notebook {
    // TODO: normalize paths used as keys

    pub fn links(&self) -> impl Iterator<Item = &Link> {
        self.notes.values().map(|note| note.links.iter()).flatten()
    }

    pub fn link_targets(&self) -> impl Iterator<Item = &Heading> {
        self.notes
            .values()
            .map(|note| note.headings.iter())
            .flatten()
    }

    pub fn new(notebook_config: &NotebookConfig) -> Self {
        // the location must be a directory
        let notes = ignore::Walk::new(&notebook_config.location)
            .filter_map(|entry| entry.ok())
            .map(|entry| {
                let path = entry.into_path();
                match Note::parse_file(&path) {
                    Ok(note) => Some((path, note)),
                    _ => None,
                }
            })
            .filter_map(|entry| entry);

        let mut notebook = Notebook {
            config: notebook_config.to_owned(),
            notes: HashMap::from_iter(notes),
            backlinks: HashMap::new(),
        };
        notebook.update_backlinks();
        notebook
    }

    pub fn refresh(self) -> Self {
        Self::new(&self.config)
    }

    pub fn upsert(&mut self, path: &PathBuf) {
        let note = Note::parse_file(path).unwrap_or(Note {
            headings: vec![],
            links: vec![],
        });

        let _ = self.notes.insert(path.to_owned(), note);
        self.update_backlinks();
    }

    pub fn remove(&mut self, path: &PathBuf) {
        self.notes.remove(path);
        self.update_backlinks();
    }

    pub fn note_for(&mut self, when: Option<DateTime<Local>>) -> PathBuf {
        let when = when.unwrap_or_else(Local::now);
        let mut path = self.config.location.clone();
        if let Some(template) = &self.config.template {
            path.push(when.format(template).to_string());
        } else {
            path.push(when.format("%Y-%m-%d").to_string());
        }
        path.set_extension("md");
        path
    }

    fn update_backlinks(&mut self) {
        // TODO: less than efficient to update all backlinks all the time
        self.backlinks = self
            .notes
            .iter()
            .flat_map(|(key, note)| {
                note.links
                    .iter()
                    .map(move |link| (link.target.to_owned(), BackLink::from((key, link))))
            })
            .fold(HashMap::new(), |mut acc, (key, backlink)| {
                acc.entry(key).or_default().push(backlink);
                acc
            })
    }
}
