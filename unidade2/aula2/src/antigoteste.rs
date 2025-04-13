use petgraph::dot::{Dot, Config};
use petgraph::graph::Graph;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
   let mut graph = Graph::<&str, &str>::new();

    // Add nodes to the graph
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    let e = graph.add_node("E");
    let f = graph.add_node("F");

    // Add edges to the graph
        graph.add_edge(a, b, "");
        graph.add_edge(b, f, "");
        graph.add_edge(f, e, "");
        graph.add_edge(c, b, "");
        graph.add_edge(a, f, "");
        graph.add_edge(b, a, "");
        graph.add_edge(b, d, "");

    // .dot
    let dot_output= format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    let mut file = File::create("graph.dot")?;
    file.write_all(dot_output.as_bytes())?;


        println!("Arquivo gravado com sucesso!");
        Ok(())

}