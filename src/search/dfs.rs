use std::rc::Rc;

use crate::maze::Maze;

use super::{path::Path, Searcher};

// The Depth-First Searcher object stores a vector of all the paths that are being considered.
pub struct DepthFirstSearcher(Vec<Path>);

impl DepthFirstSearcher {
    pub fn new(maze: &Rc<Maze>) -> DepthFirstSearcher {
        let mut initial_path = Path::new();
        initial_path.push(maze.get_start());
        DepthFirstSearcher(vec![initial_path])
    }
}

impl super::Searcher for DepthFirstSearcher {
    // To get the current path, return the last path in the vector.
    fn get_current_path(&self) -> Option<&Path> {
        self.0.last()
    }

    // To get the considered nodes, return the neighbours of the tail of each path.
    fn get_considered_nodes(&self) -> Vec<crate::maze::MazeNode> {
        self.0
            .iter()
            .filter_map(|path| path.last().cloned())
            .flat_map(|node| node.get_neighbors())
            .collect()
    }

    // To develop the next node, pop the last path from the vector and deepen it.
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
