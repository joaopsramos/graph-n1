use crate::vertex::Node;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Graph {
    pub size: usize,
    pub vertices: Vec<Node>,
}

impl Graph {
    pub fn find_by_code(&self, code: u32) -> Option<&Node> {
        self.vertices.iter().find(|vertex| vertex.code == code)
    }

    fn get_by_id(&self, id: usize) -> Option<&Node> {
        self.vertices.iter().find(|vertex| vertex.id == id)
    }

    fn get_by_ids(&self, ids: &Vec<usize>) -> Vec<&Node> {
        self.vertices
            .iter()
            .filter(|vertex| ids.contains(&vertex.id))
            .collect()
    }

    pub fn get_path(&self, start_vertex: &Node, end_vertex: &Node) -> Option<Vec<&Node>> {
        let mut queue = Vec::new();
        let mut visited = Vec::new();

        queue.push(vec![start_vertex.id]);
        visited.push(start_vertex.id);

        while let Some(path) = queue.pop() {
            let current_id = path.last().unwrap();

            if *current_id == end_vertex.id {
                let vertices = self.get_by_ids(&path);
                return Some(vertices);
            }

            let current_vertex = self.get_by_id(*current_id).unwrap();

            for id in current_vertex.edges.iter() {
                if !visited.contains(id) {
                    let mut new_path = path.clone();
                    new_path.push(*id);
                    queue.push(new_path);
                }
            }
        }

        return None;
    }

    pub fn format_path(vertices: Vec<Node>) -> Option<String> {
        let mut v_iter = vertices.iter();

        let mut result = match v_iter.next() {
            Some(vertex) => format!("{}", vertex),
            None => return None,
        };

        while let Some(vertex) = v_iter.next() {
            result = format!("{result} -> {vertex}");
        }

        Some(result)
    }
}
