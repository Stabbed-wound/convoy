use crate::pieces::Piece;
use std::ops::Deref;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum TileType {
    #[default]
    Regular,
    Town,
    Baseline,
}

impl TileType {
    #[must_use]
    pub const fn has_supplies(self) -> bool {
        match self {
            Self::Town | Self::Baseline => true,
            Self::Regular => false,
        }
    }

    #[must_use]
    pub const fn gives_income(self) -> bool {
        matches!(self, Self::Town)
    }
    
    #[must_use]
    pub const fn produces_troops(self) -> bool {
        matches!(self, Self::Baseline)
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Tile {
    pub piece_option: Option<Piece>,
    pub tile_type: TileType,
}

impl Deref for Tile {
    type Target = TileType;

    fn deref(&self) -> &Self::Target {
        &self.tile_type
    }
}
