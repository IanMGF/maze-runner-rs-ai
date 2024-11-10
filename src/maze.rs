use std::{fmt::{Debug, Formatter}, rc::Rc};

use crate::tilemap::{Tile, TileMap, TileMapIter};

type Coordinates = (usize, usize);

#[derive(PartialEq)]
pub struct Maze {
    map: TileMap,
    start_coord: Coordinates,
    end_coord: Coordinates,
}

#[derive(PartialEq, Clone)]
pub struct MazeNode {
    coord: (usize, usize),
    maze: Rc<Maze>
}

pub struct Neighbours {
    pub up: Option<MazeNode>,
    pub left: Option<MazeNode>,
    pub down: Option<MazeNode>,
    pub right: Option<MazeNode>,
}

pub struct NeighboursIter(Neighbours);

impl IntoIterator for Neighbours {
    type Item = MazeNode;

    type IntoIter = NeighboursIter;

    fn into_iter(self) -> Self::IntoIter {
        NeighboursIter(self)
    }
}

impl Iterator for NeighboursIter {
    type Item = MazeNode;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.up.take()
            .or(self.0.left.take())
            .or(self.0.down.take())
            .or(self.0.right.take())
    }
}


impl Maze {
    pub fn get_node(self: &Rc<Self>, coord: (usize, usize)) -> Option<MazeNode> {
        self.map.get(coord.0, coord.1).map(|_| MazeNode{
            coord,
            maze: self.clone()
        })
    }
    
    pub fn get_start(self: &Rc<Self>) -> MazeNode {
        MazeNode {
            coord: self.start_coord,
            maze: self.clone()
        }
    }
    
    pub fn get_end(self: &Rc<Self>) -> MazeNode {
        MazeNode {
            coord: self.end_coord,
            maze: self.clone()
        }
    }
    
    pub fn width(&self) -> usize {
        self.map.width()
    }
    
    pub fn height(&self) -> usize {
        self.map.height()
    }
    
    pub fn manhattan_distance(coord1: (usize, usize), coord2: (usize, usize)) -> usize {
        usize::abs_diff(coord1.0, coord2.0) + usize::abs_diff(coord1.1, coord2.1)
    }
}

impl MazeNode {
    pub fn get_tile(&self) -> Tile {
        #[allow(clippy::expect_used)]
        self.maze.map.get(self.coord.0, self.coord.1).expect("MazeNode has wrong coordinates")
    }
    
    pub fn get_coordinates(&self) -> (usize, usize) {
        (self.coord.0, self.coord.1)
    }
    
    pub fn get_neighbors(&self) -> Neighbours {
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
        let iterator: TileMapIter = (&value).into();
        #[allow(clippy::expect_used)]
        let start_coord = iterator.filter(|(tile, _, _)| *tile == Tile::Start).map(|(_, x, y)| (x, y)).next().expect("TileMap has no start node");
        
        let iterator: TileMapIter = (&value).into();
        #[allow(clippy::expect_used)]
        let end_coord = iterator.filter(|(tile, _, _)| *tile == Tile::End).map(|(_, x, y)| (x, y)).next().expect("TileMap has no end node");
        
        Maze {
            map: value, 
            start_coord, 
            end_coord
        }
    }
}

impl Debug for MazeNode {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self.coord)
    }
}