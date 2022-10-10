use crate::node::{Edge, Node};
use serde::{Deserialize, Serialize};

pub enum GraphError {
    EdgeAlreadyExists,
    EdgeDontExists,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Graph {
    pub is_weighted: bool,
    pub size: usize,
    pub nodes: Vec<Node>,
}

impl Graph {
    pub fn find_by_code(&self, code: usize) -> Option<&Node> {
        self.nodes.iter().find(|node| node.code == code)
    }
    pub fn find_by_code_mut(&mut self, code: usize) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|node| node.code == code)
    }

    fn get_by_codes(&self, codes: &Vec<usize>) -> Vec<&Node> {
        self.nodes
            .iter()
            .filter(|node| codes.contains(&node.code))
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

            for code in current_node.edges.iter().map(|e| &e.code) {
                if !visited.contains(code) {
                    let mut new_path = path.clone();
                    new_path.push(*code);
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
            if !current_node
                .edges
                .iter()
                .find(|e| e.code == *code)
                .is_some()
            {
                return None;
            }

            current_node = self.find_by_code(*code).unwrap();
            cycle.push(current_node);
        }

        Some(cycle)
    }

    pub fn format_path(nodes: Vec<Node>) -> Option<String> {
        let mut v_iter = nodes.iter();

        let mut result = match v_iter.next() {
            Some(node) => format!("{}", node),
            None => return None,
        };

        while let Some(node) = v_iter.next() {
            result = format!("{result} -> {node}");
        }

        Some(result)
    }

    pub fn add_edge(&mut self, edge1: Edge, edge2: Edge) -> Result<(), GraphError> {
        let node1 = self.find_by_code_mut(edge1.code).unwrap();

        if node1.edges.contains(&edge2) {
            return Err(GraphError::EdgeAlreadyExists);
        }

        node1.edges.push(edge2);

        if edge1 != edge2 {
            let node2 = self.find_by_code_mut(edge2.code).unwrap();
            node2.edges.push(edge1);
        }

        Ok(())
    }

    pub fn remove_edge(&mut self, edge1: Edge, edge2: Edge) -> Result<(), GraphError> {
        let node1 = self.find_by_code_mut(edge1.code).unwrap();

        if !node1.edges.contains(&edge2) {
            return Err(GraphError::EdgeDontExists);
        }

        node1.edges.retain(|e| *e != edge2);

        let node2 = self.find_by_code_mut(edge2.code).unwrap();

        node2.edges.retain(|e| *e != edge1);

        Ok(())
    }
}
