use std::{collections::VecDeque, rc::Rc};

use crate::maze::Maze;

use super::{path::Path, MazeNode, Searcher};

type Cost = u64;

// Fn(current_node, end_node) -> heuristical_cost;
pub trait HeuristicFn: Fn(&MazeNode, &MazeNode) -> Cost {}
impl<T> HeuristicFn for T where T: Fn(&MazeNode, &MazeNode) -> Cost {}

pub struct AStarSearcher<F> (Rc<Maze>, VecDeque<(Path, Cost, Cost)>, Box<F>) where F: HeuristicFn;

impl<F: HeuristicFn> AStarSearcher<F> {
    pub fn new(maze: Rc<Maze>, heuristic: Box<F>) -> Option<AStarSearcher<F>> {
        let start_node = maze.get_start();
        
        let mut initial_path: Path = Path::new();
        initial_path.push(start_node.clone());
        
        let initial_path_length = 0 as Cost;
        let starting_heuristic = heuristic(&start_node, &maze.get_end());
        
        let initial_path_list = [(initial_path, initial_path_length, starting_heuristic)].into();
        
        Some(AStarSearcher(maze, initial_path_list, heuristic))
    }
}

impl<F: HeuristicFn> super::Searcher for AStarSearcher<F> {
    fn get_current_path(&self) -> Option<&Path> {
        self.1.iter().min_by_key(|(_, cost, heuristic)| heuristic + cost).map(|(path, _, _)| path)
    }
    
    fn get_considered_nodes(&self) -> Vec<Box<crate::maze::MazeNode>> {
        self.1
            .iter()
            .filter_map(|(path, _, _)| path.last().map(|node| Box::new(node.clone())))
            // .flat_map(|node| node.get_neighbors().into_iter())
            .collect()
    }

    fn develop_next_node(&mut self) -> Option<crate::maze::MazeNode> {
        let (idx, _) = self.1.iter().enumerate().min_by_key(|(_, (_, cost, heuristic))| heuristic + cost)?;
        let (path, cost, _) = self.1.remove(idx)?;
        
        let node = path.last()?.clone();
        
        #[cfg(debug_assertions)]
        println!("{:?}, {}", node.get_coordinates(), cost);
        
        let mut new_paths = path.deepen_path()
            .into_iter()
            .map(|path| {
                #[allow(clippy::expect_used)]
                let node_heuristic = self.2(path.last().expect("Path is empty!"), &self.0.get_end());
                (path, cost + 1, node_heuristic)
            })
            .collect::<VecDeque<(Path, Cost, Cost)>>();
        self.1.append(&mut new_paths);
        Some(node)
    }
}

impl<F: HeuristicFn> Iterator for AStarSearcher<F> {
    type Item = crate::maze::MazeNode;

    fn next(&mut self) -> Option<Self::Item> {
        self.develop_next_node()
    }
}