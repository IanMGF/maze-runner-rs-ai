use std::{collections::VecDeque, rc::Rc};

use crate::maze::Maze;

use super::{path::Path, Searcher};

pub struct BreadthFirstSearcher (VecDeque<Path>);

impl BreadthFirstSearcher {
    pub fn new(maze: Rc<Maze>) -> BreadthFirstSearcher {
        let mut initial_path = Path::new();
        initial_path.push(maze.get_start());
        BreadthFirstSearcher([initial_path].into())
    }
}

impl super::Searcher for BreadthFirstSearcher {
    fn get_current_path(&self) -> Option<&Path> {
        self.0.front()
    }
    
    fn get_considered_nodes(&self) -> Vec<Box<crate::maze::MazeNode>> {
        self.0.iter().filter_map(|path| path.last().map(|node| Box::new(node.clone()))).collect()
    }

    fn develop_next_node(&mut self) -> Option<crate::maze::MazeNode> {
        let path = self.0.pop_front()?.to_owned();
        let node = path.last()?.clone();
        
        let mut new_paths = path.deepen_path().into_iter().collect::<VecDeque<Path>>();
        self.0.append(&mut new_paths);
        Some(node)
    }
}

impl Iterator for BreadthFirstSearcher {
    type Item = crate::maze::MazeNode;

    fn next(&mut self) -> Option<Self::Item> {
        self.develop_next_node()
    }
}