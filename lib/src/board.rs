use std::ops::{Index, IndexMut};
use std::{array, slice};

use crate::{
    constants::{BOARD_FILES, BOARD_RANKS},
    coord::Coord,
    tile::{Tile, TileType},
    Player,
};

#[derive(Clone, Debug)]
pub struct Board {
    tiles: [[Tile; BOARD_FILES as usize]; BOARD_RANKS as usize],
}

impl Board {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn get_moves(&self, piece_coord: Coord) -> Option<Vec<Coord>> {
        Some(self[piece_coord].piece_option?.get_moves(piece_coord, self))
    }

    /// Neighbours are orthogonal only
    #[must_use]
    pub fn get_neighbours(&self, Coord { rank, file }: Coord) -> Vec<Tile> {
        [
            Coord::new(rank + 1, file),
            Coord::new(rank.wrapping_sub(1), file),
            Coord::new(rank, file + 1),
            Coord::new(rank, file.wrapping_sub(1)),
        ]
        .into_iter()
        .filter_map(|coord| coord.map(|coord| self[coord]))
        .collect()
    }

    pub fn iter(&self) -> slice::Iter<Tile> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<Tile> {
        self.into_iter()
    }

    pub fn rows(&self) -> impl Iterator<Item = &[Tile]> {
        self.tiles
            .iter()
            .map(<[Tile; BOARD_FILES as usize]>::as_slice)
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            tiles: array::from_fn(|rank| {
                if rank == 0 {
                    array::from_fn(|_| Tile {
                        piece_option: None,
                        tile_type: TileType::Baseline(Player::P1),
                    })
                } else if rank == (BOARD_RANKS - 1) as usize {
                    array::from_fn(|_| Tile {
                        piece_option: None,
                        tile_type: TileType::Baseline(Player::P2),
                    })
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

impl<'a> IntoIterator for &'a Board {
    type Item = &'a Tile;
    type IntoIter = slice::Iter<'a, Tile>;

    fn into_iter(self) -> Self::IntoIter {
        self.tiles.as_flattened().iter()
    }
}

impl<'a> IntoIterator for &'a mut Board {
    type Item = &'a mut Tile;
    type IntoIter = slice::IterMut<'a, Tile>;

    fn into_iter(self) -> Self::IntoIter {
        self.tiles.as_flattened_mut().iter_mut()
    }
}
