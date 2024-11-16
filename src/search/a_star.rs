use std::{collections::VecDeque, rc::Rc};

use crate::maze::Maze;

use super::{path::Path, MazeNode, Searcher};

// Type-alias "cost" for better semantics
type Cost = u64;

// Fn(current_node, end_node) -> heuristical_cost;
pub trait HeuristicFn: Fn(&MazeNode, &MazeNode) -> Cost {}
impl<T> HeuristicFn for T where T: Fn(&MazeNode, &MazeNode) -> Cost {}

// The A* searcher will store a vector of tuples, each with a path, the cost of the path, and the predicted cost from the last node to the end node.
pub struct AStarSearcher<F>(Rc<Maze>, VecDeque<(Path, Cost, Cost)>, Box<F>)
where
    F: HeuristicFn;

impl<F: HeuristicFn> AStarSearcher<F> {
    pub fn new(maze: Rc<Maze>, heuristic: Box<F>) -> AStarSearcher<F> {
        let start_node = maze.get_start();

        let mut initial_path: Path = Path::new();
        initial_path.push(start_node.clone());

        let initial_path_length = 0 as Cost;
        let starting_heuristic = heuristic(&start_node, &maze.get_end());

        let initial_path_list = [(initial_path, initial_path_length, starting_heuristic)].into();

        AStarSearcher(maze, initial_path_list, heuristic)
    }
}

impl<F: HeuristicFn> super::Searcher for AStarSearcher<F> {
    // To get the current path, we will return the path with the lowest cost + heuristic.
    fn get_current_path(&self) -> Option<&Path> {
        self.1
            .iter()
            .min_by_key(|(_, cost, heuristic)| heuristic + cost)
            .map(|(path, _, _)| path)
    }

    // To get the considered nodes, return the neighbours of the last node of each path.
    fn get_considered_nodes(&self) -> Vec<MazeNode> {
        self.1
            .iter()
            .filter_map(|(path, _, _)| (path.last().cloned()))
            .flat_map(|node| node.get_neighbors())
            .collect()
    }

    // To develop the next node, we will take the path with the lowest cost + heuristic, and deepen it.
    #[allow(clippy::expect_used)]
    fn develop_next_node(&mut self) -> Option<MazeNode> {
        let (idx, _) = self
            .1
            .iter()
            .enumerate()
            .min_by_key(|(_, (_, cost, heuristic))| heuristic + cost)?;
        let Some((path, cost, _)) = self.1.remove(idx) else {
            unreachable!("Validated index is out-of-bounds")
        };

        let node = path.last()?.clone();

        let mut new_paths: VecDeque<(Path, Cost, Cost)> = path
            .deepen_path()
            .into_iter()
            .map(|path| {
                let node_heuristic =
                    (self.2)(path.last().expect("Path is empty!"), &self.0.get_end());
                (path, cost + 1, node_heuristic)
            })
            .collect();

        self.1.append(&mut new_paths);
        Some(node)
    }
}

impl<F: HeuristicFn> Iterator for AStarSearcher<F> {
    type Item = MazeNode;

    fn next(&mut self) -> Option<Self::Item> {
        self.develop_next_node()
    }
}
