pub mod graph {
    use std::collections::HashMap;

    pub mod graph_items {
        pub mod edge {
            #[derive(Debug, Clone, PartialEq)]
            pub struct Edge<'attr> {
                val1: String,
                val2: String,
                attrs: Vec<(&'attr str, &'attr str)>,
            }

            impl<'attr> Edge<'attr> {
                pub fn new(val1: &str, val2: &str) -> Self {
                    Edge {
                        val1: val1.to_string(),
                        val2: val2.to_string(),
                        attrs: Vec::new(),
                    }
                }
            }

            impl<'attr> Edge<'attr> {
                pub fn with_attrs(mut self, attr: &'attr [(&str, &str)]) -> Self {
                    self.attrs.extend_from_slice(attr);
                    self
                }
            }
        }
        pub mod node {
            #[derive(Clone, PartialEq, Debug)]
            pub struct Node<'attrs> {
                node: String,
                attrs: Vec<(&'attrs str, &'attrs str)>,
            }

            impl<'attrs> Node<'attrs> {
                pub fn new(node: &str) -> Self {
                    Node {
                        node: node.to_string(),
                        attrs: Vec::new(),
                    }
                }
            }

            impl<'attrs> Node<'attrs> {
                pub fn with_attrs(mut self, attr: &'attrs [(&str, &str)]) -> Self {
                    self.attrs.extend_from_slice(attr);
                    self
                }
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct Graph<'node, 'edge> {
        pub nodes: Vec<crate::graph::graph_items::node::Node<'node>>,
        pub edges: Vec<crate::graph::graph_items::edge::Edge<'edge>>,
        pub attrs: HashMap<String, String>,
    }

    impl<'node, 'edge> Graph<'node, 'edge> {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }
    }

    impl<'node, 'edge> Graph<'node, 'edge> {
        pub fn with_nodes(mut self, nodes: &'node [crate::graph::graph_items::node::Node]) -> Self {
            self.nodes.extend_from_slice(nodes);
            self
        }

        pub fn with_edges(mut self, edges: &'edge [crate::graph::graph_items::edge::Edge]) -> Self {
            self.edges.extend_from_slice(edges);
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            // attrs
            //     .iter()
            //     .map(|&(a, b)| self.attrs.insert(&a, &b))
            //     .collect::<>();
            for &(key, value) in attrs {
                self.attrs.insert(key.to_string(), value.to_string());
            }
            self
        }
    }
}
