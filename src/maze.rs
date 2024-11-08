use std::fmt::{Debug, Formatter};

use crate::tilemap::{Tile, TileMap, TileMapIter};

#[derive(PartialEq)]
pub struct Maze(TileMap);

#[derive(PartialEq, Clone)]
pub struct MazeNode<'a> {
    coord: (usize, usize),
    maze: &'a Maze
}

pub struct Neighbours<'a> {
    pub up: Option<MazeNode<'a>>,
    pub left: Option<MazeNode<'a>>,
    pub down: Option<MazeNode<'a>>,
    pub right: Option<MazeNode<'a>>,
}

pub struct NeighboursIter<'a> (Neighbours<'a>);

impl<'a> IntoIterator for Neighbours<'a> {
    type Item = Box<MazeNode<'a>>;

    type IntoIter = NeighboursIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        NeighboursIter(self)
    }
}

impl<'a> Iterator for NeighboursIter<'a> {
    type Item = Box<MazeNode<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.up.take().map(Box::new)
            .or(self.0.left.take().map(Box::new))
            .or(self.0.down.take().map(Box::new))
            .or(self.0.right.take().map(Box::new))
    }
}


impl<'a> Maze {
    pub fn get_node(&'a self, coord: (usize, usize)) -> Option<MazeNode<'a>> {
        self.0.get(coord.0, coord.1).map(|_| MazeNode::<'a>{
            coord,
            maze: self
        })
    }
    
    pub fn get_start(&'a self) -> MazeNode<'a> {
        let iterator: TileMapIter = (&self.0).into();
        #[allow(clippy::expect_used)]
        iterator.filter(|(tile, _, _)| *tile == Tile::Start).map(|(_, x, y)| MazeNode::<'a>{
            coord: (x, y),
            maze: self
        }).next().expect("Maze has no start node")
    }
    
    pub fn get_end(&'a self) -> MazeNode<'a> {
        let iterator: TileMapIter = (&self.0).into();
        #[allow(clippy::expect_used)]
        iterator.filter(|(tile, _, _)| *tile == Tile::End).map(|(_, x, y)| MazeNode::<'a>{
            coord: (x, y),
            maze: self
        }).next().expect("Maze has no end node")
    }
    
    pub fn width(&self) -> usize {
        self.0.width()
    }
    
    pub fn height(&self) -> usize {
        self.0.height()
    }
}

impl<'a> MazeNode<'a> {
    pub fn get_tile(&self) -> Tile {
        #[allow(clippy::expect_used)]
        self.maze.0.get(self.coord.0, self.coord.1).expect("MazeNode has wrong coordinates")
    }
    
    pub fn get_coordinates(&self) -> (usize, usize) {
        (self.coord.0, self.coord.1)
    }
    
    pub fn get_neighbors(&self) -> Neighbours<'a> {
        let up_coord = match self.coord.1 {
            0 => None,
            _ => Some((self.coord.0, self.coord.1 - 1))
        };
        let left_coord = match self.coord.0 {
            0 => None,
            _ => Some((self.coord.0 - 1, self.coord.1))
        };
        let down_coord = match self.coord.1 {
            y if y == self.maze.height() - 1 => None,
            _ => Some((self.coord.0, self.coord.1 + 1))
        };
        let right_coord = match self.coord.0 {
            x if x == self.maze.width() - 1 => None,
            _ => Some((self.coord.0 + 1, self.coord.1))
        };
        
        let up = up_coord.and_then(|coord| self.maze.get_node(coord)).filter(|node| node.get_tile() != Tile::Wall);
        let left = left_coord.and_then(|coord| self.maze.get_node(coord)).filter(|node| node.get_tile() != Tile::Wall);
        let down = down_coord.and_then(|coord| self.maze.get_node(coord)).filter(|node| node.get_tile() != Tile::Wall);
        let right = right_coord.and_then(|coord| self.maze.get_node(coord)).filter(|node| node.get_tile() != Tile::Wall);
        
        Neighbours { up, left, down, right }
    }
}

impl From<TileMap> for Maze {
    fn from(value: TileMap) -> Self {
        Maze(value)
    }
}

impl Debug for MazeNode<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self.coord)
    }
}