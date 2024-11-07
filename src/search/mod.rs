pub mod dfs;
pub mod bfs;
pub mod a_star;
pub mod path;

pub use crate::maze::MazeNode;

pub trait Searcher<'a>: Iterator<Item = MazeNode<'a>> {
    fn get_considered_nodes(&self) -> Vec<&MazeNode<'a>>;
    fn develop_next_node(&mut self) -> Option<MazeNode<'a>>;
    fn get_current_path(&self) -> Option<&path::Path<'a>>;
}
