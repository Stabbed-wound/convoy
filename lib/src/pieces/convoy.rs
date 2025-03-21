use crate::{ coord::Coord, Game };

use super::{ PieceColour, IsPieceVariant };

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Convoy {}

impl IsPieceVariant for Convoy {
    fn cost(&self) -> u8 {
        2
    }

    fn power(&self) -> u8 {
        0
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
