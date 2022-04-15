pub mod graph {
    use graph_items::node::Node;
    use graph_items::edge::Edge;
    use std::collections::HashMap;
    use std::fmt;

    pub mod graph_items{
        
        pub mod edge{
            use std::collections::HashMap;
            use std::fmt;


            #[derive(PartialEq, Eq, Clone, Debug)]
            pub struct Edge {
                start: String,
                end: String,
                attrs: HashMap<String, String>,

            }

            impl Edge {
                pub fn new(str1: &str, str2: &str) -> Self {
                    Self {
                        start: String::from(str1),
                        end: String::from(str2),
                        attrs: HashMap::new()
                    }
                }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    Self {
                        attrs: attrs
                            .iter()
                            .map(|(n, v)| (String::from(*n), String::from(*v)))
                            .collect(),
                        ..self
                    }
                }
            }

            impl fmt::Display for Edge {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    match self.attrs.is_empty() {
                        true => write!(f, "{} -- {}", self.start, self.end),
                        false => {
                            write!(f, "{} -- {} [", self.start, self.end);
                            self.attrs.iter().fold(Ok(()), |result, attr| {
                                result.and_then(|_| write!(f,"{}={},",attr.0,attr.1))
                            });
                            write!(f, "]");
                            Ok(())
                        }
                    }
                    
                }
            } 
        }

        pub mod node{
            use std::collections::HashMap;
            use std::fmt;

            #[derive(PartialEq, Eq, Clone, Debug)]
            pub struct Node{
                name : String,
                attrs: HashMap<String, String>,
            }

            impl Node{

                pub fn new(value: &str) -> Self {
                    Self {
                        name: String::from(value),
                        attrs: HashMap::new()
                    }
                }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    Self {
                        attrs: attrs
                            .iter()
                            .map(|(n, v)| (String::from(*n), String::from(*v)))
                            .collect(),
                        ..self
                    }
                }

                pub fn get_name(&self) -> &str {
                    &self.name
                }

                pub fn get_attr(&self, name: &str) -> Option<&str> {
                    self.attrs.get(name).map(|s| s.as_str())
                }

            }
            impl fmt::Display for Node {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    match self.attrs.is_empty() {
                        true => write!(f, "{}", self.name),
                        false => {
                            write!(f, "{} [", self.name);
                            self.attrs.iter().fold(Ok(()), |result, attr| {
                                result.and_then(|_| write!(f,"{}={},",attr.0,attr.1))
                            });
                            write!(f, "]");
                            Ok(())
                        }
                    }
                    
                }
            } 
            
        }
    }
    
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct Graph{
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {

        pub fn new() -> Self {
            Self { 
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new()
             }
        }

        pub fn with_nodes(self, nodes: &[Node]) -> Self {
            Self {
                nodes: Vec::from(nodes),
                ..self
            }
        }

        pub fn with_edges(self, edges: &[Edge]) -> Self {
            Self {
                edges : Vec::from(edges),
                ..self
            }
        }

        pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
            Self {
                attrs: attrs
                    .iter()
                    .map(|(n, v)| (String::from(*n), String::from(*v)))
                    .collect(),
                ..self
            }
        }

        pub fn get_node(self, node: &str) -> Option<Node> {
            self.nodes.iter().find(|n| n.get_name() == node).cloned()
        }
    }

    impl fmt::Display for Graph {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(f, "graph {{");
            self.nodes.iter().fold((Ok(())), |result, node| {
                result.and_then(|_| writeln!(f,"\t{}",node))
            });
            self.edges.iter().fold(Ok(()), |result, node| {
                result.and_then(|_| writeln!(f,"\t{}",node))
            });
            writeln!(f, "}}")
        }
    }
}
