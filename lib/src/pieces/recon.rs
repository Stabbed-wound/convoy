use crate::{ coord::Coord, Game };

use super::{ PieceColour, IsPieceVariant };

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Recon {}

impl IsPieceVariant for Recon {
    fn cost(&self) -> u8 {
        3
    }

    fn power(&self) -> u8 {
        1
    }

    fn get_moves(
        &self,
        _colour: PieceColour,
        Coord { rank: _, file: _ }: Coord,
        _context: &Game
    ) -> Vec<Coord> {
        unimplemented!()
    }
}
