/// hold detailed information about the content of a vault including
/// headings, links, and frontmatter
pub struct VaultIndex {
    /// cache of document contents, which may be updated in-memory without persistence to disk
    document_contents: todo!(),
}

pub enum VaultItem {
    Note(String, VaultItem),
    Task(String, VaultItem),
}

pub struct NotebookIndex {
    link_sources: HashMap<PathBuf, Vec<VaultReference>>,
    link_targets: HashMap<PathBuf, Vec<VaultReference>>,
}

pub struct VaultReference {
    file: PathBuf,
    /// line and character
    place: [u32; 2],
}

pub struct DocketIndex {
    note_index: NotebookIndex,
    task_graph: HashMap<PathBuf, Vec<PathBuf>>,
}

impl VaultIndex {
    pub fn update_document() {
        // applies an edit to an in-memory document
        todo!();
    }

    pub fn load_document() {
        // load a document into the in-memory cache
        todo!();
    }

    pub fn remove_document() {
        // remove a document from the in-memory cache
        todo!();
    }

    pub fn links_from() {
        // get links in this document to another document
        todo!();
    }

    pub fn links_to() {
        // get backlinks for a given document
        todo!();
    }

    pub fn parents_of() {
        // get parent tasks for a given task
        todo!();
    }

    pub fn children_of() {
        // get child tasks for a given task
        todo!();
    }

    pub fn targets_in() {
        // get the possible link targets in a given document
    }
}
