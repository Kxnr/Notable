use std::{collections::HashMap, path::PathBuf};

use petgraph::graph::Graph;

use crate::notebook::{Link, Note};

type State = String;

struct Docket {
    // A docket is a multi-folder notebook where each folder represents a task state
    // and reference links represent dependencies of the current task
    root: PathBuf,
    tasks: HashMap<State, Taskbook>,
}

struct Task {
    note: Note,
    depends_on: Vec<Link>,
}

struct Taskbook {}

impl Docket {
    // fn dependency_graph(&self) -> Graph {
    //     todo!()
    //     add linked nodes, add unresolved links to stack
    // }

    // TODO: constructor

    // TODO: new task
}
