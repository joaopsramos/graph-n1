use colored::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Node {
    pub code: usize,
    pub name: String,
    pub local_type: String,
    pub edges: Vec<Edge>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Edge {
    pub code: usize,
    pub weight: u32,
}

impl Node {
    pub fn is_adjacent(&self, node2: &Self) -> bool {
        self.edges
            .iter()
            .find(|&edge| edge.code == node2.code)
            .is_some()
    }

    pub fn has_buckle(&self) -> bool {
        self.edges
            .iter()
            .find(|&edge| edge.code == self.code)
            .is_some()
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

fn format_edges(edges: &Vec<Edge>) -> String {
    let mut iter = edges.iter().peekable();
    let mut string = "[".to_string();

    while let Some(edge) = iter.next() {
        string = format!("{string}{}", edge.code.to_string().cyan());

        if iter.peek().is_none() {
            continue;
        }

        string.push_str(", ");
    }

    string.push_str("]");

    string
}
