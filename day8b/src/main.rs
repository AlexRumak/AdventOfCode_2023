use petgraph::{graph::{NodeIndex, Graph}, data::Build, visit::NodeRef};
use std::{fs::{self, File}, collections::HashMap, io::Write};
use std::io::prelude::*;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file"); 

    let mut lines = contents.lines();
    let instructions = lines.next().expect("Instructions should exist");
    lines.next(); // skip blank line

    let mut g: Graph<String, u32> = Graph::new();
    
    for line in lines {
        let mut split = line.split("=");
        let node_val = split.next().expect("Should have a left side").trim().to_string();
        let edges_to = split.next().expect("Should have a right side").trim().to_string();
        let edges_split = edges_to.split(", ");

        let a = g.add_node(node_val); 

        for edge in edges_split {
            let edge_val = edge.trim().replace('(', "").replace(')', "");
            let b = g.add_node(edge_val);
            g.add_edge(a, b, 1);
        }
    }

    write_to_file_graph(&g).expect("Should have been able to write to file");
}

fn write_to_file_graph(g: &Graph<String, u32>) -> std::io::Result<()> {
    let mut file = File::create("output.txt")?;
    writeln!(file, "digraph G {{")?;
    for edge in g.edge_indices() {
        let (a, b) = g.edge_endpoints(edge).expect("Should have endpoints");
        writeln!(file, "    {} -> {}", g.node_weight(a).expect("Should have a weight"), g.node_weight(b).expect("Should have a weight"))?;
    }
    writeln!(file, "}}")?;

    Ok(())
}