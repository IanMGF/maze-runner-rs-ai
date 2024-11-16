#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Wall,
    Start,
    End,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum EmptyTileState {
    Focused,
    Visited,
    Considering,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TileMap(Vec<Vec<Tile>>);
pub struct TileMapIter(TileMap, usize);

#[allow(dead_code)]
struct TileMapBuildError;

impl TryFrom<Vec<Vec<Tile>>> for TileMap {
    type Error = &'static str;

    fn try_from(value: Vec<Vec<Tile>>) -> Result<Self, Self::Error> {
        let Some(first_row) = value.first() else {
            return Err("TileMap cannot be empty");
        };

        let first_len = first_row.len();

        if value
            .iter()
            .map(|vec| vec.len())
            .any(|size| size != first_len)
        {
            Err("Rows have different lengths")
        } else if value
            .iter()
            .flatten()
            .copied()
            .filter(|e| *e == Tile::Start)
            .count()
            != 1usize
        {
            Err("TileMap must have exactly one Start tile")
        } else if value
            .iter()
            .flatten()
            .copied()
            .filter(|e| *e == Tile::End)
            .count()
            != 1usize
        {
            Err("TileMap must have exactly one End tile")
        } else {
            Ok(TileMap(value))
        }
    }
}

impl TryFrom<String> for TileMap {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let chars: Vec<Vec<char>> = value
            .split('\n')
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();

        let tiles: Vec<Vec<Tile>> = chars
            .iter()
            .map(|row| {
                row.iter()
                    .filter_map(|char| match char {
                        '0' => Some(Tile::Empty),
                        '1' => Some(Tile::Wall),
                        '2' => Some(Tile::Start),
                        '3' => Some(Tile::End),
                        ' ' | '\t' | '\n' | '\r' => None,
                        _ => panic!("Expected values 0, 1, 2, or 3. Found {:?}", char),
                    })
                    .collect::<Vec<Tile>>()
            })
            .filter(|row| !row.is_empty())
            .collect::<Vec<Vec<Tile>>>();

        TryInto::<TileMap>::try_into(tiles)
    }
}

impl TileMap {
    pub fn width(&self) -> usize {
        #[allow(clippy::expect_used)]
        return self.0.first().expect("TileMap should never be empty").len();
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Tile> {
        self.0.get(y).and_then(|row| row.get(x)).copied()
    }
}

impl Iterator for TileMapIter {
    type Item = (Tile, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.1 % self.0.width();
        let y = self.1 / self.0.width();

        if y >= self.0.height() {
            return None;
        }

        let Some(tile) = self.0.get(x, y) else {
            unreachable!()
        };

        self.1 += 1;
        Some((tile, x, y))
    }
}

impl<'a> From<&'a TileMap> for TileMapIter {
    fn from(value: &'a TileMap) -> Self {
        TileMapIter(value.clone(), 0)
    }
}
