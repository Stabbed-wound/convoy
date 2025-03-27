use std::array;
use std::ops::{Index, IndexMut};

use crate::tile::{Tile, TileType};
use crate::{
    constants::{BOARD_FILES, BOARD_RANKS},
    coord::Coord,
};

#[derive(Clone, Debug)]
pub struct Board {
    tiles: [[Tile; BOARD_FILES as usize]; BOARD_RANKS as usize],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            tiles: array::from_fn(|rank| {
                if rank == 0 || rank == (BOARD_RANKS - 1) as usize {
                    array::from_fn(|_| Tile::new(None, TileType::Baseline))
                } else {
                    array::from_fn(|_| Tile::default())
                }
            }),
        }
    }
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
    /// Neighbours are orthogonal only
    #[must_use]
    pub fn get_neighbours(&self, Coord { rank, file }: Coord) -> Vec<Tile> {
        [
            Coord::new(rank + 1, file),
            Coord::new(rank.wrapping_sub(1), file),
            Coord::new(rank, file + 1),
            Coord::new(rank, file.wrapping_sub(1)),
        ]
        .iter()
        .filter_map(|coord| coord.map(|coord| self[coord]))
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_board() {
        let board = Board::default();

        assert_eq!(
            board.tiles,
            [
                [
                    Tile::new(None, TileType::Baseline),
                    Tile::new(None, TileType::Baseline),
                    Tile::new(None, TileType::Baseline),
                    Tile::new(None, TileType::Baseline),
                    Tile::new(None, TileType::Baseline),
                    Tile::new(None, TileType::Baseline),
                    Tile::new(None, TileType::Baseline),
                    Tile::new(None, TileType::Baseline)
                ],
                [
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular)
                ],
                [
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular)
                ],
                [
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular)
                ],
                [
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular)
                ],
                [
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular)
                ],
                [
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular),
                    Tile::new(None, TileType::Regular)
                ],
                [
                    Tile::new(None, TileType::Baseline),
                    Tile::new(None, TileType::Baseline),
                    Tile::new(None, TileType::Baseline),
                    Tile::new(None, TileType::Baseline),
                    Tile::new(None, TileType::Baseline),
                    Tile::new(None, TileType::Baseline),
                    Tile::new(None, TileType::Baseline),
                    Tile::new(None, TileType::Baseline)
                ],
            ]
        );
    }
}
