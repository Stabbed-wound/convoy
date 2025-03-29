use crate::{board::Board, coord::Coord};
use std::fmt::Debug;
use std::ops::{Deref, RangeInclusive};

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
    Artillery,
    Convoy,
    Infantry,
    Recon,
}

impl PieceType {
    #[must_use]
    pub const fn cost(&self) -> u8 {
        match self {
            Self::Convoy => 3,
            Self::Infantry => 2,
            Self::Recon | Self::Artillery => 4,
        }
    }

    #[must_use]
    pub const fn power(&self) -> u8 {
        match self {
            Self::Convoy => 0,
            Self::Infantry | Self::Artillery => 2,
            Self::Recon => 1,
        }
    }

    #[must_use]
    pub const fn range(&self) -> RangeInclusive<u8> {
        match self {
            Self::Artillery => 2..=3,
            Self::Convoy => 0..=0,
            Self::Infantry | Self::Recon => 1..=1,
        }
    }

    #[must_use]
    pub const fn speed(&self) -> u8 {
        match self {
            Self::Convoy => 3,
            Self::Infantry | Self::Artillery => 2,
            Self::Recon => 4,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    exhausted: bool,
    piece_colour: PieceColour,
    piece_type: PieceType,
}

impl Deref for Piece {
    type Target = PieceType;

    fn deref(&self) -> &Self::Target {
        &self.piece_type
    }
}

impl Piece {
    #[must_use]
    pub const fn new(piece_colour: PieceColour, piece_type: PieceType) -> Self {
        Self {
            exhausted: true,
            piece_colour,
            piece_type,
        }
    }

    #[must_use]
    pub const fn colour(&self) -> PieceColour {
        self.piece_colour
    }

    #[must_use]
    pub const fn is_exhausted(&self) -> bool {
        self.exhausted
    }

    pub const fn toggle_exhaust(&mut self) {
        self.exhausted = !self.exhausted;
    }

    #[must_use]
    pub const fn get_moves(&self, Coord { rank: _, file: _ }: Coord, _board: Board) -> Vec<Coord> {
        todo!()
    }
}
