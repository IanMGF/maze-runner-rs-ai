use std::rc::Rc;

use crate::maze::Maze;

use super::{path::Path, Searcher};

pub struct DepthFirstSearcher (Vec<Path>);

impl DepthFirstSearcher {
    pub fn new(maze: Rc<Maze>) -> DepthFirstSearcher {
        let mut initial_path = Path::new();
        initial_path.push(maze.get_start());
        DepthFirstSearcher(vec![initial_path])
    }
}
impl super::Searcher for DepthFirstSearcher {
    fn get_current_path(&self) -> Option<&Path> {
        self.0.last()
    }
    
    fn get_considered_nodes(&self) -> Vec<Box<crate::maze::MazeNode>> {
        self.0.iter().filter_map(|path| path.last().map(|node| Box::new(node.clone()))).collect()
    }

    fn develop_next_node(&mut self) -> Option<crate::maze::MazeNode> {
        let path = self.0.pop()?.to_owned();
        let node = path.last()?.clone();
        self.0.append(&mut path.deepen_path());
        Some(node)
    }
}

impl Iterator for DepthFirstSearcher {
    type Item = crate::maze::MazeNode;

    fn next(&mut self) -> Option<Self::Item> {
        self.develop_next_node()
    }
}