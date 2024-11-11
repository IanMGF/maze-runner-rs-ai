use std::collections::{vec_deque::Iter, VecDeque};

use crate::maze::MazeNode;

#[derive(Clone, Debug)]
pub struct Path(VecDeque<MazeNode>);
impl Path {
    pub fn new() -> Path {
        Path(VecDeque::new())
    }
    
    pub fn push(&mut self, node: MazeNode) {
        self.0.push_back(node);
    }
    
    pub fn first(&self) -> Option<&MazeNode> {
        self.0.front()
    }
    
    pub fn last(&self) -> Option<&MazeNode> {
        self.0.back()
    }
    
    pub fn iter(&self) -> Iter<MazeNode> {
        self.0.iter()
    }
    
    pub fn contains(&self, node: &MazeNode) -> bool {
        self.0.contains(node)
    }
    
    pub fn get_next(&self) -> Vec<MazeNode> {
        let Some(node) = self.0.back() else {
            return vec![];
        };
        
        let neighbors = node.get_neighbors();
        
        vec![
            neighbors.up,
            neighbors.left,
            neighbors.right,
            neighbors.down
        ].into_iter().flatten().filter(|node| !self.contains(node)).collect()
    }
    
    pub fn deepen_path(self) -> Vec<Path> {
        let Some(node) = self.0.back() else {
            return vec![self];
        };
        
        let neighbors = node.get_neighbors();
        
        let next_nodes: Vec<MazeNode> = vec![
            neighbors.up,
            neighbors.left,
            neighbors.down,
            neighbors.right
        ].into_iter().flatten().filter(|node| !self.contains(node)).collect();
        
        let mut new_paths = vec![];
        for node in next_nodes.into_iter() {
            let mut new_path = self.clone();
            new_path.push(node);
            new_paths.push(new_path);
        }
        
        new_paths
    }
}

impl Default for Path {
    fn default() -> Self {
        Self::new()
    }
}
