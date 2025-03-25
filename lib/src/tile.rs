use crate::pieces::Piece;

#[derive(Copy, Clone, Debug, Default)]
pub enum TileType {
    #[default]
    Regular,
    Town,
    Supply,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Tile {
    piece: Option<Piece>,
    tile_type: TileType,
}

impl Tile {
    #[must_use]
    pub const fn new(piece: Option<Piece>, tile_type: TileType) -> Self {
        Self { piece, tile_type }
    }

    #[must_use]
    pub const fn has_supplies(&self) -> bool {
        match self.tile_type {
            TileType::Town | TileType::Supply => true,
            TileType::Regular => false,
        }
    }

    #[must_use]
    pub const fn gives_income(&self) -> bool {
        matches!(self.tile_type, TileType::Town)
    }

    #[must_use]
    pub const fn piece(&self) -> Option<&Piece> {
        self.piece.as_ref()
    }
}
