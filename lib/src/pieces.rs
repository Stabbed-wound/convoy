mod convoy;
mod infantry;
mod recon;

use std::{ fmt::Debug, ops::Neg };

use convoy::Convoy;
use infantry::Infantry;
use recon::Recon;
use crate::{ coord::Coord, Game };

pub trait PieceVariant: Copy + Debug + Eq {
    fn cost(&self) -> u8;
    fn power(&self) -> u8;
    fn get_moves(&self, colour: PieceColour, pos: Coord, context: &Game) -> Vec<Coord>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceColour {
    Black,
    White,
}

impl Neg for PieceColour {
    type Output = Self;

    fn neg(self) -> Self::Output {
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
pub enum Variants {
    Convoy(Convoy),
    Infantry(Infantry),
    Recon(Recon),
}

impl Variants {
    #[must_use]
    pub fn new(r#type: PieceType) -> Self {
        match r#type {
            PieceType::Convoy => Self::Convoy(Convoy::default()),
            PieceType::Infantry => Self::Infantry(Infantry::default()),
            PieceType::Recon => Self::Recon(Recon::default()),
        }
    }
}

impl PieceVariant for Variants {
    fn cost(&self) -> u8 {
        match self {
            Self::Convoy(convoy) => convoy.cost(),
            Self::Infantry(infantry) => infantry.cost(),
            Self::Recon(recon) => recon.cost(),
        }
    }

    fn power(&self) -> u8 {
        match self {
            Self::Convoy(convoy) => convoy.power(),
            Self::Infantry(infantry) => infantry.power(),
            Self::Recon(recon) => recon.power(),
        }
    }

    fn get_moves(&self, colour: PieceColour, pos: Coord, context: &Game) -> Vec<Coord> {
        match self {
            Self::Convoy(convoy) => convoy.get_moves(colour, pos, context),
            Self::Infantry(infantry) => infantry.get_moves(colour, pos, context),
            Self::Recon(recon) => recon.get_moves(colour, pos, context),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    colour: PieceColour,
    r#type: PieceType,
    variant: Variants,
}

impl Piece {
    #[must_use]
    pub fn new(colour: PieceColour, r#type: PieceType) -> Self {
        Self { colour, r#type, variant: Variants::new(r#type) }
    }

	#[must_use]
	pub const fn colour(&self) -> PieceColour {
		self.colour
	}

    #[must_use]
    pub const fn r#type(&self) -> PieceType {
        self.r#type
    }
}

impl PieceVariant for Piece {
    fn cost(&self) -> u8 {
        <Variants as PieceVariant>::cost(&self.variant)
    }

    fn power(&self) -> u8 {
        <Variants as PieceVariant>::power(&self.variant)
    }

    fn get_moves(&self, colour: PieceColour, pos: Coord, context: &Game) -> Vec<Coord> {
        <Variants as PieceVariant>::get_moves(&self.variant, colour, pos, context)
    }
}
