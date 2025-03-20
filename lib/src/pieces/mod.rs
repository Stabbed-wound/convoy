pub mod convoy;
pub mod infantry;
pub mod recon;

use std::ops::Neg;

use convoy::Convoy;
use infantry::Infantry;
use recon::Recon;

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

pub trait IsPiece : Copy {
	fn get_cost(&self) -> u8;
	fn get_power(&self) -> u8;
	fn get_colour(&self) -> PieceColour;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Piece {
    Convoy(Convoy),
    Infantry(Infantry),
    Recon(Recon),
}

impl IsPiece for Piece {
	fn get_cost(&self) -> u8 {
		match self {
			Self::Convoy(convoy) => convoy.get_cost(),
			Self::Infantry(infantry) => infantry.get_cost(),
			Self::Recon(recon) => recon.get_cost(),
		}
	}

	fn get_power(&self) -> u8 {
		match self {
			Self::Convoy(convoy) => convoy.get_power(),
			Self::Infantry(infantry) => infantry.get_power(),
			Self::Recon(recon) => recon.get_power(),
		}
	}

	fn get_colour(&self) -> PieceColour {
		match self {
			Self::Convoy(convoy) => convoy.get_colour(),
			Self::Infantry(infantry) => infantry.get_colour(),
			Self::Recon(recon) => recon.get_colour(),
		}
	}
}
