// tasks are notes with the extra dimensions of state and dependencies
use petgraph::graph::Graph;

type State = String;

struct Docket {
    root: PathBuf,
    tasks: HashMap<State, Notebook>,
}

impl Docket {
    fn dependency_graph(&self) -> Graph {
        todo!()
    }

    // TODO: new task
}
