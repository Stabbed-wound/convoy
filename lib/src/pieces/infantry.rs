use crate::{ coord::Coord, Game };

use super::{PieceColour, PieceVariant};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Infantry {}

impl PieceVariant for Infantry {
    fn cost(&self) -> u8 {
        3
    }

    fn power(&self) -> u8 {
        2
    }

    fn get_moves(
        &self,
        _colour: PieceColour,
        Coord { rank: _, file: _ }: Coord,
        _context: &Game
    ) -> Vec<Coord> {
		todo!()
    }
}
