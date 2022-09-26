use colored::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Node {
    pub code: usize,
    pub name: String,
    pub local_type: String,
    pub edges: Vec<usize>,
}

impl Node {
    pub fn is_adjacent(&self, node2: &Self) -> bool {
        self.edges.contains(&node2.code)
    }

    pub fn has_buckle(&self) -> bool {
        self.edges.contains(&self.code)
    }

    pub fn remove_edge(&mut self, node_to_remove: &Self) {
        self.edges.retain(|code| *code != node_to_remove.code)
    }

    pub fn add_edge(&mut self, edge: usize) {
        self.edges.push(edge)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}Código: {}, Nome: {}, Tipo do local: {}, Arestas: {}{}",
            "|".green(),
            self.code.to_string().cyan(),
            self.name.cyan(),
            self.local_type.cyan(),
            format_edges(&self.edges),
            "|".green()
        )
    }
}

fn format_edges(edges: &Vec<usize>) -> String {
    let mut iter = edges.iter().peekable();
    let mut string = "[".to_string();

    while let Some(edge) = iter.next() {
        string = format!("{string}{}", edge.to_string().cyan());

        if iter.peek().is_none() {
            continue;
        }

        string.push_str(", ");
    }

    string.push_str("]");

    string
}
