pub mod convoy;
pub mod infantry;
pub mod recon;

use std::{ fmt::Debug, ops::Neg };

use convoy::Convoy;
use infantry::Infantry;
use recon::Recon;

pub trait IsPieceVariant: Copy + Debug {
    fn cost(&self) -> u8;
    fn power(&self) -> u8;
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
pub enum PieceVariant {
    Convoy(Convoy),
    Infantry(Infantry),
    Recon(Recon),
}

impl PieceVariant {
    #[must_use]
    pub fn new(r#type: PieceType) -> Self {
        match r#type {
            PieceType::Convoy => Self::Convoy(Convoy::default()),
            PieceType::Infantry => Self::Infantry(Infantry::default()),
            PieceType::Recon => Self::Recon(Recon::default()),
        }
    }
}

impl IsPieceVariant for PieceVariant {
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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    pub colour: PieceColour,
    r#type: PieceType,
    variant: PieceVariant,
}

impl Piece {
    #[must_use]
    pub fn new(colour: PieceColour, r#type: PieceType) -> Self {
        Self { colour, r#type, variant: PieceVariant::new(r#type) }
    }

    #[must_use]
    pub const fn r#type(&self) -> PieceType {
        self.r#type
    }
}

impl IsPieceVariant for Piece {
    fn cost(&self) -> u8 {
        <PieceVariant as IsPieceVariant>::cost(&self.variant)
    }

    fn power(&self) -> u8 {
        <PieceVariant as IsPieceVariant>::power(&self.variant)
    }
}
