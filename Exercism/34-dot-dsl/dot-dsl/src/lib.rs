pub mod graph {
    pub mod graph_items {
        pub mod edge {
            pub struct Edge {
                val1: String,
                val2: String,
            }

            impl Edge {
                pub fn new(val1: &str, val2: &str) -> Self {
                    Edge {
                        val1: val1.to_string(),
                        val2: val2.to_string(),
                    }
                }
            }
        }
        pub mod node {
            #[derive(Clone, PartialEq, Debug)]
            pub struct Node {
                node: String,
            }

            impl Node {
                pub fn new(node: &str) -> Self {
                    Node {
                        node: node.to_string(),
                    }
                }
            }
        }
    }
    pub struct Graph {
        pub nodes: Vec<crate::graph::graph_items::node::Node>,
        pub edges: Vec<crate::graph::graph_items::edge::Edge>,
        pub attrs: Vec<(String, String)>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: Vec::new(),
            }
        }
    }

    impl Graph {
        pub fn with_nodes(mut self, nodes: &[crate::graph::graph_items::node::Node]) -> Self {
            self.nodes.extend_from_slice(nodes);
            self
        }
    }
}
