use crate::{board::Board, coord::Coord, Player};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, RangeInclusive};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceType {
    Artillery,
    Convoy,
    Infantry,
    Recon,
}

impl Display for PieceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Artillery => "A",
            Self::Convoy => "C",
            Self::Infantry => "I",
            Self::Recon => "R",
        })
    }
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
    pub owner: Player,
    pub exhausted: bool,
    pub piece_type: PieceType,
}

impl Deref for Piece {
    type Target = PieceType;

    fn deref(&self) -> &Self::Target {
        &self.piece_type
    }
}

impl Piece {
    #[must_use]
    pub const fn new(owner: Player, piece_type: PieceType) -> Self {
        Self {
            owner,
            exhausted: true,
            piece_type,
        }
    }

    #[must_use]
    pub const fn get_moves(&self, Coord { rank: _, file: _ }: Coord, _board: &Board) -> Vec<Coord> {
        todo!()
    }
}
