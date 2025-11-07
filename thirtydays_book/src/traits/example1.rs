use std::fmt::Debug;

trait Graph {
    type Node;
    fn add_node(&mut self, node: Self::Node);
    fn print_nodes(&self);
}

struct MyGraph {
    nodes: Vec<String>,
}

impl Graph for MyGraph  {
    type Node = String;
    fn add_node(&mut self, node: Self::Node) {
        self.nodes.push(node);
    }

    fn print_nodes(&self) {
        println!("Nodes: {:?}", self.nodes);
    }
}

fn print_graph_nodes<G>(graph: &G)
    where 
    G: Graph,
    G::Node: Debug, {
        graph.print_nodes();
    }

fn main() {
    let mut graph = MyGraph { nodes: Vec::new() };
    graph.add_node(String::from("A"));
    graph.add_node(String::from("B"));

    print_graph_nodes(&graph);
} 