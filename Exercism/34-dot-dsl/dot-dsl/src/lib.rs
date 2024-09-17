pub mod graph {
    use std::collections::HashMap;

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Edge {
                val1: String,
                val2: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(val1: &str, val2: &str) -> Self {
                    Edge {
                        val1: val1.to_string(),
                        val2: val2.to_string(),
                        attrs: HashMap::new(),
                    }
                }
            }

            impl Edge {
                pub fn with_attrs(mut self, attr: &[(&str, &str)]) -> Self {
                    self.attrs
                        .extend(attr.iter().map(|&(k, v)| (k.to_string(), v.to_string())));
                    self
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|v| v.as_str())
                }
            }
        }
        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, PartialEq, Debug)]
            pub struct Node {
                pub node: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(node: &str) -> Self {
                    Node {
                        node: node.to_string(),
                        attrs: HashMap::new(),
                    }
                }
            }

            impl Node {
                pub fn with_attrs(mut self, attr: &[(&str, &str)]) -> Self {
                    self.attrs
                        .extend(attr.iter().map(|&(a, b)| (a.to_string(), b.to_string())));
                    self
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|v| v.as_str())
                }
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct Graph {
        pub nodes: Vec<crate::graph::graph_items::node::Node>,
        pub edges: Vec<crate::graph::graph_items::edge::Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }
    }

    impl Graph {
        pub fn with_nodes(mut self, nodes: &[crate::graph::graph_items::node::Node]) -> Self {
            self.nodes.extend_from_slice(nodes);
            self
        }

        pub fn with_edges(mut self, edges: &[crate::graph::graph_items::edge::Edge]) -> Self {
            self.edges.extend_from_slice(edges);
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            // for &(key, value) in attrs {
            //     self.attrs.insert(key.to_string(), value.to_string());
            // }
            self.attrs.extend(
                // extend::> Extends a collection with the contents of an iterator.
                attrs
                    .iter()
                    .map(|&(key, value)| (key.to_string(), value.to_string())),
            );
            self
        }

        pub fn node(&self, key: &str) -> Option<crate::graph::graph_items::node::Node> {
            self.nodes.iter().find(|n| n.node == key).cloned()
        }
    }
}
