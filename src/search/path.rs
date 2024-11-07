use std::collections::{linked_list::Iter, LinkedList};

use crate::maze::MazeNode;

#[derive(Clone, Debug)]
pub struct Path<'a>(LinkedList<MazeNode<'a>>);
impl<'a> Path<'a> {
    pub fn new() -> Path<'a> {
        Path::<'a>(LinkedList::new())
    }
    
    pub fn push(&mut self, node: MazeNode<'a>) {
        self.0.push_back(node);
    }
    
    pub fn first(&self) -> Option<&MazeNode<'a>> {
        self.0.front()
    }
    
    pub fn last(&self) -> Option<&MazeNode<'a>> {
        self.0.back()
    }
    
    pub fn iter(&self) -> Iter<MazeNode<'_>> {
        self.0.iter()
    }
    
    pub fn contains(&self, node: &MazeNode<'a>) -> bool {
        self.0.contains(node)
    }
    
    pub fn get_next(&self) -> Vec<MazeNode<'a>> {
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
    
    pub fn deepen_path(self) -> Vec<Path<'a>> {
        let Some(node) = self.0.back() else {
            return vec![self];
        };
        
        let neighbors = node.get_neighbors();
        
        let next_nodes: Vec<MazeNode<'a>> = vec![
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

impl<'a> Default for Path<'a> {
    fn default() -> Self {
        Self::new()
    }
}
