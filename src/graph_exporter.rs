use std::{error::Error, fs, process::Command};

use crate::graph::Graph;

const DOT_INPUT: &str = "graph.dot";
pub const DOT_OUTPUT: &str = "graph.png";

pub fn export_graph(graph: &Graph) -> Result<(), Box<dyn Error>> {
    let mut f = String::new();

    f.push_str("graph { \n");

    for node in &graph.nodes {
        f = format!("{f}    {} [label=\"{}\"]\n", node.code, node.name);
    }

    for edge in &graph.edges {
        f = format!("{f}    {} -- {}", edge.from, edge.to);

        if graph.is_weighted {
            f = format!("{f} [label=\"{}\"]", edge.weight);
        }

        f = format!("{f}\n")
    }

    f.push_str("}");

    fs::write(DOT_INPUT, f)?;

    let dot_output = Command::new("dot").args(["-Tpng", DOT_INPUT]).output()?;

    fs::write(DOT_OUTPUT, dot_output.stdout)?;

    Ok(())
}
