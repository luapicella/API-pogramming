use dot_dsl::graph::graph_items::edge::Edge;
use dot_dsl::graph::graph_items::node::Node;
use dot_dsl::graph::Graph;

fn main(){
    let attrsNodeA = vec![("color", "green"), ("label", "Green")];
    let attrsNodeB = vec![("color", "blue"), ("label", "Blue")];
    let attrsNodeC = vec![("color", "red"), ("label", "Red")];

    let nodes = vec![
        Node::new("a").with_attrs(&attrsNodeA),
        Node::new("c").with_attrs(&attrsNodeB),
        Node::new("b").with_attrs(&attrsNodeC),
    ];

    let edges = vec![
        Edge::new("b", "c"),
        Edge::new("a", "b").with_attrs(&[("color", "blue")]),
    ];

    let attrs = vec![("foo", "1"), ("title", "Testing Attrs"), ("bar", "true")];

    let graph = Graph::new()
    .with_nodes(&nodes)
    .with_edges(&edges)
    .with_attrs(&attrs);

    println!("{}",graph)
}