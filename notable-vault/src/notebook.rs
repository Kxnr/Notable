use std::{collections::HashMap, fs, path::PathBuf};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum NotebookError {
    #[error("bad notebook")]
    ErrorParsingNote,
    #[error("unsupported platform")]
    UnsupportedPlatform,
}

struct TextPoint {
    line: u32,
    column: u32,
}

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

pub struct Note {
    headings: Vec<Heading>,
    links: Vec<Link>,
}

pub struct Notebook {
    root: PathBuf,
    notes: HashMap<PathBuf, Note>,
}

impl Note {
    fn parse_file(path: &PathBuf) -> Result<Self, NotebookError> {
        // TODO: check if is dir
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
    // pub fn upsert(&mut self, path: PathBuf) {

    // }

    pub fn links(&self) -> impl Iterator<Item = &Link> {
        self.notes.values().map(|note| note.links.iter()).flatten()
    }

    // fn backlinks(&self) -> impl Iterator<Item = &Link> {
    //     todo!()
    // }

    pub fn link_targets(&self) -> impl Iterator<Item = &Heading> {
        self.notes
            .values()
            .map(|note| note.headings.iter())
            .flatten()
    }

    pub fn new(path: PathBuf) -> Self {
        let notes = ignore::Walk::new(&path)
            .filter_map(|entry| entry.ok())
            .map(|entry| {
                let path = entry.into_path();
                match Note::parse_file(&path) {
                    Ok(note) => Some((path, note)),
                    _ => None,
                }
            })
            .filter_map(|entry| entry);

        Notebook {
            root: path,
            notes: HashMap::from_iter(notes),
        }
    }

    pub fn refresh(self) -> Self {
        Self::new(self.root)
    }

    // TODO: change state

    // TODO: new note

    // TODO: update individual note from contents
}
