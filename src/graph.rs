use crate::node::Node;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub enum GraphError {
    EdgeAlreadyExists,
    EdgeDontExists,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Graph {
    pub is_weighted: bool,
    pub size: usize,
    pub edges: Vec<Edge>,
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub weight: u32,
}

impl Graph {
    pub fn make_weighted(&mut self) {
        self.is_weighted = true;
    }

    pub fn find_by_code(&self, code: usize) -> Option<&Node> {
        self.nodes.iter().find(|node| node.code == code)
    }

    pub fn find_edge_by_from_to(&self, from: usize, to: usize) -> Option<&Edge> {
        self.edges.iter().find(|edge| {
            (edge.from == from && edge.to == to) || (edge.from == to && edge.to == from)
        })
    }
    pub fn add_edge(&mut self, edge: Edge) -> Result<(), GraphError> {
        if self.edges.contains(&edge) {
            Err(GraphError::EdgeAlreadyExists)
        } else {
            self.edges.push(edge);
            Ok(())
        }
    }

    pub fn is_adjacent(&self, node1: &Node, node2: &Node) -> bool {
        self.edges
            .iter()
            .find(|&edge| {
                let from_to = (edge.from, edge.to);

                from_to == (node1.code, node2.code) || from_to == (node2.code, node1.code)
            })
            .is_some()
    }

    pub fn has_buckle(&self, node: &Node) -> bool {
        self.edges
            .iter()
            .find(|&edge| edge.from == node.code && edge.to == node.code)
            .is_some()
    }

    fn get_by_codes(&self, codes: &Vec<usize>) -> Vec<&Node> {
        self.nodes
            .iter()
            .filter(|node| codes.contains(&node.code))
            .collect()
    }

    fn find_edges_from_node(&self, node: &Node) -> Vec<&Edge> {
        self.edges
            .iter()
            .filter(|edge| edge.from == node.code)
            .collect()
    }

    pub fn get_path(&self, start_node: &Node, end_node: &Node) -> Option<Vec<&Node>> {
        let mut queue = Vec::new();
        let mut visited = Vec::new();

        queue.push(vec![start_node.code]);
        visited.push(start_node.code);

        while let Some(path) = queue.pop() {
            let current_code = path.last().unwrap();

            if *current_code == end_node.code {
                let nodes = self.get_by_codes(&path);
                return Some(nodes);
            }

            let current_node = self.find_by_code(*current_code).unwrap();
            let current_node_edges = self.find_edges_from_node(&current_node);

            for code in current_node_edges.iter().map(|e| e.to) {
                if !visited.contains(&code) {
                    let mut new_path = path.clone();
                    new_path.push(code);
                    queue.push(new_path);
                }
            }
        }

        return None;
    }

    pub fn is_cycle(codes: &Vec<usize>) -> bool {
        if codes.len() <= 2 || codes.first().unwrap() != codes.last().unwrap() {
            return false;
        }

        let mut unique_codes = codes.clone();
        unique_codes.sort_unstable();
        unique_codes.dedup();

        // The +1 here is because the first and last elements must be equal,
        // so it's removed on dedup
        if codes.len() != unique_codes.len() + 1 {
            return false;
        }

        return true;
    }

    pub fn get_cycle(&self, codes: &Vec<usize>) -> Option<Vec<&Node>> {
        if !Self::is_cycle(&codes) {
            return None;
        }

        let mut cycle = Vec::new();
        let mut codes_iter = codes.iter();

        let first_node = match self.find_by_code(*codes_iter.next().unwrap()) {
            Some(node) => node,
            None => return None,
        };

        cycle.push(first_node);

        let mut current_node = first_node;

        while let Some(code) = codes_iter.next() {
            let current_node_edges = self.find_edges_from_node(current_node);

            if !current_node_edges.iter().find(|e| e.to == *code).is_some() {
                return None;
            }

            current_node = self.find_by_code(*code).unwrap();
            cycle.push(current_node);
        }

        Some(cycle)
    }

    pub fn add_weight(edge: &mut Edge, weight: u32) {
        edge.weight = weight;
    }

    pub fn remove_edge(&mut self, from: usize, to: usize) -> Result<(), GraphError> {
        let edge = self.find_edge_by_from_to(from, to);

        if edge.is_none() {
            return Err(GraphError::EdgeDontExists);
        }

        let edge = edge.unwrap().clone();

        self.edges.retain(|e| e != &edge);

        Ok(())
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();

        for node in &self.nodes {
            string = format!("{string}{node}\n");
        }

        write!(f, "{string}")
    }
}
