use crate::node::Node;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub enum GraphError {
    EdgeAlreadyExists,
    EdgeDontExists,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub weight: u32,
}

impl Edge {
    fn has(&self, from: usize, to: usize) -> bool {
        (self.from == from && self.to == to) || (self.from == to && self.to == from)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Graph {
    pub is_weighted: bool,
    pub size: usize,
    pub edges: Vec<Edge>,
    pub nodes: Vec<Node>,
}

impl Graph {
    pub fn make_weighted(&mut self) {
        self.is_weighted = true;
    }

    pub fn find_by_code(&self, code: usize) -> Option<&Node> {
        self.nodes.iter().find(|node| node.code == code)
    }

    pub fn find_edge_by_from_to(&self, from: usize, to: usize) -> Option<&Edge> {
        self.edges.iter().find(|edge| edge.has(from, to))
    }

    pub fn add_edge(&mut self, edge: Edge) -> Result<(), GraphError> {
        match self.edges.iter().find(|e| e.has(edge.from, edge.to)) {
            Some(_) => Err(GraphError::EdgeAlreadyExists),
            None => {
                self.edges.push(edge);
                Ok(())
            }
        }
    }

    pub fn is_complete(&self) -> bool {
        let codes = self.nodes.iter().map(|n| n.code);
        let edges: Vec<_> = self
            .edges
            .iter()
            .flat_map(|e| [[e.from, e.to], [e.to, e.from]])
            .collect();

        for code1 in codes.clone() {
            let codes_without_current = codes.clone().filter(|c| *c != code1);

            for code2 in codes_without_current {
                if !edges.contains(&[code1, code2]) {
                    return false;
                }
            }
        }

        true
    }

    pub fn is_adjacent(&self, node1: &Node, node2: &Node) -> bool {
        self.edges
            .iter()
            .find(|&edge| edge.has(node1.code, node2.code))
            .is_some()
    }

    pub fn has_buckle(&self, node: &Node) -> bool {
        self.edges
            .iter()
            .find(|&edge| edge.has(node.code, node.code))
            .is_some()
    }

    fn get_by_codes(&self, codes: &Vec<usize>) -> Vec<&Node> {
        codes
            .iter()
            .map(|c| self.nodes.iter().find(|n| n.code == *c).unwrap())
            .collect()
    }

    fn find_connected_nodes(&self, node: &Node) -> Vec<usize> {
        self.edges
            .iter()
            .filter_map(|edge| {
                if edge.from == node.code {
                    return Some(edge.to);
                } else if edge.to == node.code {
                    return Some(edge.from);
                }

                None
            })
            .collect()
    }

    fn get_node_edges(&self, node: &Node) -> Vec<Edge> {
        self.edges
            .iter()
            .filter_map(|edge| {
                if edge.from == node.code {
                    return Some(Edge {
                        from: node.code,
                        to: edge.to,
                        ..*edge
                    });
                } else if edge.to == node.code {
                    return Some(Edge {
                        from: node.code,
                        to: edge.from,
                        ..*edge
                    });
                }

                None
            })
            .collect()
    }

    pub fn get_path(&self, start_node: &Node, end_node: &Node) -> Option<Vec<&Node>> {
        let mut queue = Vec::new();
        let mut visited = Vec::new();

        queue.push(vec![start_node.code]);
        visited.push(start_node.code);

        while let Some(path) = queue.pop() {
            let current_code = path.last().unwrap();
            visited.push(*current_code);

            if *current_code == end_node.code {
                let nodes = self.get_by_codes(&path);
                return Some(nodes);
            }

            let current_node = self.find_by_code(*current_code).unwrap();
            let current_node_edges = self.find_connected_nodes(&current_node);

            for code in current_node_edges {
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
            let current_node_edges = self.find_connected_nodes(current_node);

            if !current_node_edges.iter().find(|c| *c == code).is_some() {
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

    pub fn calculate_path(&self, start_node: &Node, end_node: &Node) -> Option<u32> {
        let mut path_sum = 0;
        let path = self.get_path(start_node, end_node)?;
        let mut iter = path.iter().peekable();

        while let Some(node_from) = iter.next() {
            let node_to = iter.peek();

            if node_to.is_none() {
                break;
            }

            let edge = self
                .find_edge_by_from_to(node_from.code, node_to.unwrap().code)
                .unwrap();

            path_sum += edge.weight;
        }

        Some(path_sum)
    }

    pub fn is_subgraph(&self, subgraph: &Graph) -> bool {
        let subgraph_node_codes: Vec<usize> = subgraph.nodes.iter().map(|el| el.code).collect();

        for sub_node in &subgraph.nodes {
            if !self.nodes.contains(&sub_node) {
                return false;
            }

            let sub_node_edges = subgraph.find_connected_nodes(sub_node);
            let node_edges = self.find_connected_nodes(sub_node);

            if sub_node_edges.is_empty() {
                for edge in &node_edges {
                    if subgraph_node_codes.contains(&edge) {
                        return false;
                    }
                }
            }

            for sub_node_edge in &sub_node_edges {
                if !node_edges.contains(&sub_node_edge) {
                    return false;
                }
            }
        }

        return true;
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        let mut iter = self.nodes.iter().peekable();

        while let Some(node) = iter.next() {
            // let connected_nodes = self.get_node_edges(node);

            string = format!("{string}{node}\n");

            if iter.peek().is_some() {
                string = format!("{string}");
            }
        }

        string = format!("{string}\n{}", format_edges(self.is_weighted, &self.edges));
        write!(f, "{string}")
    }
}

fn format_edges(weighted: bool, edges: &Vec<Edge>) -> String {
    let mut string = "".to_string();
    let mut iter = edges.iter().peekable();

    while let Some(edge) = iter.next() {
        string = format!(
            "{string}{} -> {}",
            edge.from.to_string().cyan(),
            edge.to.to_string().cyan()
        );

        if weighted {
            string = format!("{string} Peso = {}", edge.weight);
        }

        string = format!("{string}");
        if iter.peek().is_some() {
            string = format!("{string}\n");
        }
    }

    format!("{string}")
}
