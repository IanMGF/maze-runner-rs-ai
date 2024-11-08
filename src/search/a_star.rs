use std::collections::VecDeque;

use super::{path::Path, MazeNode, Searcher};

pub struct AStarSearcher<'a, F> (VecDeque<(Path<'a>, u64, u64)>, F) where F: Fn(&MazeNode<'_>) -> u64;

impl<F: Fn(&MazeNode<'_>) -> u64> AStarSearcher<'_, F> {
    pub fn new(initial_path: Path<'_>, heuristic: F) -> Option<AStarSearcher<'_, F>> {
        let start_node = initial_path.last()?;
        let initial_path_length = initial_path.iter().len() as u64;
        let starting_heuristic = heuristic(start_node);
        Some(AStarSearcher([(initial_path, initial_path_length, starting_heuristic)].into(), heuristic))
    }
}

impl<'a, F: Fn(&MazeNode<'_>) -> u64> super::Searcher<'a> for AStarSearcher<'a, F> {
    fn get_current_path(&self) -> Option<&Path<'a>> {
        self.0.iter().min_by_key(|(_, cost, heuristic)| heuristic + cost).map(|(path, _, _)| path)
    }
    
    fn get_considered_nodes(&self) -> Vec<Box<crate::maze::MazeNode<'a>>> {
        self.0
            .iter()
            .filter_map(|(path, _, _)| path.last().map(|node| Box::new(node.clone())))
            // .flat_map(|node| node.get_neighbors().into_iter())
            .collect()
    }

    fn develop_next_node(&mut self) -> Option<crate::maze::MazeNode<'a>> {
        let (idx, _) = self.0.iter().enumerate().min_by_key(|(_, (_, cost, heuristic))| heuristic + cost)?;
        let (path, cost, _) = self.0.remove(idx)?;
        
        let node = path.last()?.clone();
        
        #[cfg(debug_assertions)]
        println!("{:?}, {}", node.get_coordinates(), cost);
        
        let mut new_paths = path.deepen_path()
            .into_iter()
            .map(|path| {
                #[allow(clippy::expect_used)]
                let node_heuristic = self.1(path.last().expect("Path is empty!"));
                (path, cost + 1, node_heuristic)
            })
            .collect::<VecDeque<(Path<'a>, u64, u64)>>();
        self.0.append(&mut new_paths);
        Some(node)
    }
}

impl<'a, F: Fn(&MazeNode<'_>) -> u64> Iterator for AStarSearcher<'a, F> {
    type Item = crate::maze::MazeNode<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.develop_next_node()
    }
}