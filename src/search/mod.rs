pub mod a_star;
pub mod bfs;
pub mod dfs;
pub mod path;

pub use crate::maze::MazeNode;

pub trait Searcher: Iterator<Item = MazeNode> {
    fn get_considered_nodes(&self) -> Vec<MazeNode>;
    fn develop_next_node(&mut self) -> Option<MazeNode>;
    fn get_current_path(&self) -> Option<&path::Path>;
}
