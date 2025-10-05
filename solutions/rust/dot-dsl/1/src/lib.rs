
pub mod graph {
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Eq)]
    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[graph_items::node::Node]) -> Self {
            for node in nodes {
                self.nodes.push(node.clone());
            }
            self
        }

        pub fn with_edges(mut self, edges: &[graph_items::edge::Edge]) -> Self {
            for edge in edges {
                self.edges.push(edge.clone());
            }
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for (key, value) in attrs {
                self.attrs.insert(key.to_string(), value.to_string());
            }
            self
        }

        pub fn node(&self, name: &str) -> Option<&graph_items::node::Node> {
            self.nodes.iter().find(|n| n.name == name)
        }

        pub fn attr(&self, key: &str) -> Option<&String> {
            self.attrs.get(key)
        }
    }

    pub mod graph_items {

        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Eq, Clone)]
            pub struct Edge {
                pub from: String,
                pub to: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Self {
                        from: from.to_string(),
                        to: to.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (key, value) in attrs {
                        self.attrs.insert(key.to_string(), value.to_string());
                    }
                    self
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|s| s.as_str())
                }
                
            }
        }
        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Eq, Clone)]
            pub struct Node {
                pub name: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Self {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (key, value) in attrs {
                        self.attrs.insert(key.to_string(), value.to_string());
                    }
                    self
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|s| s.as_str())
                }
            }
        }
    }
}
