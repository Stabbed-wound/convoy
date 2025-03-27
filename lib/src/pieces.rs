use std::fmt::Debug;

use crate::{board::Board, coord::Coord};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceColour {
    Black,
    White,
}

impl PieceColour {
    #[must_use]
    pub const fn opposite(self) -> Self {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceType {
    Convoy,
    Infantry,
    Recon,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    pub piece_colour: PieceColour,
    pub piece_type: PieceType,
}

impl Piece {
    #[must_use]
    pub const fn cost(&self) -> u8 {
        match self.piece_type {
            PieceType::Convoy => 2,
            PieceType::Infantry | PieceType::Recon => 3,
        }
    }

    #[must_use]
    pub const fn power(&self) -> u8 {
        match self.piece_type {
            PieceType::Convoy => 0,
            PieceType::Infantry => 2,
            PieceType::Recon => 1,
        }
    }

    #[must_use]
    pub const fn speed(&self) -> u8 {
        match self.piece_type {
            PieceType::Convoy => 3,
            PieceType::Infantry => 2,
            PieceType::Recon => 4,
        }
    }

    #[must_use]
    pub const fn get_moves(&self, Coord { rank: _, file: _ }: Coord, _board: Board) -> Vec<Coord> {
        todo!()
    }
}
