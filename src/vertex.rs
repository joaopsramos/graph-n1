use colored::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Node {
    pub id: usize,
    pub code: u32,
    pub name: String,
    pub local_type: String,
    pub edges: Vec<usize>,
}

impl Node {
    pub fn is_adjacent(&self, vertex2: &Self) -> bool {
        self.edges.contains(&vertex2.id)
    }

    pub fn has_cycle(&self) -> bool {
        self.edges.contains(&self.id)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}Código: {}, Nome: {}, Tipo do local: {}{}",
            "|".green(),
            self.code.to_string().cyan(),
            self.name.cyan(),
            self.local_type.cyan(),
            "|".green()
        )
    }
}
