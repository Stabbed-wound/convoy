use std::ops::{ Index, IndexMut };

use crate::{ constants::{ BOARD_X, BOARD_Y }, coord::Coord, pieces::Piece };

pub type Tile = Option<Piece>;

/// Neighbours are orthogonal only
#[derive(Clone, Debug, Default)]
pub struct Board {
    tiles: [[Tile; BOARD_X as usize]; BOARD_Y as usize],
}

impl Index<Coord> for Board {
    type Output = Tile;

    fn index(&self, Coord { rank, file }: Coord) -> &Self::Output {
        &self.tiles[rank as usize][file as usize]
    }
}

impl IndexMut<Coord> for Board {
    fn index_mut(&mut self, Coord { rank, file }: Coord) -> &mut Self::Output {
        &mut self.tiles[rank as usize][file as usize]
    }
}

impl Board {
    #[must_use]
    pub fn get_neighbours(&self, Coord { rank, file }: Coord) -> Vec<Tile> {
        [
            Coord::new(rank + 1, file),
            Coord::new(rank - 1, file),
            Coord::new(rank, file + 1),
            Coord::new(rank, file - 1),
        ]
            .iter()
            .filter_map(|coord| coord.map(|coord| self[coord]))
            .collect()
    }
}
