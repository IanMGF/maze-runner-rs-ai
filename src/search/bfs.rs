use std::{collections::VecDeque, rc::Rc};

use crate::maze::Maze;

use super::{path::Path, Searcher};

// The Breadth-First Searcher object stores a VecDeque, ideal for popping from start, of all the paths that are being considered.
pub struct BreadthFirstSearcher(VecDeque<Path>);

impl BreadthFirstSearcher {
    pub fn new(maze: &Rc<Maze>) -> BreadthFirstSearcher {
        let mut initial_path = Path::new();
        initial_path.push(maze.get_start());
        BreadthFirstSearcher([initial_path].into())
    }
}

impl super::Searcher for BreadthFirstSearcher {
    // To get the current path, return the first path in the vector.
    fn get_current_path(&self) -> Option<&Path> {
        self.0.front()
    }

    // To get the considered nodes, return the neighbours of the tail of each path.
    fn get_considered_nodes(&self) -> Vec<crate::maze::MazeNode> {
        self.0
            .iter()
            .filter_map(|path| path.last().cloned())
            .flat_map(|node| node.get_neighbors())
            .collect()
    }

    // To develop the next node, pop the first path from the vector and deepen it.
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
