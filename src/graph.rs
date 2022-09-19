use crate::node::Node;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Graph {
    pub size: usize,
    pub nodes: Vec<Node>,
}

impl Graph {
    pub fn find_by_code(&self, code: u32) -> Option<&Node> {
        self.nodes.iter().find(|node| node.code == code)
    }

    fn get_by_id(&self, id: usize) -> Option<&Node> {
        self.nodes.iter().find(|node| node.id == id)
    }

    fn get_by_ids(&self, ids: &Vec<usize>) -> Vec<&Node> {
        self.nodes
            .iter()
            .filter(|node| ids.contains(&node.id))
            .collect()
    }

    pub fn get_path(&self, start_node: &Node, end_node: &Node) -> Option<Vec<&Node>> {
        let mut queue = Vec::new();
        let mut visited = Vec::new();

        queue.push(vec![start_node.id]);
        visited.push(start_node.id);

        while let Some(path) = queue.pop() {
            let current_id = path.last().unwrap();

            if *current_id == end_node.id {
                let nodes = self.get_by_ids(&path);
                return Some(nodes);
            }

            let current_node = self.get_by_id(*current_id).unwrap();

            for id in current_node.edges.iter() {
                if !visited.contains(id) {
                    let mut new_path = path.clone();
                    new_path.push(*id);
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
