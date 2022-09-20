use crate::node::Node;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Graph {
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

            for code in current_node.edges.iter() {
                if !visited.contains(code) {
                    let mut new_path = path.clone();
                    new_path.push(*code);
                    queue.push(new_path);
                }
            }
        }

        return None;
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
}
