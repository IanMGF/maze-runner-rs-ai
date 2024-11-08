use super::{path::Path, Searcher};

pub struct DepthFirstSearcher<'a> (Vec<Path<'a>>);

impl DepthFirstSearcher<'_> {
    pub fn new(initial_path: Path<'_>) -> DepthFirstSearcher<'_> {
        DepthFirstSearcher(vec![initial_path])
    }
}
impl<'a> super::Searcher<'a> for DepthFirstSearcher<'a> {
    fn get_current_path(&self) -> Option<&Path<'a>> {
        self.0.last()
    }
    
    fn get_considered_nodes(&self) -> Vec<Box<crate::maze::MazeNode<'a>>> {
        self.0.iter().filter_map(|path| path.last().map(|node| Box::new(node.clone()))).collect()
    }

    fn develop_next_node(&mut self) -> Option<crate::maze::MazeNode<'a>> {
        let path = self.0.pop()?.to_owned();
        let node = path.last()?.clone();
        self.0.append(&mut path.deepen_path());
        Some(node)
    }
}

impl<'a> Iterator for DepthFirstSearcher<'a> {
    type Item = crate::maze::MazeNode<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.develop_next_node()
    }
}