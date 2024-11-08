use std::collections::LinkedList;

use super::{path::Path, Searcher};

pub struct BreadthFirstSearcher<'a> (LinkedList<Path<'a>>);

impl BreadthFirstSearcher<'_> {
    pub fn new(initial_path: Path<'_>) -> BreadthFirstSearcher<'_> {
        BreadthFirstSearcher([initial_path].into())
    }
}

impl<'a> super::Searcher<'a> for BreadthFirstSearcher<'a> {
    fn get_current_path(&self) -> Option<&Path<'a>> {
        self.0.front()
    }
    
    fn get_considered_nodes(&self) -> Vec<Box<crate::maze::MazeNode<'a>>> {
        self.0.iter().filter_map(|path| path.last().map(|node| Box::new(node.clone()))).collect()
    }

    fn develop_next_node(&mut self) -> Option<crate::maze::MazeNode<'a>> {
        let path = self.0.pop_front()?.to_owned();
        let node = path.last()?.clone();
        
        let mut new_paths = path.deepen_path().into_iter().collect::<LinkedList<Path<'a>>>();
        self.0.append(&mut new_paths);
        Some(node)
    }
}

impl<'a> Iterator for BreadthFirstSearcher<'a> {
    type Item = crate::maze::MazeNode<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.develop_next_node()
    }
}