#[derive(Clone, Debug)]
struct Node {
    value: u8,
    children: Vec<Node>,
}

enum Prefix {
    Root,
    Blank,
    Continuation,
    LastChild,
    OtherChild,
    HasChildren,
    NoChildren,
}

enum NodePosition {
    Root,
    LastChild,
    Child,
}

impl Prefix {
    fn as_str(&self) -> &'static str {
        match self {
            Prefix::Root => "┌",
            Prefix::Blank => " ",
            Prefix::Continuation => "|",
            Prefix::LastChild => "└",
            Prefix::OtherChild => "├",
            Prefix::HasChildren => "┬",
            Prefix::NoChildren => "─",
        }
    }
}

fn main() {
    let tree = Node {
        value: 0,
        children: [
            Node {
                value: 1,
                children: [
                    Node {
                        value: 2,
                        children: [].to_vec(),
                    },
                    Node {
                        value: 2,
                        children: [].to_vec(),
                    },
                ]
                .to_vec(),
            },
            Node {
                value: 1,
                children: [
                    Node {
                        value: 2,
                        children: [].to_vec(),
                    },
                    Node {
                        value: 2,
                        children: [
                            Node {
                                value: 3,
                                children: [].to_vec(),
                            },
                            Node {
                                value: 3,
                                children: [].to_vec(),
                            },
                            Node {
                                value: 3,
                                children: [].to_vec(),
                            },
                        ]
                        .to_vec(),
                    },
                    Node {
                        value: 2,
                        children: [].to_vec(),
                    },
                ]
                .to_vec(),
            },
        ]
        .to_vec(),
    };

    fn print_dfs(tree: &Node, prefix_stack: &mut Vec<Prefix>, node_position: NodePosition) {
        for chr in prefix_stack.iter() {
            print!("{}", chr.as_str())
        }

        // continuation character
        let has_children = tree.children.len() > 0;
        if has_children && matches!(node_position, NodePosition::Root) {
            print!("{}", Prefix::Root.as_str());
        } else if has_children {
            print!("{}", Prefix::HasChildren.as_str());
        } else {
            print! {"{}", Prefix::NoChildren.as_str()};
        }

        print!("{}", tree.value);
        println!();

        let mut iter = tree.children.iter().peekable();

        // replaces the separator used for the current node with the appropriate continuation
        // character for children of this node. If there are no children, the outer function will
        // pop off the separator.
        _ = prefix_stack.pop();
        match node_position {
            NodePosition::Child => {
                prefix_stack.push(Prefix::Continuation);
            }
            NodePosition::LastChild => {
                prefix_stack.push(Prefix::Blank);
            }
            // TODO: push spacing equal to prefix on ROOT
            _ => {}
        }

        while let Some(child) = iter.next() {
            let last = iter.peek().is_none();

            if last {
                prefix_stack.push(Prefix::LastChild);
            } else {
                prefix_stack.push(Prefix::OtherChild);
            }

            print_dfs(
                &child,
                prefix_stack,
                if last {
                    NodePosition::LastChild
                } else {
                    NodePosition::Child
                },
            );
            _ = prefix_stack.pop();
        }

        // group spacing
        if matches!(node_position, NodePosition::LastChild) {
            for chr in prefix_stack.iter() {
                print!("{}", chr.as_str())
            }
            println!();
        }
    }

    let mut prefix = vec![];
    print_dfs(&tree, &mut prefix, NodePosition::Root);
}
